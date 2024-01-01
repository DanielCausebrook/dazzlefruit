use crate::pattern_builder::component::layer::{DisplayPane, LayerTypeInfo};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::layer::texture::TextureLayer;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::{LayerCore};
use crate::pattern_builder::component::layer::generic::GenericLayer;
use crate::pattern_builder::component::layer::standard_types::{TEXTURE_LAYER, VOID};
use crate::pattern_builder::library::core::empty::empty_texture_layer;
use crate::pattern_builder::pattern_context::PatternContext;

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

    pub fn into_layer(self) -> GenericLayer<Self> {
        GenericLayer::new(self, LayerTypeInfo::new("Cyclic Generator"), &VOID, &TEXTURE_LAYER)
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

impl LayerCore for CyclicTextureGenerator {
    type Input = ();
    type Output = TextureLayer;
    fn next(&mut self, _: (), _t: f64, _ctx: &PatternContext) -> TextureLayer {
        let textures = self.textures.read();
        if textures.is_empty() {
            empty_texture_layer()
        } else {
            let texture = textures.get(self.next_texture % textures.len()).unwrap();
            self.next_texture = (self.next_texture + 1) % textures.len();
            texture.clone()
        }
    }
}
