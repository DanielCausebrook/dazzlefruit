use std::iter::repeat_with;
use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::ComponentInfo;
use crate::pattern_builder::component::data::{BlendMode, DisplayPane, FrameSize, PixelFrame};
use crate::pattern_builder::component::property::cloning::BlendModeProperty;
use crate::pattern_builder::component::property::num::NumProperty;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::texture::{Texture, TextureProperty};

#[derive(Clone)]
pub struct Repeater {
    info: ComponentInfo,
    texture: TextureProperty,
    pixels_per_repeat: NumProperty<FrameSize>,
}

impl Repeater {
    pub fn new(pixels_per_repeat: FrameSize, texture: impl Texture) -> Self {
        Self {
            info: ComponentInfo::new("Repeater"),
            texture: TextureProperty::new(Box::new(texture), PropertyInfo::new("Texture").display_pane(DisplayPane::Tree)),
            pixels_per_repeat: NumProperty::new(pixels_per_repeat, PropertyInfo::new("Pixels per Repeat")),
        }
    }
}

impl_component_config!(self: Repeater, self.info, [
    self.texture,
    self.pixels_per_repeat,
]);

impl_component!(self: Repeater, *self, "pixel");

impl Texture for Repeater {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let mini_frame = self.texture.write().next_frame(t, self.pixels_per_repeat.get());
        repeat_with(|| mini_frame.clone()).flatten().take(num_pixels as usize).collect()
    }
}
