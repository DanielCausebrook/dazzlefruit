use std::collections::HashMap;
use futures::StreamExt;
use rand::random;
use tauri::{AppHandle, Manager};
use tauri::async_runtime::{JoinHandle, spawn};
use tokio::sync::RwLockWriteGuard;
use tokio::sync::watch::Receiver;
use tokio_stream::wrappers::WatchStream;
use crate::{AppState, LockedAppState};
use component::data::RandId;
use crate::pattern_builder::component::data::PixelFrame;
use crate::pattern_builder::component::property::{PropReadGuard, PropView, PropWriteGuard};
use crate::pattern_builder::component::layer::texture::{TextureLayer};
use crate::pattern_builder::pattern::Pattern;
use crate::pattern_builder::library::core::empty::{Empty};
use crate::tauri_events::PixelUpdatePayload;

pub mod library;
pub mod component;
pub mod math_functions;
pub mod pattern_context;
pub mod pattern;

pub struct PatternBuilder {
    id: u64,
    num_pixels: usize,
    pattern: Pattern,
    pixel_updater_handle: JoinHandle<()>,
    property_map: HashMap<RandId, PropView>,
}

impl PatternBuilder {
    pub fn new(app_handle: AppHandle, num_pixels: usize) -> PatternBuilder {
        let id = random();
        let pattern = Pattern::new(Empty::new_texture_layer(), num_pixels);
        let mut update_receiver = WatchStream::new(pattern.get_frame_receiver());
        Self {
            id,
            num_pixels,
            pixel_updater_handle: spawn(async move {
                while let Some(pixel_data) = update_receiver.next().await {
                    app_handle.emit_all(
                        "pixel-update",
                        PixelUpdatePayload { id, pixel_data: pixel_data.into_srgba_components() },
                    ).unwrap();
                }
            }),
            pattern: pattern,
            property_map: HashMap::new(),
        }
    }

    pub fn read_texture(&self) -> PropReadGuard<'_, TextureLayer> {
        self.pattern.layer().read()
    }

    pub fn write_texture(&self) -> PropWriteGuard<'_, TextureLayer> {
        self.pattern.layer().write()
    }

    pub fn set_texture(&mut self, texture: TextureLayer) {
        self.pattern.layer().try_replace_value(texture).unwrap();
    }

    pub fn get_pattern_update_receiver(&self) -> Receiver<PixelFrame> {
        self.pattern.get_frame_receiver()
    }
}

#[tauri::command]
pub async fn get_pattern_config(tauri_state: tauri::State<'_, LockedAppState>) -> Result<String, String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    let view = state.pattern_builder.pattern.view();
    state.pattern_builder.property_map = view.generate_property_map();
    // eprintln!("{}", serde_json::to_string(&view_data).unwrap());
    serde_json::to_string(&view).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_property(id: RandId, value: String, tauri_state: tauri::State<'_, LockedAppState>) -> Result<(), String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    let property = state.pattern_builder.property_map.get_mut(&id).ok_or("Unknown property id")?;
    property.try_update(value.as_str())?;
    Ok(())
}
