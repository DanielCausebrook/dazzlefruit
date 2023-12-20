pub mod raw;
pub mod num;
pub mod layer;
pub mod computed;
pub mod color;
pub mod string;
pub mod num_vec;
pub mod layer_stack;

use std::collections::HashMap;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use dyn_clone::{clone_trait_object, DynClone};
use parking_lot::RwLock;
use rand::random;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use crate::pattern_builder::component::layer::LayerView;
use crate::pattern_builder::component::data::{DisplayPane, RandId};
use crate::pattern_builder::component::property::computed::ComputedPropCore;

pub struct Prop<T> where T: 'static {
    info: PropertyInfo,
    core: Arc<RwLock<Box<dyn PropCore<Value=T>>>>,
}

impl<T> Prop<T> where T: 'static {
    pub fn new(core: impl PropCore<Value=T>, info: PropertyInfo) -> Self {
        Self {
            info,
            core: Arc::new(RwLock::new(Box::new(core))),
        }
    }
    pub fn info(&self) -> &PropertyInfo {
        &self.info
    }

    pub fn view(&self) -> PropView {
        PropView(Box::new(self.clone()))
    }

    pub fn read(&self) -> PropReadGuard<T> {
        PropReadGuard::new(&self.core)
    }

    pub fn write(&self) -> PropWriteGuard<T> {
        PropWriteGuard::new(&self.core)
    }

    pub fn replace_core(&self, core: impl PropCore<Value=T>) -> Box<dyn PropCore<Value=T>> {
        mem::replace(&mut *self.core.write(), Box::new(core))
    }

    pub fn try_replace_value(&self, value: T) -> Result<T, String> {
        self.core.write().try_replace(value)
    }

    pub fn fork(&self) -> Self {
        Self {
            info: self.info.fork(),
            core: Arc::new(RwLock::new(self.core.read().fork_dyn())),
        }
    }

    pub fn map_core<'a, F, U>(&'a self, func: F) -> ComputedPropCore<impl Fn() -> U + Clone + 'static, U> where F: Fn(&T) -> U + Clone + Send + Sync + 'static, U: Send + Sync {
        let clone = (*self).clone();
        ComputedPropCore::new(move || {
            func(&*clone.read())
        })
    }
}
impl<T> Clone for Prop<T> {
    fn clone(&self) -> Self {
        Self {
            info: self.info.clone(),
            core: self.core.clone(),
        }
    }
}


// Prop read/write guards

pub enum PropRead<'a, T> where T: 'static {
    Value(T),
    Ref(&'a T),
}

pub struct PropReadGuard<'a, T> where T: 'static {
    lock: &'a RwLock<Box<dyn PropCore<Value=T>>>,
    value: PropRead<'a, T>,
}

impl<'a, T> PropReadGuard<'a, T> where T: 'static {
    pub fn new(lock: &'a RwLock<Box<dyn PropCore<Value=T>>>) -> Self {
        mem::forget(lock.read());
        Self {
            value: unsafe { & *lock.data_ptr() }.read(),
            lock,
        }
    }
}

impl<'a, T> Drop for PropReadGuard<'a, T> where T: 'static {
    fn drop(&mut self) {
        unsafe { self.lock.force_unlock_read(); }
    }
}

impl<'a, T> Deref for PropReadGuard<'a, T> where T: 'static {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.value {
            PropRead::Value(ref val) => val,
            PropRead::Ref(ref val) => *val,
        }
    }
}

pub enum PropWrite<'a, T> where T: 'static {
    Value(T),
    Ref(&'a mut T),
}


pub struct PropWriteGuard<'a, T> where T: 'static {
    lock: &'a RwLock<Box<dyn PropCore<Value=T>>>,
    value: PropWrite<'a, T>,
}

impl<'a, T> PropWriteGuard<'a, T> where T: 'static {
    pub fn new(lock: &'a RwLock<Box<dyn PropCore<Value=T>>>) -> Self {
        mem::forget(lock.write());
        Self {
            value: unsafe { &mut *lock.data_ptr() }.write(),
            lock,
        }
    }
}

impl<'a, T> Drop for PropWriteGuard<'a, T> where T: 'static {
    fn drop(&mut self) {
        unsafe { self.lock.force_unlock_write(); }
    }
}

impl<'a, T> Deref for PropWriteGuard<'a, T> where T: 'static {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.value {
            PropWrite::Value(ref val) => val,
            PropWrite::Ref(ref val) => *val,
        }
    }
}

impl<'a, T> DerefMut for PropWriteGuard<'a, T> where T: 'static {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.value {
            PropWrite::Value(ref mut val) => val,
            PropWrite::Ref(ref mut val) => *val,
        }
    }
}


// ErasedPropCore guards

struct ErasedPropCoreReadGuard<'a> {
    erased_prop: &'a dyn ErasedProp,
    core_view: &'a dyn ErasedPropCore,
}

impl<'a> ErasedPropCoreReadGuard<'a> {
    fn new(prop: &'a impl ErasedProp) -> Self {
        Self {
            core_view: unsafe { prop.read_core_as_view_forgetting_guard() },
            erased_prop: prop,
        }
    }
}

impl Drop for ErasedPropCoreReadGuard<'_> {
    fn drop(&mut self) {
        unsafe {self.erased_prop.force_unlock_read()};
    }
}

impl<'a> Deref for ErasedPropCoreReadGuard<'a> {
    type Target = dyn ErasedPropCore + 'a;

    fn deref(&self) -> &Self::Target {
        self.core_view
    }
}


struct ErasedPropCoreWriteGuard<'a> {
    erased_prop: &'a dyn ErasedProp,
    core_view: &'a mut dyn ErasedPropCore,
}

impl<'a> ErasedPropCoreWriteGuard<'a> {
    fn new(prop: &'a impl ErasedProp) -> Self {
        Self {
            core_view: unsafe { prop.write_core_as_view_forgetting_guard() },
            erased_prop: prop,
        }
    }
}

impl Drop for ErasedPropCoreWriteGuard<'_> {
    fn drop(&mut self) {
        unsafe {self.erased_prop.force_unlock_write()};
    }
}

impl<'a> Deref for ErasedPropCoreWriteGuard<'a> {
    type Target = dyn ErasedPropCore + 'a;

    fn deref(&self) -> &Self::Target {
        self.core_view
    }
}

impl DerefMut for ErasedPropCoreWriteGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.core_view
    }
}


// Erased prop for view

trait ErasedProp: Send + Sync + DynClone {
    fn info(&self) -> &PropertyInfo;
    fn read_core(&self) -> ErasedPropCoreReadGuard<'_>;
    fn write_core(&self) -> ErasedPropCoreWriteGuard<'_>;
    unsafe fn read_core_as_view_forgetting_guard(&self) -> &dyn ErasedPropCore;
    unsafe fn write_core_as_view_forgetting_guard(&self) -> &mut dyn ErasedPropCore;
    unsafe fn force_unlock_read(&self);
    unsafe fn force_unlock_write(&self);
}
clone_trait_object!(ErasedProp);

impl<T> ErasedProp for Prop<T> {
    fn info(&self) -> &PropertyInfo {
        self.info()
    }

    fn read_core(&self) -> ErasedPropCoreReadGuard<'_> {
        ErasedPropCoreReadGuard::new(self)
    }

    fn write_core(&self) -> ErasedPropCoreWriteGuard<'_> {
        ErasedPropCoreWriteGuard::new(self)
    }

    unsafe fn read_core_as_view_forgetting_guard(&self) -> &dyn ErasedPropCore {
        mem::forget(self.core.read());
        (*self.core.data_ptr()).as_ref()
    }

    unsafe fn write_core_as_view_forgetting_guard(&self) -> &mut dyn ErasedPropCore {
        mem::forget(self.core.write());
        (*self.core.data_ptr()).as_mut()
    }

    unsafe fn force_unlock_read(&self) {
        self.core.force_unlock_read();
    }

    unsafe fn force_unlock_write(&self) {
        self.core.force_unlock_write();
    }
}


// Prop view

#[derive(Clone)]
pub struct PropView (Box<dyn ErasedProp>);

impl PropView {
    pub fn info(&self) -> &PropertyInfo {
        &self.0.info()
    }

    pub fn child_layer_views(&self) -> Vec<LayerView> {
        self.0.read_core().child_layer_views()
    }

    pub fn try_update(&mut self, str: &str) -> Result<(), String> {
        self.0.write_core().try_update(str)
    }
}

impl Serialize for PropView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Prop", 7)?;
        struct_ser.serialize_field("id", &self.info().id())?;
        struct_ser.serialize_field("type", &self.0.read_core().prop_type_id())?;
        struct_ser.serialize_field("name", &self.info().name())?;
        struct_ser.serialize_field("description", &self.info().description())?;
        struct_ser.serialize_field("display_pane", &self.info().display_pane())?;
        struct_ser.serialize_field("value", &self.0.read_core().value_serialize())?;
        struct_ser.serialize_field("data", &self.0.read_core().view_data())?;
        struct_ser.end()
    }
}


// Core traits

pub trait PropCore: ErasedPropCore + DynClone + Send + Sync + 'static {
    type Value;
    fn read(&self) -> PropRead<Self::Value>;
    fn write(&mut self) -> PropWrite<Self::Value>;
    fn try_replace(&mut self, value: Self::Value) -> Result<Self::Value, String> where Self::Value: Sized;
    fn fork_dyn(&self) -> Box<dyn PropCore<Value=Self::Value>>;
    fn into_prop(self, info: PropertyInfo) -> Prop<Self::Value> where Self: Sized {
        Prop::new(self, info)
    }
}
clone_trait_object!(<T> PropCore<Value=T>);

pub trait ErasedPropCore: Send + Sync + DynClone {
    fn prop_type_id(&self) -> String;
    fn view_data(&self) -> HashMap<String, Box<dyn erased_serde::Serialize + 'static>> {
        HashMap::new()
    }
    fn child_layer_views(&self) -> Vec<LayerView> { vec![] }
    fn try_update(&mut self, str: &str) -> Result<(), String>;
    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_>;
}
clone_trait_object!(ErasedPropCore);


// PropertyInfo

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
    pub fn set_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    pub fn set_display_pane(mut self, val: DisplayPane) -> Self {
        self.display_pane = val;
        self
    }
    pub fn id(&self) -> RandId { self.id }
    pub fn name(&self) -> &Option<String> { &self.name }
    pub fn description(&self) -> &Option<String> { &self.description }
    pub fn display_pane(&self) -> DisplayPane {
        self.display_pane
    }
    pub fn fork(&self) -> Self {
        let mut fork = self.clone();
        fork.id = random();
        fork
    }
}
