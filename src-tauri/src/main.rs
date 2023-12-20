// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![feature(iter_next_chunk)]
#![feature(associated_type_defaults)]
#![feature(inline_const)]

extern crate core;

use std::str::FromStr;

use nalgebra_glm::{DVec3, smoothstep};
use palette::{Lighten, Mix, WithAlpha};
use palette::rgb::Rgb;
use tauri::{AppHandle, Manager};
use tokio::sync::{RwLock};
use pattern_builder::component::layer::LayerInfo;

use pattern_builder::component::layer::texture::Texture;

use crate::neopixel_controller::NeopixelController;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{BlendMode, DisplayPane, PixelFrame};
use crate::pattern_builder::component::layer::filter::Filter;
use crate::pattern_builder::component::layer::Layer;
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::layer::standard_types::{PIXEL_FRAME, VOID};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::layer::TexturePropCore;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::layer::texture::TextureLayer;
use crate::pattern_builder::component::layer::texture_generator::TextureGenerator;
use crate::pattern_builder::library::color_range::ColorRange;
use crate::pattern_builder::library::core::{Group, SolidColor};
use crate::pattern_builder::library::core::empty::Empty;
use crate::pattern_builder::library::filters::persistence_effect::PersistenceEffectConfig;
use crate::pattern_builder::library::pulsing_blocks::PulsingBlocksConfig;
use crate::pattern_builder::library::sparkles::SparklesConfig;
use crate::pattern_builder::library::texture_generators::cyclic::CyclicTextureGenerator;
use crate::pattern_builder::library::two_tone::TwoToneConfig;
use crate::pattern_builder::library::waves::Wave;
use crate::pattern_builder::math_functions::triangle_sin;
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_birds_pattern() -> TextureLayer {
    // let mut mask_vec = vec![0.5; num_pixels];
    // for x in (0..12).chain(13..49).chain(53..89).step_by(2) {
    //     mask_vec[x] = 1.0;
    // }
    // let bird_mask = MaskLayer::new(mask_vec);

    let colors = TwoToneConfig::new(
        (
            SolidColor::new(palette::named::BLUE.into()).into_layer(LayerInfo::new("Color 1")),
            SolidColor::new(palette::named::PURPLE.into()).into_layer(LayerInfo::new("Color 2"))
        ),
        0.8
    );
    colors.gradient_width().try_replace_value(0.3).unwrap();

    let sparkle_color_group = Group::new();
    let sparkle_mask = TwoToneConfig::new(
        (
            SolidColor::new(palette::named::BLACK.into()).into_layer(LayerInfo::new("Opaque")),
            SolidColor::new(palette::named::BLACK.transparent().into()).into_layer(LayerInfo::new("Transparent"))
        ),
        0.3
    );
    sparkle_mask.noise_scaling().try_replace_value(DVec3::repeat(0.1)).unwrap();
    sparkle_mask.noise_travel_speed().try_replace_value(DVec3::new(5.0, 0.0, 0.0)).unwrap();
    sparkle_mask.gradient_offset().try_replace_value(-0.2).unwrap();
    let sparkle_mask_component = sparkle_mask.into_texture().into_layer(LayerInfo::new("Sparkle Mask"));
    sparkle_mask_component.blend_mode().try_replace_value(BlendMode::AlphaMask).unwrap();
    let sparkle_colors = TwoToneConfig::new(
        (
            SolidColor::new(palette::named::BLUE.into_linear().lighten(0.3).into()).into_layer(LayerInfo::new("Color")),
            SolidColor::new(palette::named::PURPLE.into_linear().lighten(0.3).into()).into_layer(LayerInfo::new("Color"))
        ),
        2.0
    );
    sparkle_colors.noise_scaling().try_replace_value(DVec3::repeat(10.0)).unwrap();
    sparkle_colors.gradient_width().try_replace_value(0.3).unwrap();
    sparkle_color_group.add_texture(sparkle_colors.into_texture().into_layer(LayerInfo::new("Colours")));
    sparkle_color_group.add_texture(sparkle_mask_component);
    let sparkles = SparklesConfig::new(
        sparkle_color_group.into_layer(LayerInfo::new("Sparkle Color")),
        1.0,
        1.0
    );

    let group = Group::new();
    group.add_texture(colors.into_texture().into_layer(LayerInfo::new("Colors")));
    group.add_texture(sparkles.into_texture().into_layer(LayerInfo::new("Sparkles")));
    // group.add_filter_layer(PersistenceEffectConfig::new(2.0).create());
    // group.add_filter_layer(Box::new(bird_mask));

    group.into_layer(LayerInfo::new("Birds Pattern"))
}

fn get_test_pattern() -> TextureLayer {
    let colors = vec![
        palette::named::PURPLE,
        palette::named::BLUE,
        palette::named::CYAN,
    ];
    let transparent = palette::named::BLACK.transparent().into();
    let group = Group::new();
    for color in colors {
        let tt = TwoToneConfig::new(
            (
                SolidColor::new(transparent).into_layer(LayerInfo::new("Transparent")),
                SolidColor::new(color.into()).into_layer(LayerInfo::new("Color"))
            ),
            1.0
        );
        tt.noise_scaling().try_replace_value(DVec3::repeat(0.1)).unwrap();
        tt.gradient_offset().try_replace_value(0.15).unwrap();
        tt.gradient_width().try_replace_value(0.15).unwrap();
        group.add_texture(tt.into_texture().into_layer(LayerInfo::new("Two Tone")));
    }
    group.into_layer(LayerInfo::new("Test 1"))
}

fn get_test_pattern_2() -> TextureLayer {
    let producer = CyclicTextureGenerator::new(
        vec![
            SolidColor::new(palette::named::BLUE.into()),
            SolidColor::new(palette::named::PURPLE.into()),
            SolidColor::new(palette::named::RED.into()),
        ].into_iter()
            .map(|color| color.into_layer(LayerInfo::new("Color")))
            .collect()
    );
    let pulsing_blocks = PulsingBlocksConfig::new(producer.into_layer(LayerInfo::new("Textures")));
    pulsing_blocks.into_texture().into_layer(LayerInfo::new("Test 2"))
}

#[derive(Clone)]
pub struct Pulse {
    texture: Prop<TextureLayer>,
    period: Prop<f32>,
    width: Prop<f32>,
    smoothness: Prop<f32>,
}

impl Pulse {
    pub fn new(texture: TextureLayer) -> Self {
        Self {
            texture: TexturePropCore::new(texture).into_prop(PropertyInfo::new("Texture").set_display_pane(DisplayPane::Tree)),
            period: NumPropCore::new_slider(3.0, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Period")),
            width: NumPropCore::new_slider(5.0, 1.0..20.0, 0.2).into_prop(PropertyInfo::new("Width")),
            smoothness: NumPropCore::new_slider(3.0, 0.0..10.0, 0.01).into_prop(PropertyInfo::new("Smoothness")),
        }
    }
}

impl Component for Pulse {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties![
            self.texture,
            self.period,
            self.width,
            self.smoothness,
        ]
    }

    fn detach(&mut self) {
        fork_properties!(
            self.texture,
            self.period,
            self.width,
            self.smoothness,
        );
    }
}

impl Texture for Pulse {
    fn next_frame(&mut self, t: f64, ctx: &PatternContext) -> PixelFrame {
        let pulse_pos = 0.5 * (triangle_sin(*self.smoothness.read(), *self.period.read(), t as f32) + 1.0) * (ctx.num_pixels() as f32 - *self.width.read());
        let step1 = [pulse_pos - 0.5, pulse_pos + 0.5];
        let step2 = [pulse_pos + *self.width.read() - 0.5, pulse_pos + *self.width.read() + 0.5];
        self.texture.write().next(None, t, ctx).into_iter()
            .zip(0..)
            .map(|(pixel, x)| {
                let amount = smoothstep(step1[0], step1[1], x as f32) - smoothstep(step2[0], step2[1], x as f32);
                palette::named::BLACK.into_linear().transparent().mix(pixel, amount)
            })
            .collect()
        // self.texture.write().next_frame(t, num_pixels).into_iter()
        //     .zip(0..)
        //     .map(|(pixel, x)| {
        //         palette::named::BLACK.into_linear().transparent().mix(pixel, 0.5 + 0.5 * triangle_sin(self.smoothness.get(), self.width.get() * 2.0, x as f32))
        //     })
        //     .collect()
    }
}

fn get_test_pattern_3() -> LayerStack<(), PixelFrame> {
    let mut stack = LayerStack::new(&VOID, &PIXEL_FRAME);
    stack.push(Wave::new(
        ColorRange::new(Rgb::from_str("#FF00E1").unwrap().into()).into_layer(LayerInfo::new("Color")),
        ColorRange::new(Rgb::from_str("#0433FF").unwrap().into()).into_layer(LayerInfo::new("Color"))
    ).into_layer(LayerInfo::new("Wave")));
    let mut mask_stack = LayerStack::new(&VOID, &PIXEL_FRAME);
    let pulse = Pulse::new(Empty::new_texture_layer());
    pulse.texture.replace_core(RawPropCore::new(SolidColor::new(palette::named::WHITE.into()).into_layer(LayerInfo::new("Color"))));
    mask_stack.push(pulse.into_layer(LayerInfo::new("Pulse")));
    mask_stack.push(PersistenceEffectConfig::new(2.0).into_filter().into_layer(LayerInfo::new("Persistence Effect")));

    let sparkles = SparklesConfig::default();
    sparkles.texture().replace_core(RawPropCore::new(SolidColor::new(palette::named::WHITE.into()).into_layer(LayerInfo::new("Color"))));

    mask_stack.push(sparkles.into_texture().into_layer(LayerInfo::new("Sparkles")));
    let mask_group_layer = Group::from(mask_stack).into_layer(LayerInfo::new("Mask"));
    mask_group_layer.blend_mode().try_replace_value(BlendMode::AlphaMask).unwrap();
    stack.push(mask_group_layer);

    // group.add_texture(Repeater::new(25, Pulse::new(SolidColor::new(palette::named::WHITE.into()))));

    stack
}

fn main() {
    // 10.0.1.43

    tauri::Builder::default()
        .setup(move |app| {
            let mut state = AppState {
                connection: None,
                app_handle: app.handle().clone(),
                neopixel_controller: None,
                pattern_builder: PatternBuilder::new(app.handle().clone(), 350),
            };
            // state.pattern_builder.set_layer(get_birds_pattern());
            // state.pattern_builder.set_layer(get_test_pattern_2());
            state.pattern_builder.set_texture(get_test_pattern_3());
            app.manage(LockedAppState(RwLock::new(state)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            pico_connection::connect,
            pico_connection::disconnect,
            neopixel_controller::init_neopixel,
            pattern_builder::get_pattern_config,
            pattern_builder::update_property,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
