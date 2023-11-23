use dyn_clone::{clone_trait_object, DynClone};
use crate::pattern_builder::component::{Component, ComponentConfig};
use crate::pattern_builder::component::shared_component::SharedComponent;
use crate::pattern_builder::component::data::{FrameSize, PixelFrame};
use crate::pattern_builder::component::property::locked::{ComponentProperty, ComponentVecProperty};

use crate::pattern_builder::library::core::RawPixels;
use crate::pattern_builder::library::core::texture_layer::TextureLayer;

pub trait Texture: Component + DynClone {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame;

    fn into_layer(self) -> TextureLayer where Self: Sized {
        TextureLayer::new(self)
    }
}
clone_trait_object!(Texture);

impl Component for Box<dyn Texture> {
    fn config(&self) -> &dyn ComponentConfig {
        self.as_ref().config()
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        self.as_mut().config_mut()
    }

    fn component_type(&self) -> &'static str {
        self.as_ref().component_type()
    }
}

impl Texture for Box<dyn Texture> {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame { self.as_mut().next_frame(t, num_pixels) }
}

impl<T: Texture + Clone> Texture for SharedComponent<T> {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        self.write().next_frame(t, num_pixels)
    }
}

impl From<PixelFrame> for Box<dyn Texture> {
    fn from(value: PixelFrame) -> Self {
        Box::new(RawPixels::new(value))
    }
}

pub type TextureProperty = ComponentProperty<Box<dyn Texture>>;
pub type TextureVecProperty = ComponentVecProperty<Box<dyn Texture>>;
