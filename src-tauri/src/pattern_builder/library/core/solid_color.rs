use crate::pattern_builder::component::{ComponentInfo, ComponentConfig, Component};
use crate::pattern_builder::component::texture::Texture;
use crate::pattern_builder::component::data::{DisplayPane, FrameSize, Pixel, PixelFrame};
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

impl ComponentConfig for SolidColor {
    fn info(&self) -> &ComponentInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        &mut self.info
    }

    fn properties(&self) -> Vec<&dyn Property> {
        vec![&self.color]
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        vec![&mut self.color]
    }
}

impl Component for SolidColor {
    fn config(&self) -> &dyn ComponentConfig {
        self
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        self
    }

    fn component_type(&self) -> &'static str {
        "pixel"
    }
}

impl Texture for SolidColor {
    fn get_blend_mode(&self) -> &BlendModeProperty {
        &self.blend_mode
    }

    fn next_frame(&mut self, _t: f64, num_pixels: FrameSize) -> PixelFrame {
        vec![self.color.get(); num_pixels as usize]
    }
}