use crate::pattern_builder::component::layer::{DisplayPane, Layer, LayerTypeInfo};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::layer::{LayerCore};
use crate::pattern_builder::library::core::empty::empty_texture_layer;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct CyclicLayerGenerator {
    textures: Prop<Vec<Layer>>,
    next_texture: usize,
}

impl CyclicLayerGenerator {
    pub fn new(textures: Vec<Layer>) -> CyclicLayerGenerator {
        let shared_textures = textures.into_iter()
            .map(|t| t)
            .collect();
        CyclicLayerGenerator {
            textures: RawPropCore::new(shared_textures).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            next_texture: 0,
        }
    }

    pub fn into_layer(self) -> Layer {
        Layer::new(self, LayerTypeInfo::new("Cyclic Generator"))
    }
}

impl LayerCore for CyclicLayerGenerator {
    type Input = ();
    type Output = Layer;
    fn next(&mut self, _: (), _t: f64, _ctx: &PatternContext) -> Layer {
        let textures = self.textures.read();
        if textures.is_empty() {
            empty_texture_layer()
        } else {
            let texture = textures.get(self.next_texture % textures.len()).unwrap();
            self.next_texture = (self.next_texture + 1) % textures.len();
            texture.clone()
        }
    }

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
