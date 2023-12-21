use std::collections::HashMap;
use futures::StreamExt;
use rand::random;
use tauri::{AppHandle, Manager};
use tauri::async_runtime::{JoinHandle, spawn};
use tokio::sync::RwLockWriteGuard;
use tokio::sync::watch::Receiver;
use tokio_stream::wrappers::WatchStream;
use crate::{AppState, LockedAppState};
use component::RandId;
use crate::pattern_builder::component::frame::{ColorPixel, Frame};
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::layer::standard_types::{PIXEL_FRAME, VOID};
use crate::pattern_builder::component::property::PropView;
use crate::pattern_builder::pattern::Pattern;
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
        let pattern = Pattern::new(LayerStack::new(&VOID, &PIXEL_FRAME), num_pixels);
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

    pub fn set_texture(&mut self, texture: LayerStack<(), Frame<ColorPixel>>) {
        self.pattern.layer().try_replace_value(texture).unwrap();
    }

    pub fn get_pattern_update_receiver(&self) -> Receiver<Frame<ColorPixel>> {
        self.pattern.get_frame_receiver()
    }
}

#[tauri::command]
pub async fn get_pattern_config(tauri_state: tauri::State<'_, LockedAppState>) -> Result<String, String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    let view = state.pattern_builder.pattern.view();
    state.pattern_builder.property_map = view.generate_property_map();
    // eprintln!("{}", serde_json::to_string(&view).unwrap());
    serde_json::to_string(&view).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_property(id: RandId, value: String, tauri_state: tauri::State<'_, LockedAppState>) -> Result<(), String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    let property = state.pattern_builder.property_map.get_mut(&id).ok_or("Unknown property id")?;
    property.try_update(value.as_str())?;
    Ok(())
}
