use palette::WithAlpha;

use crate::impl_component;
use crate::pattern_builder::component::basic_config::BasicPixelLayerConfig;
use crate::pattern_builder::component::data::{BlendMode, FrameSize, PixelFrame};
use crate::pattern_builder::component::texture::Texture;

#[derive(Clone)]
pub struct RawPixels {
    config: BasicPixelLayerConfig,
    pixels: PixelFrame,
}

impl RawPixels {
    pub fn new(pixels: PixelFrame) -> Self {
        Self { config: BasicPixelLayerConfig::new("Raw Pixels", None), pixels }
    }
}

impl_component!(self: RawPixels, self.config, "pixel");

impl Texture for RawPixels {
    fn next_frame(&mut self, _t: f64, num_pixels: FrameSize) -> PixelFrame {
        let mut pixels = self.pixels.clone();
        pixels.resize_with(num_pixels as usize, || palette::named::BLACK.with_alpha(0.0).into_linear());
        pixels
    }
}

impl From<PixelFrame> for RawPixels {
    fn from(value: PixelFrame) -> Self {
        RawPixels::new(value)
    }
}