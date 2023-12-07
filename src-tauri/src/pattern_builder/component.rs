use std::collections::HashMap;
use dyn_clone::{clone_trait_object, DynClone};
use rand::random;
use crate::pattern_builder::component::data::RandId;
use crate::pattern_builder::component::property::{PropertyInfo};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::string::StringProp;

pub mod view_serde;
pub mod data;
pub mod texture;
pub mod filter;
pub mod texture_generator;
mod macros;
// pub mod shared_component;
pub mod property;
pub trait Component: Send + Sync + 'static {
    fn view_properties(&self) -> Vec<PropView>;
    fn detach(&mut self);
}

impl<T> Component for Box<T> where T: Component + ?Sized {
    fn view_properties(&self) -> Vec<PropView> {
        self.as_ref().view_properties()
    }

    fn detach(&mut self) {
        self.as_mut().detach()
    }
}

pub trait Layer: Component + Send + Sync + DynClone + 'static {
    fn type_str(&self) -> String;
    fn info(&self) -> &LayerInfo;
    fn view_data(&self) -> HashMap<String, Box<dyn erased_serde::Serialize + 'static>> {
        HashMap::new()
    }
}
clone_trait_object!(Layer);

impl<T> Layer for Box<T> where T: Layer + Clone + ?Sized {
    fn type_str(&self) -> String {
        self.as_ref().type_str()
    }
    fn info(&self) -> &LayerInfo { self.as_ref().info() }
    fn view_data(&self) -> HashMap<String, Box<dyn erased_serde::Serialize + 'static>> {
        self.as_ref().view_data()
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
