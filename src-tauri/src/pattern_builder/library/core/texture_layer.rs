use crate::{impl_component};
use crate::pattern_builder::component::{ComponentConfig, ComponentInfo};
use crate::pattern_builder::component::data::{FrameSize, PixelFrame};
use crate::pattern_builder::component::property::cloning::BlendModeProperty;
use crate::pattern_builder::component::property::num::{NumProperty, NumSlider};
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::texture::Texture;


#[derive(Clone)]
pub struct TextureLayer {
    texture: Box<dyn Texture>,
    blend_mode: BlendModeProperty,
    opacity: NumProperty<f32>,
    cache: Option<PixelFrame>,
}

impl TextureLayer {
    pub fn new(texture: impl Texture) -> Self {
        Self {
            texture: Box::new(texture),
            blend_mode: BlendModeProperty::default(),
            opacity: NumProperty::new(1.0, PropertyInfo::new("Opacity")).set_slider(Some(NumSlider::new(0.0..1.0, 0.01))),
            cache: None,
        }
    }

    pub fn blend_mode(&self) -> &BlendModeProperty {
        &self.blend_mode
    }

    pub fn opacity(&self) -> f32 {
        self.opacity.get()
    }
}

impl_component!(self: TextureLayer, *self, "texture-box");

impl ComponentConfig for TextureLayer {
    fn info(&self) -> &ComponentInfo {
        self.texture.config().info()
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        self.texture.config_mut().info_mut()
    }

    fn properties(&self) -> Vec<&dyn Property> {
        let mut props = self.texture.config().properties();
        props.append(&mut vec![&self.blend_mode, &self.opacity]);
        props
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        let mut props = self.texture.config_mut().properties_mut();
        props.append(&mut vec![&mut self.blend_mode, &mut self.opacity]);
        props
    }
}

impl Texture for TextureLayer {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        self.texture.next_frame(t, num_pixels)
    }
}