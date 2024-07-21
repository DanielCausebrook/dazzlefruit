use std::collections::HashMap;
use crate::pattern_builder::component::layer::LayerView;
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::property::{ErasedPropCore, PropCore, PropRead, PropWrite};

#[derive(Clone)]
pub struct LayerStackPropCore (LayerStack);

impl LayerStackPropCore {
    pub fn new(stack: LayerStack) -> Self {
        Self(stack)
    }
    pub fn fork(&self) -> Self {
        let mut clone_stack = self.0.clone();
        clone_stack.detach();
        Self(clone_stack)
    }
}

impl PropCore for LayerStackPropCore {
    type Value = LayerStack;

    fn read(&self) -> PropRead<Self::Value> {
        PropRead::Ref(&self.0)
    }

    fn write(&mut self) -> PropWrite<Self::Value> {
        PropWrite::Ref(&mut self.0)
    }

    fn fork_dyn(&self) -> Box<dyn PropCore<Value=Self::Value>> {
        Box::new(self.fork())
    }
}

impl ErasedPropCore for LayerStackPropCore {
    fn prop_type_id(&self) -> String {
        "layer-stack".to_string()
    }

    fn child_layer_views(&self) -> Vec<LayerView> {
        self.0.layer_views()
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        todo!("Waiting for library")
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(self.0.layer_views().iter().map(|v| v.info().id() ).collect::<Vec<_>>())
    }

    fn view_data(&self) -> HashMap<String, Box<dyn erased_serde::Serialize + 'static>> {
        [
            ("errors", Vec::<()>::new())
        ].into_iter()
            .map(|(k, v)| (k.to_string(), Box::new(v) as Box<dyn erased_serde::Serialize>))
            .collect()
    }
}