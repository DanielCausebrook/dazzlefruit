use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::ComponentInfo;
use crate::pattern_builder::component::texture::Texture;
use crate::pattern_builder::component::data::{BlendMode, DisplayPane, FrameSize, Pixel, PixelFrame};
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::cloning::{BlendModeProperty, ColorProperty};

#[derive(Clone)]
pub struct SolidColor {
    info: ComponentInfo,
    blend_mode: BlendModeProperty,
    color: ColorProperty,
}

impl SolidColor {
    pub fn new(color: Pixel) -> Self {
        Self {
            info: ComponentInfo::new("Color"),
            blend_mode: BlendModeProperty::default(),
            color: ColorProperty::new(color, PropertyInfo::unnamed().display_pane(DisplayPane::Tree)),
        }
    }
}

impl_component!(self: SolidColor, *self, "pixel");

impl_component_config!(self: SolidColor, self.info, [
    self.color,
]);

impl Texture for SolidColor {
    fn blend_mode(&self) -> BlendMode {
        self.blend_mode.get()
    }

    fn next_frame(&mut self, _t: f64, num_pixels: FrameSize) -> PixelFrame {
        vec![self.color.get(); num_pixels as usize]
    }
}