use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::{ComponentInfo, ComponentConfig, Component};
use crate::pattern_builder::component::data::{FrameSize, PixelFrame};
use crate::pattern_builder::component::property::cloning::{BlendModeProperty, ColorProperty};
use crate::pattern_builder::component::property::num::NumProperty;
use crate::pattern_builder::component::property::Property;
use crate::pattern_builder::component::texture::Texture;

#[derive(Clone)]
pub struct ColorRange {
    info: ComponentInfo,
    color: ColorProperty,
    variance: NumProperty<f64>,
    speed: NumProperty<f64>,
    blend_mode: BlendModeProperty,
}

impl_component!(self: ColorRange, *self, "pixel");

impl_component_config!(self: ColorRange, self.info, [
    self.color,
    self.variance,
    self.speed,
]);

impl Texture for ColorRange {
    fn get_blend_mode(&self) -> &BlendModeProperty {
        &self.blend_mode
    }

    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let color = self.color.get();
        todo!()
    }
}
