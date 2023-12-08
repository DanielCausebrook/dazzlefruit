use crate::pattern_builder::component::data::DisplayPane;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::layer::texture::{Texture, TextureLayer};
use crate::pattern_builder::component::layer::texture_generator::TextureGenerator;
use crate::pattern_builder::library::core::empty::{Empty, EmptyTexture};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::LayerInfo;

#[derive(Clone)]
pub struct CyclicTextureGenerator {
    textures: Prop<Vec<TextureLayer>>,
    next_texture: usize,
}

impl CyclicTextureGenerator {
    pub fn new(textures: Vec<TextureLayer>) -> CyclicTextureGenerator {
        let shared_textures = textures.into_iter()
            .map(|t| t)
            .collect();
        CyclicTextureGenerator {
            textures: RawPropCore::new(shared_textures).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            next_texture: 0,
        }
    }
}

impl Component for CyclicTextureGenerator {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties![
            self.textures,
        ]
    }

    fn detach(&mut self) {
        fork_properties!(
            self.textures,
        );
    }
}

impl TextureGenerator for CyclicTextureGenerator {
    fn next_texture(&mut self) -> TextureLayer {
        let textures = self.textures.read();
        if textures.is_empty() {
            Empty::<EmptyTexture>::new().into_layer(LayerInfo::new("Empty"))
        } else {
            let texture = textures.get(self.next_texture % textures.len()).unwrap();
            self.next_texture = (self.next_texture + 1) % textures.len();
            texture.clone()
        }
    }
}
