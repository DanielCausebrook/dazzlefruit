// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![feature(iter_next_chunk)]
#![feature(associated_type_defaults)]
#![feature(inline_const)]

extern crate core;

use std::str::FromStr;

use palette::rgb::Rgb;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;
use pattern_builder::component::layer::LayerInfo;

use crate::neopixel_controller::NeopixelController;
use crate::pattern_builder::component::frame::{Frame};
use crate::pattern_builder::component::layer::standard_types::SCALAR_FRAME;
use crate::pattern_builder::library::core::group::Group;
use pattern_builder::library::color::textures::solid_color::SolidColor;
use pattern_builder::library::color::filters::alpha_mask::AlphaMask;
use crate::pattern_builder::library::color::textures::color_range::ColorRange;
use crate::pattern_builder::library::generic::filters::persistence::Persistence;
use crate::pattern_builder::library::generic::filters::stutter::Stutter;
use crate::pattern_builder::library::scalar::textures::pulse::Pulse;
use crate::pattern_builder::library::scalar::textures::simplex_noise::SimplexNoise;
use crate::pattern_builder::library::scalar::textures::sparkles::Sparkles;
use crate::pattern_builder::library::scalar::textures::waves::Waves;
use crate::pattern_builder::library::transformers::scalar_to_dual_texture::ScalarToDualTexture;
use crate::pattern_builder::library::transformers::scalar_to_texture::ScalarToTexture;
use crate::pattern_builder::pattern::Pattern;
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


fn get_test_pattern() -> Pattern {
    let pattern = Pattern::new("Test Pattern", 350, 60.0);

    let waves = Waves::new();
    *waves.wave1_scale().write() = 36.0;
    *waves.wave2_scale().write() = 45.0;
    pattern.stack().write().push(waves.into_layer(LayerInfo::new("Waves")));

    let transformer = ScalarToDualTexture::new();
    transformer.texture_a().write().push(ColorRange::new(Rgb::from_str("#FF00E1").unwrap().into()).into_layer(LayerInfo::new("Color")));
    transformer.texture_b().write().push(ColorRange::new(Rgb::from_str("#0433FF").unwrap().into()).into_layer(LayerInfo::new("Color")));
    pattern.stack().write().push(transformer.into_layer(LayerInfo::new("Dual Texture")));

    let mask = AlphaMask::new();
    mask.stack().write().push(Pulse::new(4.0, 10.0, 3.0).into_layer(LayerInfo::new("Pulse")));
    mask.stack().write().push(Persistence::new(5.0).into_layer(&SCALAR_FRAME, LayerInfo::new("Persistence")));
    mask.stack().write().push(Sparkles::new(7.0, 5.0).into_layer(LayerInfo::new("Sparkles")));
    pattern.stack().write().push(mask.into_layer(LayerInfo::new("Alpha Mask")));

    pattern
}

fn get_test_pattern_2() -> Pattern {
    let pattern = Pattern::new("Stutter Pulse", 350, 60.0);

    let green_group = Group::new();
    let noise = SimplexNoise::new(0.5);
    noise.scale().write().x = 1.0/40.0;
    green_group.stack().write().push(noise.into_layer(LayerInfo::new("Noise")));
    let into_texture = ScalarToTexture::new();
    into_texture.texture().write().push(SolidColor::new(Rgb::from_str("#00DE2D").unwrap().into()).into_layer(LayerInfo::new("Color")));
    *into_texture.lower_bound().write() = 0.3;
    *into_texture.upper_bound().write() = 0.35;
    green_group.stack().write().push(into_texture.into_layer(LayerInfo::new("Into Texture")));
    pattern.stack().write().push(green_group.into_layer(LayerInfo::new("Green BG")));

    let sparkles_group = Group::new();

    let range_a = ColorRange::new(Rgb::from_str("#B51A00").unwrap().into());
    *range_a.variance().write() = 10.0;
    sparkles_group.stack().write().push(range_a.into_layer(LayerInfo::new("Color Range")));
    let mask = AlphaMask::new();
    mask.stack().write().push(Sparkles::new(7.0, 3.0).into_layer(LayerInfo::new("Sparkles")));
    sparkles_group.stack().write().push(mask.into_layer(LayerInfo::new("Alpha Mask")));
    pattern.stack().write().push(sparkles_group.into_layer(LayerInfo::new("BG")));

    let pulse_group = Group::new();
    pulse_group.stack().write().push(Pulse::new(3.5, 3.0, 8.0).into_layer(LayerInfo::new("Pulse")));
    pulse_group.stack().write().push(Stutter::new_partially_empty(0.026, 0.0, |ctx| Frame::empty(ctx.num_pixels())).into_layer(&SCALAR_FRAME, LayerInfo::new("Stutter")));
    pulse_group.stack().write().push(Persistence::new(2.7).into_layer(&SCALAR_FRAME, LayerInfo::new("Persistence")));
    let texture = ScalarToTexture::new();
    texture.texture().write().push(SolidColor::new(Rgb::from_str("#FFB846").unwrap().into()).into_layer(LayerInfo::new("Color")));
    pulse_group.stack().write().push(texture.into_layer(LayerInfo::new("To Texture")));

    pattern.stack().write().push(pulse_group.into_layer(LayerInfo::new("Stuttering Pulse")));

    pattern
}

fn main() {
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
