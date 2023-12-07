use std::mem;
use palette::WithAlpha;
use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq};

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::{Layer, Component};
use crate::pattern_builder::component::data::{DisplayPane, Frame, FrameSize, PixelFrame};
use crate::pattern_builder::component::filter::{Filter, FilterLayer};
use crate::pattern_builder::component::property::{PropertyInfo};
use crate::pattern_builder::component::property::{PropCore, ErasedPropCore, Prop, PropRead, PropWrite, PropView};
use crate::pattern_builder::component::texture::{Texture, TextureLayer};

#[derive(Clone)]
pub struct Group {
    layers: Prop<Vec<GroupedLayer>>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            layers: GroupedLayerVecProp::new().into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }

    pub fn add_texture(&self, texture: TextureLayer) {
        self.layers.write().push(GroupedLayer::Texture(texture))
    }

    pub fn add_filter(&self, filter: FilterLayer) {
        self.layers.write().push(GroupedLayer::Filter(filter))
    }
}

impl Component for Group {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.layers)
    }

    fn detach(&mut self) {
        fork_properties!(self.layers);
    }
}

impl Texture for Group {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let mut pixel_data = self.layers.write().iter_mut()
            .fold(None, |active_option, layer| {
                match layer {
                    GroupedLayer::Texture(pixel_layer) => {
                        let pixel_data: PixelFrame = pixel_layer.next_frame(t, num_pixels).into_iter().map(|pixel| pixel.with_alpha(pixel.alpha * *pixel_layer.opacity().read())).collect();
                        match active_option {
                            Some(active) => Some(pixel_data.blend(active, *pixel_layer.blend_mode().read())),
                            None => Some(pixel_data),
                        }
                    },
                    GroupedLayer::Filter(filter_layer) => {
                        active_option.map(|active| filter_layer.next_frame(t, active))
                    }
                }
            })
            .unwrap_or_else(|| vec![]);
        pixel_data.resize_with(num_pixels as usize, || palette::named::BLACK.with_alpha(0.0).into_linear());
        pixel_data
    }
}

#[derive(Clone)]
enum GroupedLayer {
    Texture(TextureLayer),
    Filter(FilterLayer),
}

impl GroupedLayer {
    pub fn as_component_ref(&self) -> &dyn Layer {
        match self {
            GroupedLayer::Texture(ref layer) => layer,
            GroupedLayer::Filter(ref layer) => layer,
        }
    }

    pub fn as_component_mut(&mut self) -> &mut dyn Layer {
        match self {
            GroupedLayer::Texture(ref mut layer) => layer,
            GroupedLayer::Filter(ref mut  layer) => layer,
        }
    }
}

#[derive(Clone)]
struct GroupedLayerVecProp(Vec<GroupedLayer>);

impl GroupedLayerVecProp {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn fork(&self) -> Self {
        let mut clone = self.clone();
        for layer in clone.0.iter_mut() {
            match layer {
                GroupedLayer::Texture(layer) => layer.detach(),
                GroupedLayer::Filter(layer) => layer.detach(),
            }
        }
        clone
    }
}

impl PropCore for GroupedLayerVecProp {
    type Value = Vec<GroupedLayer>;

    fn read(&self) -> PropRead<Self::Value> {
        PropRead::Ref(&self.0)
    }

    fn write(&mut self) -> PropWrite<Self::Value> {
        PropWrite::Ref(&mut self.0)
    }

    fn try_replace(&mut self, value: Self::Value) -> Result<Self::Value, String> where Self::Value: Sized {
        Ok(mem::replace(&mut self.0, value))
    }

    fn fork_dyn(&self) -> Box<dyn PropCore<Value=Self::Value>> {
        Box::new(self.fork())
    }
}

impl ErasedPropCore for GroupedLayerVecProp {
    fn prop_type_id(&self) -> String {
        "component-vec".to_string()
    }

    fn for_each_child_layer<'a>(&self, func: &mut (dyn FnMut(&dyn Layer) + 'a)) {
        for layer in self.0.iter() {
            func(layer.as_component_ref())
        }
    }

    fn for_each_child_layer_mut<'a>(&mut self, func: &mut (dyn FnMut(&mut dyn Layer) + 'a)) {
        for layer in self.0.iter_mut() {
            func(layer.as_component_mut())
        }
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        todo!()
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(ComponentVecSerializer(&self.0))
    }
}

struct ComponentVecSerializer<'a> (&'a Vec<GroupedLayer>);

impl Serialize for ComponentVecSerializer<'_>{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut seq_ser = serializer.serialize_seq(Some(self.0.len()))?;
        for layer in &*self.0 {
            seq_ser.serialize_element(&layer.as_component_ref().info().id())?;
        }
        seq_ser.end()
    }
}
