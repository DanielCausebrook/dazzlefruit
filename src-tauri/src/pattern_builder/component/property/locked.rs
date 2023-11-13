use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Serializer};
use tokio::sync::watch;
use serde::ser::SerializeStruct;
use crate::pattern_builder::component::{Component};
use crate::pattern_builder::component::shared_component::SharedComponent;
use crate::pattern_builder::component::property::{Property, PropertyInfo, SerializableSender};
use crate::pattern_builder::component::texture::Texture;
use crate::pattern_builder::component::texture_generator::TextureGenerator;
use crate::watch_guard::{RWLockWatchReadGuard, RWLockWatchSender, RWLockWatchWriteGuard};

#[derive(Clone)]
pub struct LockedProperty<T: Clone> {
    info: PropertyInfo,
    value: Arc<SerializableSender<RwLock<T>>>,
}

impl<T: Clone> LockedProperty<T> {
    pub fn new(val: T, info: PropertyInfo) -> Self {
        Self {
            info,
            value: Arc::new(SerializableSender::new(RwLock::new(val))),
        }
    }
    pub fn info(&self) -> &PropertyInfo {
        &self.info
    }

    pub fn set_info(mut self, info: PropertyInfo) -> Self {
        self.info = info;
        self
    }

    pub fn read(&self) -> RWLockWatchReadGuard<'_, T> {
        self.value.read()
    }

    pub fn write(&self) -> RWLockWatchWriteGuard<'_, T> {
        self.value.write()
    }

    pub fn replace(&self, val: T) -> T {
        self.value.send_replace(RwLock::new(val)).into_inner()
    }

    pub fn subscribe(&self) -> watch::Receiver<RwLock<T>> {
        self.value.subscribe()
    }

    pub fn shallow_detach(&mut self) {
        let clone = self.read().clone();
        self.value = Arc::new(SerializableSender::new(RwLock::new(clone)));
    }
}

pub type ComponentProperty<T> = LockedProperty<T>;

impl<T: Component + Clone> Property for ComponentProperty<T> {
    fn info(&self) -> &PropertyInfo { &self.info }
    fn type_id(&self) -> &'static str { "component" }
    fn for_each_child_component<'a>(&self, mut func: Box<dyn FnMut(&dyn Component) + 'a>) {
        func(&*self.read());
    }
    fn for_each_child_component_mut<'a>(&mut self, mut func: Box<dyn FnMut(&mut dyn Component) + 'a>) {
        func(&mut *self.write())
    }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        todo!()
    }
    fn shallow_detach(&mut self) { self.shallow_detach() }
}

impl<T: Component + Clone> Serialize for ComponentProperty<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Property", 6)?;
        self.info.serialize_into::<S>(&mut struct_ser)?;
        struct_ser.serialize_field("property_type", self.type_id())?;
        struct_ser.serialize_field("value", &self.read().config().info().get_id())?;
        struct_ser.end()
    }
}

pub type ComponentVecProperty<T> = LockedProperty<Vec<T>>;

impl<T: Component + Clone> Property for ComponentVecProperty<T> {
    fn info(&self) -> &PropertyInfo { &self.info }
    fn type_id(&self) -> &'static str { "componentVec" }
    fn for_each_child_component<'a>(&self, mut func: Box<dyn FnMut(&dyn Component) + 'a>) {
        for component in self.read().iter() {
            func(& *component);
        }
    }
    fn for_each_child_component_mut<'a>(&mut self, mut func: Box<dyn FnMut(&mut dyn Component) + 'a>) {
        for component in self.write().iter_mut() {
            func(&mut *component);
        }
    }
    fn try_update(&self, serialized_value: String) -> Result<(), String> {
        todo!()
    }

    fn shallow_detach(&mut self) { self.shallow_detach(); }
}

impl<T: Component + Clone> Serialize for ComponentVecProperty<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Property", 6)?;
        self.info.serialize_into::<S>(&mut struct_ser)?;
        struct_ser.serialize_field("property_type", self.type_id())?;
        struct_ser.serialize_field("value", &self.read().iter()
            .map(|texture| texture.config().info().get_id())
            .collect::<Vec<_>>()
        )?;
        struct_ser.end()
    }
}
