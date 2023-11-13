use dyn_clone::{clone_trait_object, DynClone};
use rand::random;
use property::cloning::{OptionStringProperty, StringProperty};
use crate::pattern_builder::component::data::RandId;
use crate::pattern_builder::component::property::{Property, PropertyInfo};

pub mod view_serde;
pub mod property;
pub mod basic_config;
pub mod data;
pub mod texture;
pub mod filter;
pub mod texture_generator;
mod macros;
pub mod shared_component;

pub trait Component: Send + Sync + DynClone + 'static {
    fn config(&self) -> &dyn ComponentConfig;
    fn config_mut(&mut self) -> &mut dyn ComponentConfig;
    fn component_type(&self) -> &'static str;
}
clone_trait_object!(Component);

impl<T> Component for Box<T> where T: Component + Clone + ?Sized {
    fn config(&self) -> &dyn ComponentConfig {
        self.as_ref().config()
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        self.as_mut().config_mut()
    }

    fn component_type(&self) -> &'static str {
        self.as_ref().component_type()
    }
}

pub trait ComponentConfig: Send + Sync + DynClone + 'static {
    fn info(&self) -> &ComponentInfo;
    fn info_mut(&mut self) -> &mut ComponentInfo;
    fn properties(&self) -> Vec<&dyn Property>;
    fn properties_mut(&mut self) -> Vec<&mut dyn Property>;
    fn detach(&mut self) {
        self.info_mut().detach();
        for property in self.properties_mut() {
            property.detach();
        }
    }
}
impl<T> ComponentConfig for Box<T> where T: ComponentConfig + Clone + ?Sized {
    fn info(&self) -> &ComponentInfo { self.as_ref().info() }
    fn info_mut(&mut self) -> &mut ComponentInfo { self.as_mut().info_mut() }
    fn properties(&self) -> Vec<&dyn Property> { self.as_ref().properties() }
    fn properties_mut(&mut self) -> Vec<&mut dyn Property> { self.as_mut().properties_mut() }
    fn detach(&mut self) { self.as_mut().detach() }
}

#[derive(Clone)]
pub struct ComponentInfo {
    id: RandId,
    name: StringProperty,
    description: OptionStringProperty,
}

impl ComponentInfo {
    pub fn new(name: &str) -> Self {
        Self {
            id: random(),
            name: StringProperty::new(name.to_string(), PropertyInfo::unnamed()),
            description: OptionStringProperty::new(None, PropertyInfo::unnamed()),
        }
    }

    pub fn description(self, value: Option<&str>) -> Self {
        self.description.replace(value.map(|s| s.to_string()));
        self
    }

    pub fn get_id(&self) -> RandId {
        self.id
    }

    pub fn get_name_prop(&self) -> &StringProperty {
        &self.name
    }

    pub fn get_description_prop(&self) -> &OptionStringProperty {
        &self.description
    }

    pub fn detach(&mut self) {
        self.id = random();
        self.name.shallow_detach();
        self.description.shallow_detach();
    }
}
