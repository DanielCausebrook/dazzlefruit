use palette::{LinSrgba, Srgb};
use serde::{Serialize, Serializer};
use std::sync::Arc;
use tokio::sync::watch;
use palette::rgb::Rgb;
use serde::ser::SerializeStruct;
use std::str::FromStr;
use crate::pattern_builder::component::data::BlendMode;
use crate::pattern_builder::component::property::{Property, PropertyInfo, SerializableSender};

#[derive(Clone)]
pub struct CloningProperty<T: Clone> {
    info: PropertyInfo,
    value: Arc<SerializableSender<T>>,
}

impl<T: Clone> CloningProperty<T> {
    pub fn new(val: T, info: PropertyInfo) -> Self {
        Self {
            info,
            value: Arc::new(SerializableSender::new(val)),
        }
    }
    pub fn set_info(mut self, info: PropertyInfo) -> Self {
        self.info = info;
        self
    }
    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }
    pub fn replace(&self, val: T) -> T {
        self.value.send_replace(val)
    }
    pub fn subscribe(&self) -> watch::Receiver<T> {
        self.value.subscribe()
    }
    pub fn shallow_detach(&mut self) {
        self.value = Arc::new(SerializableSender::new(self.get()));
    }
}

impl<T: Clone + Serialize> CloningProperty<T> {
    fn serialize_with<S>(&self, type_id: &str, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Property", 6)?;
        self.info.serialize_into::<S>(&mut struct_ser)?;
        struct_ser.serialize_field("property_type", type_id)?;
        struct_ser.serialize_field("value", &self.value)?;
        struct_ser.end()
    }
}

pub(crate) type StringProperty = CloningProperty<String>;

impl Property for StringProperty {
    fn info(&self) -> &PropertyInfo { &self.info }
    fn type_id(&self) -> &'static str { "string" }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        self.replace(serialized_value);
        Ok(())
    }
    fn shallow_detach(&mut self) { self.shallow_detach() }
}

impl Serialize for StringProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.serialize_with("string", serializer)
    }
}

pub(crate) type OptionStringProperty = CloningProperty<Option<String>>;

impl Property for OptionStringProperty {
    fn info(&self) -> &PropertyInfo { &self.info }
    fn type_id(&self) -> &'static str { "optionString" }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        todo!()
    }
    fn shallow_detach(&mut self) { self.shallow_detach() }
}

impl Serialize for OptionStringProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.serialize_with("optionString", serializer)
    }
}

pub(crate) type BoolProperty = CloningProperty<bool>;

impl Property for BoolProperty {
    fn info(&self) -> &PropertyInfo { &self.info }
    fn type_id(&self) -> &'static str { "bool" }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        self.replace(bool::from_str(&serialized_value).map_err(|e| e.to_string())?);
        Ok(())
    }
    fn shallow_detach(&mut self) { self.shallow_detach() }
}

impl Serialize for BoolProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.serialize_with("bool", serializer)
    }
}

pub type BlendModeProperty = CloningProperty<BlendMode>;

impl Property for BlendModeProperty {
    fn info(&self) -> &PropertyInfo { &self.info }
    fn type_id(&self) -> &'static str { "blendMode" }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        todo!()
    }
    fn shallow_detach(&mut self) { self.shallow_detach() }
}

impl Default for BlendModeProperty {
    fn default() -> Self {
        BlendModeProperty::new(BlendMode::Normal, PropertyInfo::new("Blend Mode"))
    }
}

impl Serialize for BlendModeProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.serialize_with("blendMode", serializer)
    }
}

impl From<BlendMode> for BlendModeProperty {
    fn from(value: BlendMode) -> Self {
        Self::new(value, PropertyInfo::unnamed())
    }
}

pub type ColorProperty = CloningProperty<LinSrgba>;

impl Property for ColorProperty {
    fn info(&self) -> &PropertyInfo { &self.info }
    fn type_id(&self) -> &'static str { "color" }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        let color = Rgb::from_str(serialized_value.as_str())
            .map_err(|e| e.to_string())?;
        self.replace(color.into());
        Ok(())
    }
    fn shallow_detach(&mut self) { self.shallow_detach() }
}

impl Serialize for ColorProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Property", 5)?;
        self.info.serialize_into::<S>(&mut struct_ser)?;
        struct_ser.serialize_field("property_type", "color")?;
        struct_ser.serialize_field("value", &(Srgb::<u8>::from_linear(self.value.borrow().clone().premultiply().into())).into_components())?;
        struct_ser.end()
    }
}
