pub mod num;
pub mod cloning;
pub mod locked;

use rand::random;
use serde::{Serialize, Serializer};
use std::ops::{Deref, DerefMut};
use dyn_clone::{clone_trait_object, DynClone};
use erased_serde::serialize_trait_object;
use serde::ser::SerializeStruct;
use tokio::sync::watch;
use crate::pattern_builder::component::{Component};
use crate::pattern_builder::component::data::DisplayPane;
use crate::pattern_builder::RandId;

#[derive(Clone)]
pub struct PropertyInfo {
    id: RandId,
    name: Option<String>,
    description: Option<String>,
    display_pane: DisplayPane,
}

impl PropertyInfo {
    pub fn new(name: &str) -> Self {
        Self {
            id: random(),
            name: Some(name.to_string()),
            description: None,
            display_pane: DisplayPane::Config,
        }
    }
    pub fn unnamed() -> Self {
        Self {
            id: random(),
            name: None,
            description: None,
            display_pane: DisplayPane::Config,
        }
    }
    pub(crate) fn serialize_into<S>(&self, struct_ser: &mut S::SerializeStruct) -> Result<(), S::Error> where S: Serializer  {
        struct_ser.serialize_field("id", &self.get_id())?;
        struct_ser.serialize_field("name", &self.get_name())?;
        struct_ser.serialize_field("description", &self.get_description())?;
        struct_ser.serialize_field("display_pane", &self.display_pane)?;
        Ok(())
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    pub fn display_pane(mut self, val: DisplayPane) -> Self {
        self.display_pane = val;
        self
    }
    pub fn get_id(&self) -> RandId { self.id }
    pub fn get_name(&self) -> &Option<String> { &self.name }
    pub fn get_description(&self) -> &Option<String> { &self.description }
}

impl Serialize for PropertyInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_unit()
    }
}

pub trait Property: DynClone + erased_serde::Serialize + Send + Sync + 'static {
    fn get_info(&self) -> &PropertyInfo;

    fn get_type_id(&self) -> &'static str;

    fn for_each_child_component<'a>(&self, _func: Box<dyn FnMut(&dyn Component) + 'a>) {}

    fn for_each_child_component_mut<'a>(&mut self, _func: Box<dyn FnMut(&mut dyn Component) + 'a>) {}

    fn try_update(&self, serialized_value: String) -> Result<(), String>;

    fn shallow_detach(&mut self);

    fn detach(&mut self) {
        self.shallow_detach();
        self.for_each_child_component_mut(Box::new(|component| component.config_mut().detach()))
    }
}
clone_trait_object!(Property);
serialize_trait_object!(Property);

impl<T: Property + Clone + Serialize + ?Sized> Property for Box<T> {
    fn get_info(&self) -> &PropertyInfo {
        self.as_ref().get_info()
    }

    fn get_type_id(&self) -> &'static str {
        self.as_ref().get_type_id()
    }

    fn for_each_child_component<'a>(&self, func: Box<dyn FnMut(&dyn Component) + 'a>) {
        self.as_ref().for_each_child_component(func)
    }

    fn for_each_child_component_mut<'a>(&mut self, func: Box<dyn FnMut(&mut dyn Component) + 'a>) {
        self.as_mut().for_each_child_component_mut(func)
    }

    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        self.as_ref().try_update(serialized_value)
    }

    fn shallow_detach(&mut self) {
        self.as_mut().shallow_detach()
    }

    fn detach(&mut self) {
        self.as_mut().detach()
    }
}

struct SerializableSender<T>(watch::Sender<T>);

impl<T> SerializableSender<T> {
    fn new(val: T) -> Self { Self(watch::channel(val).0) }
}

impl<T> Deref for SerializableSender<T> {
    type Target = watch::Sender<T>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for SerializableSender<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T: Serialize> Serialize for SerializableSender<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.borrow().serialize(serializer)
    }
}
