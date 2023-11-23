use palette::WithAlpha;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::{Component, ComponentInfo};
use crate::pattern_builder::component::data::{BlendMode, DisplayPane, Frame, FrameSize, PixelFrame};
use crate::pattern_builder::component::filter::Filter;
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::cloning::BlendModeProperty;
use crate::pattern_builder::component::property::locked::LockedProperty;
use crate::pattern_builder::component::texture::Texture;

#[derive(Clone)]
pub struct GroupLayer {
    info: ComponentInfo,
    layers: LayerVecProperty,
    blend_mode: BlendModeProperty,
}

impl GroupLayer {
    pub fn new() -> Self {
        Self {
            info: ComponentInfo::new("Group"),
            layers: LayerVecProperty::new(vec![], PropertyInfo::unnamed().display_pane(DisplayPane::Tree)),
            blend_mode: BlendModeProperty::default(),
        }
    }

    pub fn add_pixel_layer(&self, layer: impl Texture) {
        self.layers.write().push(Layer::Pixel(Box::new(layer)))
    }

    pub fn add_filter_layer(&self, layer: impl Filter) {
        self.layers.write().push(Layer::Filter(Box::new(layer)))
    }
}

impl_component!(self: GroupLayer, *self, "group");

impl_component_config!(self: GroupLayer, self.info, [
    self.layers
]);

impl Texture for GroupLayer {
    fn blend_mode(&self) -> BlendMode {
        self.blend_mode.get()
    }

    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let mut pixel_data = self.layers.write().iter_mut()
            .fold(None, |active_option, layer| {
                match layer {
                    Layer::Pixel(pixel_layer) => {
                        let pixel_data = pixel_layer.next_frame(t, num_pixels);
                        match active_option {
                            Some(active) => Some(pixel_data.blend(active, pixel_layer.blend_mode())),
                            None => Some(pixel_data),
                        }
                    },
                    Layer::Filter(filter_layer) => {
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
enum Layer {
    Pixel(Box<dyn Texture>),
    Filter(Box<dyn Filter>),
}

impl Layer {
    fn as_component_ref(&self) -> &dyn Component {
        match self {
            Layer::Pixel(ref layer) => layer,
            Layer::Filter(ref layer) => layer,
        }
    }

    fn as_component_mut(&mut self) -> &mut dyn Component {
        match self {
            Layer::Pixel(ref mut layer) => layer,
            Layer::Filter(ref mut  layer) => layer,
        }
    }
}

type LayerVecProperty = LockedProperty<Vec<Layer>>;

impl Property for LayerVecProperty {
    fn info(&self) -> &PropertyInfo { &self.info() }
    fn type_id(&self) -> &'static str { "componentVec" }
    fn for_each_child_component<'a>(&self, mut func: Box<dyn FnMut(&dyn Component) + 'a>) {
        for layer in self.read().iter() {
            func(layer.as_component_ref());
        }
    }
    fn for_each_child_component_mut<'a>(&mut self, mut func: Box<dyn FnMut(&mut dyn Component) + 'a>) {
        for layer in self.write().iter_mut() {
            func(layer.as_component_mut())
        }
    }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        todo!()
    }

    fn shallow_detach(&mut self) { self.shallow_detach() }
}

impl Serialize for LayerVecProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Property", 6)?;
        self.info().serialize_into::<S>(&mut struct_ser)?;
        struct_ser.serialize_field("property_type", self.type_id())?;
        struct_ser.serialize_field("value", &self.read().iter()
            .map(|layer| layer.as_component_ref().config().info().get_id())
            .collect::<Vec<_>>()
        )?;
        struct_ser.end()
    }
}
