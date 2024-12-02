use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use futures::StreamExt;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tauri::async_runtime::{JoinHandle, spawn};
use tokio::sync::{broadcast, RwLockReadGuard, RwLockWriteGuard, watch};
use tokio_stream::wrappers::WatchStream;
use crate::{AppState, LockedAppState};
use component::RandId;
use crate::pattern_builder::component::frame::{ColorPixel, Frame, ScalarPixel};
use crate::pattern_builder::component::layer::io_type::DynTypeMapper;
use crate::pattern_builder::component::layer::Layer;
use crate::pattern_builder::pattern::Pattern;
use crate::pattern_builder::pattern_context::PatternContext;
use crate::pattern_builder::pattern_context::position_map::PositionMap;
use crate::tauri_events::PixelUpdatePayload;

pub mod library;
pub mod component;
pub mod math_functions;
pub mod pattern_context;
pub mod pattern;

mod standard_types {
    use crate::pattern_builder::component::layer::io_type::{DynTypeInfo};
    use crate::{impl_dyn_type, static_dyn_type_def};
    use crate::pattern_builder::component::frame::{ColorPixel, Frame, ScalarPixel};
    use crate::pattern_builder::component::layer::Layer;

    static_dyn_type_def!(VOID = DynTypeInfo::new("()"));
    impl_dyn_type!((): VOID);

    static_dyn_type_def!(COLOR_FRAME = DynTypeInfo::new("ColorFrame"));
    impl_dyn_type!(Frame<ColorPixel>: COLOR_FRAME);

    static_dyn_type_def!(SCALAR_FRAME = DynTypeInfo::new("ScalarFrame"));
    impl_dyn_type!(Frame<ScalarPixel>: SCALAR_FRAME);

    static_dyn_type_def!(LAYER = DynTypeInfo::new("Layer"));
    impl_dyn_type!(Layer: LAYER);
}

pub struct PatternBuilder {
    open_patterns: HashMap<RandId, OpenPattern>,
    pattern_ordering: Vec<RandId>,
    pattern_context: watch::Sender<PatternContext<'static>>,
    type_mapper: Arc<DynTypeMapper>,
    pattern_update_sender: broadcast::Sender<(RandId, Frame<ColorPixel>)>,
    app_handle: AppHandle,
}

impl PatternBuilder {
    pub fn new(app_handle: AppHandle, num_pixels: usize) -> PatternBuilder {

        let mut type_mapper = DynTypeMapper::default();
        type_mapper.add_basic_mappings::<()>();
        type_mapper.add_basic_mappings::<Frame<ColorPixel>>();
        type_mapper.add_basic_mappings::<Frame<ScalarPixel>>();
        type_mapper.add_basic_mappings::<Layer>();
        let arc_type_mapper = Arc::new(type_mapper);

        Self {
            open_patterns: HashMap::new(),
            pattern_ordering: vec![],
            type_mapper: arc_type_mapper.clone(),
            pattern_context: watch::channel(PatternContext::new(num_pixels, arc_type_mapper)).0,
            pattern_update_sender: broadcast::channel(100).0,
            app_handle,
        }
    }

    pub fn load_pattern(&mut self, pattern: Pattern) {
        let app_handle = self.app_handle.clone();
        let mut update_receiver = WatchStream::new(pattern.get_frame_receiver());
        let id = pattern.id();
        let update_sender = self.pattern_update_sender.clone();
        let open_pattern = OpenPattern {
            pixel_updater_handle: spawn(async move {
                while let Some(pixel_data) = update_receiver.next().await {
                    let _ = update_sender.send((id, pixel_data.clone()));
                    app_handle.emit(
                        "pixel-update",
                        PixelUpdatePayload { id, pixel_data: pixel_data.into_srgba_components() },
                    ).unwrap();
                }
            }),
            pattern,
        };
        self.pattern_ordering.push(open_pattern.pattern.id());
        self.open_patterns.insert(open_pattern.pattern.id(), open_pattern);
    }

    pub fn pattern(&self, id: RandId) -> Option<&Pattern> {
        self.open_patterns.get(&id).map(|open_pattern| &open_pattern.pattern)
    }

    pub fn pattern_mut(&mut self, id: RandId) -> Option<&mut Pattern> {
        self.open_patterns.get_mut(&id).map(|open_pattern| &mut open_pattern.pattern)
    }

    pub fn pattern_update_receiver(&self) -> broadcast::Receiver<(RandId, Frame<ColorPixel>)> {
        self.pattern_update_sender.subscribe()
    }

    pub fn pattern_context(&self) -> watch::Receiver<PatternContext<'static>> {
        self.pattern_context.subscribe()
    }

    pub fn load_position_map(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let file_contents = fs::read_to_string(path).map_err(|err| err.to_string())?;
        let position_map: PositionMap<'static> = serde_json::from_str(&*file_contents).map_err(|err| err.to_string())?;
        self.pattern_context.send_modify(|ctx| ctx.set_position_map(position_map));
        Ok(())
    }
}

struct OpenPattern {
    pattern: Pattern,
    pixel_updater_handle: JoinHandle<()>,
}

impl Drop for OpenPattern {
    fn drop(&mut self) {
        self.pixel_updater_handle.abort();
    }
}

#[tauri::command]
pub async fn view_open_patterns(tauri_state: tauri::State<'_, LockedAppState>) -> Result<String, String> {
    let state: RwLockReadGuard<AppState> = tauri_state.0.read().await;
    let p_b = &state.pattern_builder;
    #[derive(Serialize)]
    struct PatternInfo { id: RandId, name: String }
    serde_json::to_string(
        &p_b.pattern_ordering.iter()
            .map(|id| p_b.open_patterns.get(id).unwrap())
            .map(|open_pattern| PatternInfo { id: open_pattern.pattern.id(), name: open_pattern.pattern.name() })
            .collect::<Vec<_>>()
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn view_pattern(id: RandId, tauri_state: tauri::State<'_, LockedAppState>) -> Result<String, String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    let view = state.pattern_builder
        .pattern_mut(id).ok_or(format!("Unknown pattern id {}", id))?.view();
    // eprintln!("{}", serde_json::to_string(&view).unwrap());
    serde_json::to_string(&view).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_property(pattern_id: RandId, prop_id: RandId, value: String, tauri_state: tauri::State<'_, LockedAppState>) -> Result<(), String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    state.pattern_builder.open_patterns
        .get_mut(&pattern_id).ok_or(format!("Unknown pattern id {}", pattern_id))?
        .pattern
        .try_update_prop(prop_id, value)
}

#[tauri::command]
pub async fn position_map(tauri_state: tauri::State<'_, LockedAppState>) -> Result<String, String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    let pattern_context = state.pattern_builder.pattern_context.borrow();
    Ok(
        serde_json::to_string(&pattern_context.position_map())
            .expect("Failed converting Position Map to string.")
    )
}

#[tauri::command]
pub async fn load_position_map(path: String, tauri_state: tauri::State<'_, LockedAppState>) -> Result<String, String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;
    state.pattern_builder.load_position_map(path)?;
    let pattern_context = state.pattern_builder.pattern_context.borrow();
    Ok(
        serde_json::to_string(&pattern_context.position_map())
            .expect("Failed converting Position Map to string.")
    )
}