use dyn_clone::{clone_trait_object, DynClone};
use crate::pattern_builder::component::{ComponentInfo, ComponentConfig, Component};
use crate::pattern_builder::component::data::DisplayPane;
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::locked::TextureVecProperty;
use crate::pattern_builder::component::texture::{Texture};
use crate::pattern_builder::library::core::empty::Empty;

pub trait TextureGenerator: Component + DynClone + Send + Sync {
    fn next_texture(&mut self) -> Box<dyn Texture>;
}
clone_trait_object!(TextureGenerator);

#[derive(Clone)]
pub struct CyclingTextureGenerator {
    info: ComponentInfo,
    textures: TextureVecProperty,
    next_texture: usize,
}

impl CyclingTextureGenerator {
    pub fn new(textures: Vec<Box<dyn Texture>>) -> CyclingTextureGenerator {
        CyclingTextureGenerator {
            info: ComponentInfo::new("Cycling Texture"),
            textures: TextureVecProperty::new(textures, PropertyInfo::unnamed().display_pane(DisplayPane::Tree)),
            next_texture: 0,
        }
    }
}

impl Component for CyclingTextureGenerator {
    fn config(&self) -> &dyn ComponentConfig {
        self
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        self
    }

    fn component_type(&self) -> &'static str {
        "producer"
    }
}

impl ComponentConfig for CyclingTextureGenerator {
    fn info(&self) -> &ComponentInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        &mut self.info
    }

    fn properties(&self) -> Vec<&dyn Property> {
        vec![&self.textures]
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        vec![&mut self.textures]
    }
}

impl TextureGenerator for CyclingTextureGenerator {
    fn next_texture(&mut self) -> Box<dyn Texture> {
        let textures = self.textures.read();
        if textures.is_empty() {
            Box::new(Empty::new())
        } else {
            let texture = textures.get(self.next_texture % textures.len()).unwrap();
            self.next_texture = (self.next_texture + 1) % textures.len();
            texture.clone()
        }
    }
}
