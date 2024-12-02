// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![feature(iter_next_chunk)]
#![feature(associated_type_defaults)]
#![feature(inline_const)]
#![feature(map_try_insert)]
#![feature(trait_upcasting)]

extern crate core;

use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::RwLock;

use crate::neopixel_controller::NeopixelController;
use crate::pattern_builder::PatternBuilder;
use crate::pico_connection::PicoConnectionHandle;
use crate::tauri_events::DebugMessagePayload;

mod pico_connection;
mod neopixel_controller;
mod tauri_events;
mod pattern_builder;
mod test_patterns;

pub struct AppState {
    connection: Option<PicoConnectionHandle>,
    neopixel_controller: Option<NeopixelController>,
    pattern_builder: PatternBuilder,
    app_handle: AppHandle,
}

pub struct LockedAppState(pub RwLock<AppState>);

impl AppState {
    fn debug_println(&self, message: &str) {
        self.app_handle.emit("debug-println", DebugMessagePayload{ message: message.parse().unwrap() }).unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let mut state = AppState {
                connection: None,
                app_handle: app.handle().clone(),
                neopixel_controller: None,
                pattern_builder: PatternBuilder::new(app.handle().clone(), 150),
            };
            for pattern in test_patterns::test_patterns(state.pattern_builder.pattern_context()) {
                state.pattern_builder.load_pattern(pattern);
            }
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
