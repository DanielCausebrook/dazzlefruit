// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![feature(iter_next_chunk)]
#![feature(associated_type_defaults)]
#![feature(inline_const)]

extern crate core;

use std::str::FromStr;

use nalgebra_glm::smoothstep;
use palette::rgb::Rgb;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;
use pattern_builder::component::layer::LayerInfo;

use crate::neopixel_controller::NeopixelController;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::frame::{Frame, ScalarPixel};
use crate::pattern_builder::component::layer::LayerCore;
use crate::pattern_builder::component::layer::scalar_texture::ScalarTextureLayer;
use crate::pattern_builder::component::layer::standard_types::SCALAR_FRAME;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::library::color_range::ColorRange;
use crate::pattern_builder::library::core::solid_color::SolidColor;
use crate::pattern_builder::library::filters::alpha_mask::AlphaMask;
use crate::pattern_builder::library::scalar_filters::persistence::Persistence;
use crate::pattern_builder::library::scalar_textures::sparkles::Sparkles;
use crate::pattern_builder::library::transformers::scalar_to_dual_texture::ScalarToDualTexture;
use crate::pattern_builder::library::waves::Wave;
use crate::pattern_builder::math_functions::triangle_sin;
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
pub struct Pulse {
    period: Prop<f64>,
    width: Prop<f64>,
    smoothness: Prop<f64>,
}

impl Pulse {
    pub fn new(period: f64, width: f64) -> Self {
        Self {
            period: NumPropCore::new_slider(period, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Period")),
            width: NumPropCore::new_slider(width, 1.0..20.0, 0.2).into_prop(PropertyInfo::new("Width")),
            smoothness: NumPropCore::new_slider(3.0, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Smoothness")),
        }
    }

    pub fn into_layer(self, layer_info: LayerInfo) -> ScalarTextureLayer {
        ScalarTextureLayer::new(self, layer_info)
    }
}

impl Component for Pulse {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties![
            self.period,
            self.width,
            self.smoothness,
        ]
    }

    fn detach(&mut self) {
        fork_properties!(
            self.period,
            self.width,
            self.smoothness,
        );
    }
}

impl LayerCore for Pulse {
    type Input = ();
    type Output = Frame<ScalarPixel>;
    fn next(&mut self, _: (), t: f64, ctx: &PatternContext) -> Frame<ScalarPixel> {
        let pulse_pos = 0.5 * (triangle_sin(*self.smoothness.read(), *self.period.read(), t) + 1.0) * (ctx.num_pixels() as f64 - *self.width.read());
        let step1 = [pulse_pos - 0.5, pulse_pos + 0.5];
        let step2 = [pulse_pos + *self.width.read() - 0.5, pulse_pos + *self.width.read() + 0.5];
        (0..ctx.num_pixels()).into_iter()
            .map(|x| x as f64)
            .map(|x| {
                smoothstep(step1[0], step1[1], x) - smoothstep(step2[0], step2[1], x)
            })
            .collect()
    }
}

fn get_test_pattern() -> Pattern {
    let pattern = Pattern::new("Test Pattern", 350, 60.0);

    pattern.stack().write().push(Wave::new().into_layer(LayerInfo::new("Waves")));

    let transformer = ScalarToDualTexture::new();
    transformer.texture_a().write().push(ColorRange::new(Rgb::from_str("#FF00E1").unwrap().into()).into_layer(LayerInfo::new("Color")));
    transformer.texture_b().write().push(ColorRange::new(Rgb::from_str("#0433FF").unwrap().into()).into_layer(LayerInfo::new("Color")));
    pattern.stack().write().push(transformer.into_layer(LayerInfo::new("Dual Texture")));

    let mask = AlphaMask::new();
    mask.stack().write().push(Pulse::new(4.0, 10.0).into_layer(LayerInfo::new("Pulse")));
    mask.stack().write().push(Persistence::new(5.0).into_layer(&SCALAR_FRAME, LayerInfo::new("Persistence")));
    mask.stack().write().push(Sparkles::new(7.0, 5.0).into_layer(LayerInfo::new("Sparkles")));
    pattern.stack().write().push(mask.into_layer(LayerInfo::new("Alpha Mask")));

    pattern
}

fn get_test_pattern_2() -> Pattern {
    let pattern = Pattern::new("Simple Test Pattern", 350, 60.0);

    // pattern.stack().write().push(Wave::new().into_layer(LayerInfo::new("Waves")));
    //
    // let transformer = ScalarToDualTexture::new();
    // transformer.texture_a().write().push(ColorRange::new(Rgb::from_str("#FF00E1").unwrap().into()).into_layer(LayerInfo::new("Color")));
    // transformer.texture_b().write().push(ColorRange::new(Rgb::from_str("#0433FF").unwrap().into()).into_layer(LayerInfo::new("Color")));
    // pattern.stack().write().push(transformer.into_layer(LayerInfo::new("Dual Texture")));
    pattern.stack().write().push(SolidColor::new(Rgb::from_str("#FF00E1").unwrap().into()).into_layer(LayerInfo::new("Color")));

    let mask = AlphaMask::new();
    mask.stack().write().push(Pulse::new(4.0, 10.0).into_layer(LayerInfo::new("Pulse")));
    mask.stack().write().push(Persistence::new(5.0).into_layer(&SCALAR_FRAME, LayerInfo::new("Persistence")));
    mask.stack().write().push(Sparkles::new(7.0, 5.0).into_layer(LayerInfo::new("Sparkles")));
    pattern.stack().write().push(mask.into_layer(LayerInfo::new("Alpha Mask")));

    pattern
}

fn main() {
    // 10.0.1.43

    tauri::Builder::default()
        .setup(move |app| {
            let mut state = AppState {
                connection: None,
                app_handle: app.handle().clone(),
                neopixel_controller: None,
                pattern_builder: PatternBuilder::new(app.handle().clone()),
            };
            state.pattern_builder.load_pattern(get_test_pattern());
            state.pattern_builder.load_pattern(get_test_pattern_2());
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
