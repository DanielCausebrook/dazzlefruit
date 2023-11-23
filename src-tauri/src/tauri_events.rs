
#[derive(Clone, serde::Serialize)]
pub struct DebugMessagePayload { pub message: String, }

#[derive(Clone, serde::Serialize)]
pub struct ConnectionOpenPayload { pub ip: String, }

#[derive(Clone, serde::Serialize)]
pub struct PixelUpdatePayload { pub id: u64, pub pixel_data: Vec<(u8, u8, u8, u8)> }