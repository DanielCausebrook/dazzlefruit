// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![feature(iter_next_chunk)]
#![feature(trait_upcasting)]
#![feature(associated_type_defaults)]

extern crate core;

use std::str::FromStr;

use palette::{Lighten, WithAlpha};
use palette::rgb::Rgb;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;
use tokio::sync::RwLockWriteGuard;

use pattern_builder::component::texture::Texture;

use crate::neopixel_controller::NeopixelController;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::BlendMode;
use crate::pattern_builder::component::property::{PropertyInfo};
use crate::pattern_builder::component::texture_generator::{CyclingTextureGenerator, TextureGeneratorProperty};
use crate::pattern_builder::library::core::{GroupLayer, SolidColor};
use crate::pattern_builder::library::pulsing_blocks::PulsingBlocksConfig;
use crate::pattern_builder::library::sparkles::SparklesConfig;
use crate::pattern_builder::library::two_tone::TwoToneConfig;
use crate::pattern_builder::library::waves::Wave;
use crate::pattern_builder::PatternBuilder;
use crate::pico_connection::PicoConnectionHandle;
use crate::tauri_events::DebugMessagePayload;

mod pico_connection;
mod neopixel_controller;
mod tauri_events;
mod pattern_builder;
mod watch_guard;

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

#[tauri::command]
async fn set_color(color_str: String, tauri_state: tauri::State<'_, LockedAppState>) -> Result<(), String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    let color = Rgb::from_str(color_str.as_str())
        .map_err(|e| e.to_string())?;
    state.pattern_builder.set_layer(SolidColor::new(color.into()));
    Ok(())
}

fn get_birds_pattern() -> Box<dyn Texture> {
    // let mut mask_vec = vec![0.5; num_pixels];
    // for x in (0..12).chain(13..49).chain(53..89).step_by(2) {
    //     mask_vec[x] = 1.0;
    // }
    // let bird_mask = MaskLayer::new(mask_vec);

    let colors = TwoToneConfig::new(
        (SolidColor::new(palette::named::BLUE.into()), SolidColor::new(palette::named::PURPLE.into())),
        0.8,
        1.0
    ).init_gradient_width(0.3);

    let sparkle_color_group = GroupLayer::new();
    let sparkle_mask = TwoToneConfig::new(
        (SolidColor::new(palette::named::BLACK.into()), SolidColor::new(palette::named::BLACK.transparent().into())),
        0.3,
        0.1,
    ).init_noise_velocity(5.0).init_gradient_offset(-0.2).init_blend_mode(BlendMode::AlphaMask);
    let sparkle_colors = TwoToneConfig::new(
        (
            SolidColor::new(palette::named::BLUE.into_linear().lighten(0.3).into()),
            SolidColor::new(palette::named::PURPLE.into_linear().lighten(0.3).into())
        ),
        2.0,
        10.0
    ).init_gradient_width(0.3);
    sparkle_color_group.add_pixel_layer(sparkle_colors.into_texture());
    sparkle_color_group.add_pixel_layer(sparkle_mask.into_texture());
    let sparkles = SparklesConfig::new(sparkle_color_group, 1.0, 1.0);

    let group = GroupLayer::new();
    group.add_pixel_layer(colors.into_texture());
    group.add_pixel_layer(sparkles.into_texture());
    // group.add_filter_layer(PersistenceEffectConfig::new(2.0).create());
    // group.add_filter_layer(Box::new(bird_mask));

    Box::new(group)
}

fn get_test_pattern() -> Box<dyn Texture> {
    let colors = vec![
        palette::named::PURPLE,
        palette::named::BLUE,
        palette::named::CYAN,
    ];
    let transparent = palette::named::BLACK.transparent().into();
    let group = GroupLayer::new();
    for color in colors {
        let tt = TwoToneConfig::new(
            (SolidColor::new(transparent), SolidColor::new(color.into())),
            1.0,
            0.1
        )
            .init_gradient_offset(0.15)
            .init_gradient_width(0.15);
        group.add_pixel_layer(tt.into_texture());
    }
    Box::new(group)
}

fn get_test_pattern_2() -> Box<dyn Texture> {
    let producer = CyclingTextureGenerator::new(vec![
        Box::new(SolidColor::new(palette::named::BLUE.into())),
        Box::new(SolidColor::new(palette::named::PURPLE.into())),
        Box::new(SolidColor::new(palette::named::RED.into())),
    ]);
    let pulsing_blocks = PulsingBlocksConfig::new(TextureGeneratorProperty::new(Box::new(producer), PropertyInfo::unnamed()));
    Box::new(pulsing_blocks.into_texture())
}
fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            let mut state = AppState {
                connection: None,
                app_handle: app.handle().clone(),
                neopixel_controller: None,
                pattern_builder: PatternBuilder::new(app.handle().clone(), 100),
            };
            // state.pattern_builder.set_layer(get_birds_pattern());
            // state.pattern_builder.set_layer(get_test_pattern_2());
            let group = GroupLayer::new();
            group.add_pixel_layer(Wave::new(
                SolidColor::new(Rgb::from_str("#FF00E1").unwrap().into()),
                SolidColor::new(Rgb::from_str("#0433FF").unwrap().into())
            ));
            let sparkles = SparklesConfig::new(SolidColor::new(palette::named::WHITE.into()), 1.0, 2.0);

            sparkles.blend_mode().replace(BlendMode::AlphaMask);
            group.add_pixel_layer(sparkles.into_texture());
            state.pattern_builder.set_layer(group);
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
            set_color
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
