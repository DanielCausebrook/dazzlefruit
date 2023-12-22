use std::sync::Arc;
use tokio::sync::{broadcast, Mutex, RwLockReadGuard, RwLockWriteGuard};

use tauri::async_runtime::{JoinHandle, spawn};
use crate::{AppState, LockedAppState};
use crate::pattern_builder::component::frame::{ColorPixel, Frame};
use crate::pattern_builder::component::RandId;
use crate::pico_connection::packet_types::{TcpPacketType, UdpPacketType};
use crate::pico_connection::PicoConnectionHandle;

#[derive(Clone)]
struct NeopixelControllerData {
    pico_connection: PicoConnectionHandle,
    selected_pattern_id: Arc<Mutex<Option<RandId>>>,
    num_pixels: u16,
}

pub struct NeopixelController {
    data: NeopixelControllerData,
    listener_handle: JoinHandle<()>,
}

impl Drop for NeopixelController {
    fn drop(&mut self) {
        self.listener_handle.abort();
    }
}

impl NeopixelControllerData {

    async fn display(&self, mut pixel_data: Frame<ColorPixel>) {
        pixel_data.resize_with_empty(self.num_pixels as usize);
        let bytes = pixel_data.iter()
            .flat_map(|color| {
                let color_pre = color.premultiply();
                [
                    (color_pre.red * 255.0).round() as u8,
                    (color_pre.green * 225.0).round() as u8,
                    (color_pre.blue * 225.0).round() as u8,
                ]
            })
            .collect::<Vec<u8>>();
        self.pico_connection.send_udp(
            UdpPacketType::Neopixel_Show,
            &bytes
        ).await;
    }

    pub async fn show_pattern_id(&self, pattern_id: RandId) {
        *self.selected_pattern_id.lock().await = Some(pattern_id);
    }

    pub async fn show_none(&self) {
        *self.selected_pattern_id.lock().await = None;
        self.display(Frame::empty(self.num_pixels as usize)).await;
    }
}

impl NeopixelController {
    pub async fn new(pico_connection: PicoConnectionHandle, num_pixels: u16, mut pattern_update_receiver: broadcast::Receiver<(RandId, Frame<ColorPixel>)>) -> Result<Self, String> {
        let data = NeopixelControllerData{
            pico_connection,
            selected_pattern_id: Arc::new(Mutex::new(None)),
            num_pixels
        };
        let controller = Self {
            data: data.clone(),
            listener_handle: spawn(async move {
                loop {
                    let (pattern_id, frame) = pattern_update_receiver.recv().await.unwrap();
                    let selected_lock = data.selected_pattern_id.lock().await;
                    if Some(pattern_id) == *selected_lock {
                        data.display(frame).await;
                    }
                    drop(selected_lock);
                }
            }),
        };

        let init_data = num_pixels.to_be_bytes();
        match controller.data.pico_connection.send_tcp_await_response(TcpPacketType::Neopixel_Init, &init_data).await {
            Ok(Ok(())) => {},
            Ok(Err(msg)) => return Err(msg),
            Err(e) => return Err(e.to_string()),
        };
        Ok(controller)
    }
}

#[tauri::command]
pub async fn init_neopixel(num_pixels: u16, tauri_state: tauri::State<'_, LockedAppState>) -> Result<(), String> {
    let mut state: RwLockWriteGuard<AppState> = tauri_state.0.write().await;

    state.neopixel_controller = Some(NeopixelController::new(
        state.connection.clone().ok_or("Not connected to a pico.")?,
        num_pixels,
        state.pattern_builder.pattern_update_receiver()
    ).await?);
    Ok(())
}

#[tauri::command]
pub async fn set_neopixel_pattern(pattern_id: Option<RandId>, tauri_state: tauri::State<'_, LockedAppState>) -> Result<(), String> {
    let state: RwLockReadGuard<AppState> = tauri_state.0.read().await;

    let controller = state.neopixel_controller.as_ref().ok_or("Pico not connected!")?;
    if let Some(pattern_id) = pattern_id {
        if state.pattern_builder.pattern(pattern_id).is_none() {
            return Err(format!("Pattern with id {} not found", pattern_id));
        }
        controller.data.show_pattern_id(pattern_id).await;
    } else {
        controller.data.show_none().await;
    }
    Ok(())
}

