use tokio::sync::{RwLockWriteGuard, watch};

use tauri::async_runtime::{JoinHandle, spawn};
use crate::{AppState, LockedAppState};
use crate::pattern_builder::component::data::PixelFrame;

use crate::pico_connection::packet_types::{TcpPacketType, UdpPacketType};
use crate::pico_connection::PicoConnectionHandle;

#[derive(Clone)]
struct NeopixelControllerData {
    pico_connection: PicoConnectionHandle,
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

    async fn display(&self, mut pixel_data: PixelFrame) {
        pixel_data.resize_with_transparent(self.num_pixels as usize);
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
}

impl NeopixelController {
    pub async fn new(pico_connection: PicoConnectionHandle, num_pixels: u16, mut pixel_update_receiver: watch::Receiver<PixelFrame>) -> Result<Self, String> {
        let data = NeopixelControllerData{ pico_connection, num_pixels };
        let controller = Self {
            data: data.clone(),
            listener_handle: spawn(async move {
                loop {
                    pixel_update_receiver.changed().await.unwrap();
                    let pixel_data = pixel_update_receiver.borrow().clone();
                    data.display(pixel_data).await;
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
        state.pattern_builder.get_pattern_update_receiver()
    ).await?);
    Ok(())
}