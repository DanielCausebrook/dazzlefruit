// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![feature(iter_next_chunk)]
#![feature(associated_type_defaults)]
#![feature(inline_const)]

extern crate core;

use std::str::FromStr;
use palette::{Alpha, Hsl, Hsla, IntoColor, Srgba, WithHue};
use palette::encoding::Srgb;

use palette::rgb::Rgb;
use tauri::{AppHandle, Manager};
use tokio::sync::{RwLock, watch};

use crate::neopixel_controller::NeopixelController;
use crate::pattern_builder::component::frame::{Frame, ScalarPixel};
use crate::pattern_builder::component::layer::standard_types::{SCALAR_FRAME, VOID};
use crate::pattern_builder::library::core::group::Group;
use pattern_builder::library::color::textures::solid_color::SolidColor;
use pattern_builder::library::color::filters::alpha_mask::AlphaMask;
use crate::pattern_builder::component::layer::generic::GenericLayer;
use crate::pattern_builder::component::layer::{DisplayPane, Layer, LayerCore, LayerIcon, LayerTypeInfo};
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::library::color::filters::map_hsl_component::MapHslComponent;
use crate::pattern_builder::library::color::textures::color_range::ColorRange;
use crate::pattern_builder::library::generic::filters::persistence::Persistence;
use crate::pattern_builder::library::generic::filters::stutter::Stutter;
use crate::pattern_builder::library::scalar::textures::pulse::Pulse;
use crate::pattern_builder::library::scalar::textures::simplex_noise::SimplexNoise;
use crate::pattern_builder::library::scalar::textures::sparkles::Sparkles;
use crate::pattern_builder::library::scalar::textures::dual_waves::DualWaves;
use crate::pattern_builder::library::transformers::scalar_to_dual_texture::ScalarToDualTexture;
use crate::pattern_builder::library::transformers::scalar_to_texture::ScalarToTexture;
use crate::pattern_builder::pattern::Pattern;
use crate::pattern_builder::pattern_context::PatternContext;
use crate::pattern_builder::PatternBuilder;
use crate::pico_connection::PicoConnectionHandle;
use crate::tauri_events::DebugMessagePayload;

mod pico_connection;
mod neopixel_controller;
mod tauri_events;
mod pattern_builder;

pub struct AppState {
    connection: Option<PicoConnectionHandle>,
    neopixel_controller: Option<NeopixelController>,
    pattern_builder: PatternBuilder,
    app_handle: AppHandle,
}

pub struct LockedAppState(pub RwLock<AppState>);

impl AppState {
    fn debug_println(&self, message: &str) {
        self.app_handle.emit_all("debug-println", DebugMessagePayload{ message: message.parse().unwrap() }).unwrap();
    }
}

#[derive(Clone)]
pub struct AddValue {
    speed: Prop<f64>,
}

impl AddValue {
    pub fn new(multiplier: f64) -> Self {
        AddValue {
            speed: NumPropCore::new(multiplier).into_prop(PropertyInfo::new("Rate of Change")),
        }
    }

    pub fn into_layer(self) -> GenericLayer<Self> {
        GenericLayer::new(self, LayerTypeInfo::new("Add value"), &SCALAR_FRAME, &SCALAR_FRAME)
    }
}

impl LayerCore for AddValue {
    type Input = Frame<ScalarPixel>;
    type Output = Frame<ScalarPixel>;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        input.into_iter()
            .map(|value| value + t * *self.speed.read())
            .collect()
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.speed)
    }

    fn detach(&mut self) {
        fork_properties!(self.speed);
    }
}

#[derive(Clone)]
struct SinglePixel {
    position: Prop<u64>,
}

impl SinglePixel {
    pub fn new(num_pixels: u64) -> Self {
        SinglePixel{
            position: NumPropCore::new_slider(0, 0..num_pixels, 1).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }

    pub fn into_layer(self) -> GenericLayer<Self> {
        GenericLayer::new(self, LayerTypeInfo::new("Single Pixel").with_icon(LayerIcon::Texture), &VOID, &SCALAR_FRAME)
    }
}

impl LayerCore for SinglePixel {
    type Input = ();
    type Output = Frame<ScalarPixel>;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        let pos = *self.position.read();
        (0..ctx.num_pixels()).map(|x| if x as u64 == pos {1.0} else {0.0})
            .collect()
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.position,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.position
        )
    }
}

fn get_solid_color_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Solid Color", pattern_context, 60.0);

    pattern.stack().write().push(SolidColor::new(Rgb::from_str("#0000FF").unwrap().into()).into_layer());

    pattern
}

fn get_single_pixel_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Single Pixel", pattern_context, 60.0);

    pattern.stack().write().push(SinglePixel::new(150).into_layer());


    let transformer = ScalarToTexture::new();
    transformer.texture().write().push(SolidColor::new(Rgb::from_str("#0000FF").unwrap().into()).into_layer());
    pattern.stack().write().push(transformer.into_layer());

    pattern
}

fn get_test_pattern(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Test Pattern", pattern_context, 60.0);

    let waves = DualWaves::new();
    *waves.wave1_scale().write() = 36.0;
    *waves.wave2_scale().write() = 45.0;
    pattern.stack().write().push(waves.into_layer());

    let transformer = ScalarToDualTexture::new();
    transformer.texture_a().write().push(ColorRange::new(Rgb::from_str("#FF00E1").unwrap().into()).into_layer());
    transformer.texture_b().write().push(ColorRange::new(Rgb::from_str("#0433FF").unwrap().into()).into_layer());
    pattern.stack().write().push(transformer.into_layer());

    let mask = AlphaMask::new();
    mask.stack().write().push(Pulse::new(4.0, 10.0, 3.0).into_layer());
    mask.stack().write().push(Persistence::new(5.0).into_layer(&SCALAR_FRAME));
    mask.stack().write().push(Sparkles::new(7.0, 5.0).into_layer());
    pattern.stack().write().push(mask.into_layer());

    pattern
}

fn get_test_pattern_2(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Stutter Pulse", pattern_context, 60.0);

    let green_group = Group::new();
    let noise = SimplexNoise::new(0.5);
    noise.scale().write().x = 1.0/40.0;
    noise.scale().write().y = 1.0/40.0;
    noise.scale().write().z = 1.0/40.0;
    green_group.stack().write().push(noise.into_layer());
    let into_texture = ScalarToTexture::new();
    into_texture.texture().write().push(SolidColor::new(Rgb::from_str("#FEFF66").unwrap().into()).into_layer());
    *into_texture.lower_bound().write() = 0.3;
    *into_texture.upper_bound().write() = 0.35;
    green_group.stack().write().push(into_texture.into_layer());
    pattern.stack().write().push(green_group.into_layer().with_name("Pulses"));

    let sparkles_group = Group::new();

    let range_a = ColorRange::new(Rgb::from_str("#B55748").unwrap().into());
    *range_a.variance().write() = 10.0;
    sparkles_group.stack().write().push(range_a.into_layer());
    let mask = AlphaMask::new();
    mask.stack().write().push(Sparkles::new(7.0, 3.0).into_layer());
    sparkles_group.stack().write().push(mask.into_layer());
    pattern.stack().write().push(sparkles_group.into_layer().with_name("BG Sparkle"));

    let hue_shift = MapHslComponent::new_hue();
    hue_shift.map().write().push(AddValue::new(60.0).into_layer());
    pattern.stack().write().push(hue_shift.into_layer().with_name("Hue Shift"));

    let pulse_group = Group::new();
    pulse_group.stack().write().push(Pulse::new(3.5, 3.0, 8.0).into_layer());
    pulse_group.stack().write().push(Stutter::new_partially_empty(0.026, 0.0, |ctx| Frame::empty(ctx.num_pixels())).into_layer(&SCALAR_FRAME));
    pulse_group.stack().write().push(Persistence::new(2.7).into_layer(&SCALAR_FRAME));
    let texture = ScalarToTexture::new();
    texture.texture().write().push(SolidColor::new(Rgb::from_str("#FFB846").unwrap().into()).into_layer().with_name("Gold"));
    pulse_group.stack().write().push(texture.into_layer());

    pattern.stack().write().push(pulse_group.into_layer().with_name("Stuttering Pulse"));

    pattern
}

fn get_test_pattern_3(pattern_context: watch::Receiver<PatternContext<'static>>) -> Pattern {
    let pattern = Pattern::new("Blocks", pattern_context, 60.0);

    let color: Alpha<Hsl<Srgb, f64>, f64> = Hsla::new(0.0, 1.0, 0.5, 1.0);
    let num_colors = 10;

    for x in 0..num_colors {
        let layer = Group::new();
        let noise = SimplexNoise::new(0.8 - (0.3 * (x + 1) as f64 / num_colors as f64));
        noise.scale().write().x = 1.0/30.0;
        noise.scale().write().y = 1.0/30.0;
        noise.scale().write().z = 1.0/30.0;
        layer.stack().write().push(noise.into_layer());
        let into_texture = ScalarToTexture::new();
        let c2 = color.clone().with_hue(color.clone().hue.into_degrees() + (x as f64 * 360.0 / num_colors as f64));
        into_texture.texture().write().push(SolidColor::new(IntoColor::<Srgba<f64>>::into_color(c2).into_linear()).into_layer());
        *into_texture.lower_bound().write() = 0.3;
        *into_texture.upper_bound().write() = 0.35;
        layer.stack().write().push(into_texture.into_layer());
        pattern.stack().write().push(layer.into_layer().with_name(format!("Layer {}", x).as_str()));
    }

    let hue_shift = MapHslComponent::new_hue();
    hue_shift.map().write().push(AddValue::new(20.0).into_layer());
    pattern.stack().write().push(hue_shift.into_layer().with_name("Hue Rotate"));

    pattern
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let mut state = AppState {
                connection: None,
                app_handle: app.handle().clone(),
                neopixel_controller: None,
                pattern_builder: PatternBuilder::new(app.handle().clone(), 150),
            };
            state.pattern_builder.load_pattern(get_solid_color_pattern(state.pattern_builder.pattern_context()));
            state.pattern_builder.load_pattern(get_single_pixel_pattern(state.pattern_builder.pattern_context()));
            state.pattern_builder.load_pattern(get_test_pattern(state.pattern_builder.pattern_context()));
            state.pattern_builder.load_pattern(get_test_pattern_2(state.pattern_builder.pattern_context()));
            state.pattern_builder.load_pattern(get_test_pattern_3(state.pattern_builder.pattern_context()));
            app.manage(LockedAppState(RwLock::new(state)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            pico_connection::connect,
            pico_connection::disconnect,
            neopixel_controller::init_neopixel,
            neopixel_controller::set_neopixel_pattern,
            pattern_builder::view_open_patterns,
            pattern_builder::view_pattern,
            pattern_builder::update_property,
            pattern_builder::position_map,
            pattern_builder::load_position_map,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
