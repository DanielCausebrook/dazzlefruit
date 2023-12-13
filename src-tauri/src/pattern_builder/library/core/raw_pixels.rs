use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::PixelFrame;
use crate::pattern_builder::component::property::{PropView};
use crate::pattern_builder::component::layer::texture::Texture;
use crate::pattern_builder::pattern_context::PatternContext;

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
    fn next_frame(&mut self, _t: f64, ctx: &PatternContext) -> PixelFrame {
        let mut pixels = self.pixels.clone();
        pixels.resize_with_transparent(ctx.num_pixels());
        pixels
    }
}

impl From<PixelFrame> for RawPixels {
    fn from(value: PixelFrame) -> Self {
        RawPixels::new(value)
    }
}