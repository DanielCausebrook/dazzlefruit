use std::collections::HashMap;
use serde::ser::{SerializeStruct};
use serde::{Serialize, Serializer};
use crate::pattern_builder::component::{Layer, LayerInfo};
use crate::pattern_builder::component::data::RandId;
use crate::pattern_builder::component::property::PropView;

#[derive(Serialize)]
pub struct PatternBuilderViewData {
    root_id: RandId,
    components: HashMap<RandId, ComponentViewData>,
}

impl PatternBuilderViewData {
    pub fn new(root_component: &dyn Layer) -> Self {
        let mut configs = vec![];
        let mut current_configs = vec![ComponentViewData::new(root_component)];
        while !current_configs.is_empty() {
            let mut next_configs = vec![];
            for current_config in current_configs {
                for property in current_config.property_views() {
                    property.for_each_child_component(|component| {
                        next_configs.push(ComponentViewData::new(component));
                    });
                }
                configs.push(current_config);
            }
            current_configs = next_configs;
        }
        Self {
            root_id: root_component.info().id(),
            components: configs.into_iter()
                .map(|c| (c.info().id(), c))
                .collect(),
        }
    }
    pub fn generate_property_map(&self) -> HashMap<RandId, PropView> {
        self.components.values()
            .flat_map(|layer_config| layer_config.property_views())
            .map(|prop| (prop.info().id(), prop.clone()))
            .collect::<HashMap<RandId, PropView>>()
    }
}

struct ComponentViewData {
    type_str: String,
    info: LayerInfo,
    property_views: Vec<PropView>,
    data: HashMap<String, Box<dyn erased_serde::Serialize + 'static>>,
}

impl ComponentViewData {
    fn new(component: &dyn Layer,) -> Self {
        Self {
            type_str: component.type_str(),
            info: component.info().clone(),
            property_views: component.view_properties(),
            data: component.view_data(),
        }
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }

    fn property_views(&self) -> &Vec<PropView> {
        &self.property_views
    }
}

impl Serialize for ComponentViewData {
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