use std::ops::Range;
use std::sync::Arc;

use num_traits::Num;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use tokio::sync::watch;

use crate::pattern_builder::component::property::{Property, PropertyInfo, SerializableSender};

#[derive(Clone, Serialize)]
pub struct NumSlider<T: Num + Copy + Serialize + Send + Sync + 'static> {
    range: Range<T>,
    step: T,
}

impl<T: Num + Copy + Serialize + Send + Sync + 'static> NumSlider<T> {
    pub fn new(range: Range<T>, step: T) -> Self {
        Self { range, step }
    }
}

#[derive(Clone)]
pub struct NumProperty<T: Num + Copy + Serialize + Send + Sync + 'static> {
    info: PropertyInfo,
    value: Arc<SerializableSender<T>>,
    slider: Option<NumSlider<T>>,
}

impl<T: Num + Copy + Serialize + Send + Sync + 'static> NumProperty<T> {
    pub fn new(val: T, info: PropertyInfo) -> Self {
        Self {
            info,
            value: Arc::new(SerializableSender::new(val)),
            slider: None,
        }
    }
    fn serialize_with<S>(&self, type_id: &str, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Property", 7)?;
        self.info.serialize_into::<S>(&mut struct_ser)?;
        struct_ser.serialize_field("property_type", type_id)?;
        struct_ser.serialize_field("value", &self.value)?;
        struct_ser.serialize_field("slider", &self.slider)?;
        struct_ser.end()
    }
    pub fn set_info(mut self, info: PropertyInfo) -> Self {
        self.info = info;
        self
    }
    pub fn set_slider(mut self, slider: Option<NumSlider<T>>) -> Self {
        self.slider = slider;
        self
    }
    pub fn get_slider(&self) -> &Option<NumSlider<T>> {
        &self.slider
    }
    pub fn get(&self) -> T {
        *self.value.borrow()
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

impl<T: Num + Copy + Serialize + Send + Sync + 'static> Property for NumProperty<T> {
    fn info(&self) -> &PropertyInfo { &self.info }
    fn type_id(&self) -> &'static str { "num" }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        self.replace(T::from_str_radix(&serialized_value, 10).map_err(|_| "Could not parse number.")?);
        Ok(())
    }
    fn shallow_detach(&mut self) { self.shallow_detach() }
}

impl<T: Num + Copy + Serialize + Send + Sync + 'static> Serialize for NumProperty<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.serialize_with("num", serializer)
    }
}

impl<T: Num + Copy + Serialize + Send + Sync + 'static> From<T> for NumProperty<T> {
    fn from(value: T) -> Self {
        NumProperty::new(value, PropertyInfo::unnamed())
    }
}

#[derive(Clone)]
pub struct OptionNumProperty<T: Num + Copy + Serialize + Send + Sync + 'static> {
    info: PropertyInfo,
    value: Arc<SerializableSender<Option<T>>>,
    range: Range<T>,
    step: T,
}

impl<T: Num + Copy + Serialize + Send + Sync + 'static> OptionNumProperty<T> {
    pub fn new(val: Option<T>, range: Range<T>, step: T, info: PropertyInfo) -> Self {
        Self {
            info,
            value: Arc::new(SerializableSender::new(val)),
            range,
            step,
        }
    }
    fn serialize_with<S>(&self, type_id: &str, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Property", 8)?;
        self.info.serialize_into::<S>(&mut struct_ser)?;
        struct_ser.serialize_field("property_type", type_id)?;
        struct_ser.serialize_field("value", &self.value)?;
        struct_ser.serialize_field("range", &self.range)?;
        struct_ser.serialize_field("step", &self.step)?;
        struct_ser.end()
    }
    pub fn set_info(mut self, info: PropertyInfo) -> Self {
        self.info = info;
        self
    }
    pub fn set_range(mut self, range: Range<T>, step: T) -> Self {
        self.range = range;
        self.step = step;
        self
    }
    pub fn get(&self) -> Option<T> { self.value.borrow().clone() }
    pub fn replace(&self, val: Option<T>) -> Option<T> {
        self.value.send_replace(val)
    }
    pub fn subscribe(&self) -> watch::Receiver<Option<T>> {
        self.value.subscribe()
    }
    pub fn shallow_detach(&mut self) {
        self.value = Arc::new(SerializableSender::new(self.get()));
    }
}

impl<T: Num + Copy + Serialize + Send + Sync + 'static> Property for OptionNumProperty<T> {
    fn info(&self) -> &PropertyInfo { &self.info }
    fn type_id(&self) -> &'static str { "optionNum" }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        todo!()
    }
    fn shallow_detach(&mut self) { self.shallow_detach() }
}

impl<T: Num + Copy + Serialize + Send + Sync + 'static> Serialize for OptionNumProperty<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.serialize_with("optionNum", serializer)
    }
}
