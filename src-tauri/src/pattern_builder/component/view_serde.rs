use std::collections::HashMap;
use dyn_clone::clone_box;
use serde::ser::{SerializeStruct};
use serde::{Serialize, Serializer};
use crate::pattern_builder::component::{Component, ComponentConfig};
use crate::pattern_builder::component::data::RandId;
use crate::pattern_builder::component::property::Property;

#[derive(Serialize)]
pub struct PatternBuilderViewData {
    root_id: RandId,
    layer_configs: HashMap<RandId, ComponentConfigViewData>,
}

impl PatternBuilderViewData {
    pub fn new(root_component: &dyn Component) -> Self {
        let mut configs = vec![];
        let mut current_configs = vec![ComponentConfigViewData::new(
            root_component.component_type(),
            clone_box(root_component.config())
        )];
        while !current_configs.is_empty() {
            let mut next_configs = vec![];
            for current_config in current_configs {
                for property in current_config.get_component_config().properties() {
                    property.for_each_child_component(Box::new(|component| {
                        next_configs.push(ComponentConfigViewData::new(
                            component.component_type(),
                            clone_box(component.config())
                        ));
                    }));
                }
                configs.push(current_config);
            }
            current_configs = next_configs;
        }
        Self {
            root_id: root_component.config().info().get_id(),
            layer_configs: configs.into_iter()
                .map(|c| (c.get_component_config().info().get_id(), c))
                .collect(),
        }
    }
    pub fn generate_property_map(&self) -> HashMap<RandId, Box<dyn Property>> {
        self.layer_configs.values()
            .flat_map(|layer_config| layer_config.get_component_config().properties())
            .map(|prop| (prop.get_info().get_id(), clone_box(prop)))
            .collect::<HashMap<RandId, Box<dyn Property>>>()
    }
}

struct ComponentConfigViewData {
    component_type: &'static str,
    component_config: Box<dyn ComponentConfig + 'static>,
}

impl ComponentConfigViewData {
    fn new(component_type: &'static str, component_config: Box<dyn ComponentConfig + 'static>) -> Self {
        Self { component_type, component_config }
    }
    fn get_component_config(&self) -> &Box<dyn ComponentConfig + 'static> {
        &self.component_config
    }
}

impl Serialize for ComponentConfigViewData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let config = &self.get_component_config();
        let mut struct_ser = serializer.serialize_struct("Layer", 5)?;
        struct_ser.serialize_field("id", &config.info().get_id())?;
        struct_ser.serialize_field("layer_type", self.component_type)?;
        struct_ser.serialize_field("name", &config.info().get_name_prop())?;
        struct_ser.serialize_field("description", &config.info().get_description_prop())?;
        struct_ser.serialize_field("properties", &config.properties())?;
        struct_ser.end()
    }
}