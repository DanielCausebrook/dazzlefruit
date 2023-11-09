use palette::WithAlpha;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::pattern_builder::component::{Component, ComponentConfig, ComponentInfo};
use crate::pattern_builder::component::texture::Texture;
use crate::pattern_builder::component::data::{DisplayPane, Frame, FrameSize, PixelFrame};
use crate::pattern_builder::component::filter::Filter;
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::cloning::BlendModeProperty;
use crate::pattern_builder::component::property::locked::{LockedProperty};

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

impl ComponentConfig for GroupLayer {
    fn info(&self) -> &ComponentInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        &mut self.info
    }

    fn properties(&self) -> Vec<&dyn Property> {
        vec![&self.layers]
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        vec![&mut self.layers]
    }
}

impl Component for GroupLayer {
    fn config(&self) -> &dyn ComponentConfig { self }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig { self }

    fn component_type(&self) -> &'static str { "group" }
}

impl Texture for GroupLayer {
    fn get_blend_mode(&self) -> &BlendModeProperty {
        &self.blend_mode
    }

    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let mut pixel_data = self.layers.write().iter_mut()
            .fold(None, |active_option, layer| {
                match layer {
                    Layer::Pixel(pixel_layer) => {
                        let pixel_data = pixel_layer.next_frame(t, num_pixels);
                        match active_option {
                            Some(active) => Some(pixel_data.blend(active, pixel_layer.get_blend_mode().get())),
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

impl Component for Layer {
    fn config(&self) -> &dyn ComponentConfig {
        match self {
            Layer::Pixel(layer) => layer.config(),
            Layer::Filter(layer) => layer.config(),
        }
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        match self {
            Layer::Pixel(layer) => layer.config_mut(),
            Layer::Filter(layer) => layer.config_mut(),
        }
    }

    fn component_type(&self) -> &'static str {
        match self {
            Layer::Pixel(layer) => layer.component_type(),
            Layer::Filter(layer) => layer.component_type(),
        }
    }
}

type LayerVecProperty = LockedProperty<Vec<Layer>>;

impl Property for LayerVecProperty {
    fn get_info(&self) -> &PropertyInfo { &self.get_info() }
    fn get_type_id(&self) -> &'static str { "layerVec" }
    fn for_each_child_component<'a>(&self, mut func: Box<dyn FnMut(&dyn Component) + 'a>) {
        for layer in self.read().iter() {
            func(layer);
        }
    }
    fn for_each_child_component_mut<'a>(&mut self, mut func: Box<dyn FnMut(&mut dyn Component) + 'a>) {
        for layer in self.write().iter_mut() {
            func(layer)
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
        self.get_info().serialize_into::<S>(&mut struct_ser)?;
        struct_ser.serialize_field("property_type", self.get_type_id())?;
        struct_ser.serialize_field("value", &self.read().iter()
            .map(|layer| layer.config().info().get_id())
            .collect::<Vec<_>>()
        )?;
        struct_ser.end()
    }
}
