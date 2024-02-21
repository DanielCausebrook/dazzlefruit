use serde::Serialize;
use crate::pattern_builder::component::RandId;

#[derive(Clone, Serialize)]
pub struct DebugMessagePayload { pub message: String, }

#[derive(Clone, Serialize)]
pub struct ConnectionOpenPayload { pub ip: String, }

#[derive(Clone, Serialize)]
pub struct PixelUpdatePayload { pub id: RandId, pub pixel_data: Vec<(u8, u8, u8, u8)> }