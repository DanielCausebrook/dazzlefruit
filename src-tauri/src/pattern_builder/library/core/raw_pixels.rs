use palette::WithAlpha;
use crate::pattern_builder::component::texture::{Texture};
use crate::pattern_builder::component::basic_config::BasicPixelLayerConfig;
use crate::pattern_builder::component::{Component, ComponentConfig};
use crate::pattern_builder::component::data::{FrameSize, PixelFrame};
use crate::pattern_builder::component::property::cloning::BlendModeProperty;

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

impl Component for RawPixels {
    fn config(&self) -> &dyn ComponentConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        &mut self.config
    }

    fn component_type(&self) -> &'static str { "pixel" }
}

impl Texture for RawPixels {
    fn get_blend_mode(&self) -> &BlendModeProperty {
        &self.config.get_blend_mode()
    }

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