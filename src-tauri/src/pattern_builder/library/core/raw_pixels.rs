use palette::WithAlpha;

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{FrameSize, PixelFrame};
use crate::pattern_builder::component::property::{PropView};
use crate::pattern_builder::component::layer::texture::Texture;

#[derive(Clone)]
pub struct RawPixels {
    pixels: PixelFrame,
}

impl RawPixels {
    pub fn new(pixels: PixelFrame) -> Self {
        Self { pixels }
    }
}

impl Component for RawPixels {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!()
    }

    fn detach(&mut self) {
        fork_properties!();
    }
}

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