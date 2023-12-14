use std::collections::HashMap;
use rand::random;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::RandId;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::string::StringProp;

pub mod texture;
pub mod filter;
pub mod texture_generator;

pub trait Layer: Component + Send + Sync + Clone {
    fn layer_type(&self) -> String;
    fn info(&self) -> &LayerInfo;
    fn view(&self) -> LayerView {
        LayerView::new(self)
    }
}

impl<T> Layer for Box<T> where T: Layer + Clone + ?Sized {
    fn layer_type(&self) -> String {
        self.as_ref().layer_type()
    }

    fn info(&self) -> &LayerInfo {
        self.as_ref().info()
    }

    fn view(&self) -> LayerView {
        self.as_ref().view()
    }
}

pub struct LayerView {
    type_str: String,
    info: LayerInfo,
    property_views: Vec<PropView>,
    data: HashMap<String, Box<dyn erased_serde::Serialize + 'static>>,
}

impl LayerView {
    pub fn new(layer: &(impl Layer + ?Sized)) -> Self {
        Self {
            type_str: layer.layer_type(),
            info: layer.info().clone(),
            property_views: layer.view_properties(),
            data: HashMap::new(),
        }
    }

    pub fn add_data(mut self, key: &str, value: impl Serialize + 'static) -> Self {
        self.data.insert(key.to_string(), Box::new(value));
        self
    }

    pub fn info(&self) -> &LayerInfo {
        &self.info
    }

    pub fn property_views(&self) -> &Vec<PropView> {
        &self.property_views
    }
}

impl Serialize for LayerView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Layer", 6)?;
        struct_ser.serialize_field("id", &self.info.id())?;
        struct_ser.serialize_field("type", &self.type_str)?;
        struct_ser.serialize_field("name", &self.info.name().view())?;
        struct_ser.serialize_field("description", &self.info.description().view())?;
        struct_ser.serialize_field("data", &self.data)?;
        struct_ser.serialize_field("properties", &self.property_views)?;
        struct_ser.end()
    }
}

#[derive(Clone)]
pub struct LayerInfo {
    id: RandId,
    name: Prop<String>,
    description: Prop<Option<String>>,
}

impl LayerInfo {
    pub fn new(name: &str) -> Self {
        Self {
            id: random(),
            name: StringProp::new(name.to_string()).into_prop(PropertyInfo::unnamed()),
            description: RawPropCore::new(None).into_prop(PropertyInfo::unnamed()),
        }
    }

    pub fn set_description(self, value: &str) -> Self {
        self.description.try_replace_value(Some(value.to_string())).unwrap();
        self
    }

    pub fn id(&self) -> RandId {
        self.id
    }

    pub fn name(&self) -> &Prop<String> {
        &self.name
    }

    pub fn description(&self) -> &Prop<Option<String>> {
        &self.description
    }

    pub fn detach(&mut self) {
        self.id = random();
        self.name = self.name.fork();
        self.description = self.description.fork();
    }
}
