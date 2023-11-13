use crate::pattern_builder::component::{ComponentInfo, ComponentConfig};
use crate::pattern_builder::component::property::Property;
use crate::pattern_builder::component::property::cloning::BlendModeProperty;

#[derive(Clone)]
pub struct BasicPixelLayerConfig {
    info: ComponentInfo,
    blend_mode: BlendModeProperty,
}

impl BasicPixelLayerConfig {
    pub fn new(name: &str, description: Option<&str>) -> Self {
        Self {
            info: ComponentInfo::new(name).description(description),
            blend_mode: BlendModeProperty::default(),
        }
    }
    pub fn blend_mode(&self) -> &BlendModeProperty {
        &self.blend_mode
    }
}

impl ComponentConfig for BasicPixelLayerConfig {
    fn info(&self) -> &ComponentInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        &mut self.info
    }

    fn properties(&self) -> Vec<&dyn Property> {
        vec![]
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        vec![]
    }
}

#[derive(Clone)]
pub struct BasicConfig {
    info: ComponentInfo,
}

impl BasicConfig {
    pub fn new(name: &str, description: Option<&str>) -> Self {
        Self {
            info: ComponentInfo::new(name).description(description),
        }
    }
}

impl ComponentConfig for BasicConfig {
    fn info(&self) -> &ComponentInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        &mut self.info
    }

    fn properties(&self) -> Vec<&dyn Property> {
        vec![]
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        vec![]
    }
}
