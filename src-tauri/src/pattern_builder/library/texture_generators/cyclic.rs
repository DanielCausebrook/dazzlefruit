use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::ComponentInfo;
use crate::pattern_builder::component::data::DisplayPane;
use crate::pattern_builder::component::property::locked::ComponentVecProperty;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::shared_component::SharedComponent;
use crate::pattern_builder::component::texture::Texture;
use crate::pattern_builder::component::texture_generator::TextureGenerator;
use crate::pattern_builder::library::core::empty::Empty;

#[derive(Clone)]
pub struct CyclicTextureGenerator {
    info: ComponentInfo,
    textures: ComponentVecProperty<SharedComponent<Box<dyn Texture>>>,
    next_texture: usize,
}

impl CyclicTextureGenerator {
    pub fn new(textures: Vec<Box<dyn Texture>>) -> CyclicTextureGenerator {
        let shared_textures = textures.into_iter()
            .map(|t| SharedComponent::new(t))
            .collect();
        CyclicTextureGenerator {
            info: ComponentInfo::new("Cycling Texture"),
            textures: ComponentVecProperty::new(shared_textures, PropertyInfo::unnamed().display_pane(DisplayPane::Tree)),
            next_texture: 0,
        }
    }
}


impl_component!(self: CyclicTextureGenerator, *self, "generator");

impl_component_config!(self: CyclicTextureGenerator, self.info, [
    self.textures
]);

impl TextureGenerator for CyclicTextureGenerator {
    fn next_texture(&mut self) -> SharedComponent<Box<dyn Texture>> {
        let textures = self.textures.read();
        if textures.is_empty() {
            SharedComponent::new(Box::new(Empty::new()))
        } else {
            let texture = textures.get(self.next_texture % textures.len()).unwrap();
            self.next_texture = (self.next_texture + 1) % textures.len();
            texture.clone()
        }
    }
}
