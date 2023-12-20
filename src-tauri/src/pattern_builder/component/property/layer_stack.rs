use std::collections::HashMap;
use std::mem;
use crate::pattern_builder::component::layer::LayerView;
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::property::{ErasedPropCore, PropCore, PropRead, PropWrite};

pub struct LayerStackPropCore<I, O> (LayerStack<I, O>) where I: 'static, O: 'static;

impl<I, O> LayerStackPropCore<I, O> where I: 'static, O: 'static {
    pub fn new(stack: LayerStack<I, O>) -> Self {
        Self(stack)
    }
    pub fn fork(&self) -> Self {
        let mut clone_stack = self.0.clone();
        clone_stack.detach();
        Self(clone_stack)
    }
}

impl<I, O> Clone for LayerStackPropCore<I, O> where I: 'static, O: 'static {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<I, O> PropCore for LayerStackPropCore<I, O> where I: 'static, O: 'static {
    type Value = LayerStack<I, O>;

    fn read(&self) -> PropRead<Self::Value> {
        PropRead::Ref(&self.0)
    }

    fn write(&mut self) -> PropWrite<Self::Value> {
        PropWrite::Ref(&mut self.0)
    }

    fn try_replace(&mut self, value: Self::Value) -> Result<Self::Value, String> where Self::Value: Sized {
        Ok(mem::replace(&mut self.0, value))
    }

    fn fork_dyn(&self) -> Box<dyn PropCore<Value=Self::Value>> {
        Box::new(self.fork())
    }
}

impl<I, O> ErasedPropCore for LayerStackPropCore<I, O> where I: 'static, O: 'static {
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
            ("errors", self.0.type_errors())
        ].into_iter()
            .map(|(k, v)| (k.to_string(), Box::new(v) as Box<dyn erased_serde::Serialize>))
            .collect()
    }
}