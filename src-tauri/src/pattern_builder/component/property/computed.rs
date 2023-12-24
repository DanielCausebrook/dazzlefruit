use erased_serde::Serialize;
use crate::pattern_builder::component::property::{PropCore, ErasedPropCore, PropRead, PropWrite};


pub struct ComputedPropCore<F, T>(F) where F: Fn() -> T + Send + Sync + Clone + 'static, T: Send + Sync + 'static;

impl<F, T> ComputedPropCore<F, T> where F: Fn() -> T + Send + Sync + Clone + 'static, T: Send + Sync + 'static {
    pub fn new(func: F) -> Self {
        Self(func)
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }
}

impl<F, T> Clone for ComputedPropCore<F, T> where F: Fn() -> T + Send + Sync + Clone + 'static, T: Send + Sync + 'static {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<F, T> PropCore for ComputedPropCore<F, T> where F: Fn() -> T + Send + Sync + Clone + 'static, T: Send + Sync + 'static {
    type Value = T;

    fn read(&self) -> PropRead<Self::Value> {
        PropRead::Value((self.0)())
    }

    fn write(&mut self) -> PropWrite<Self::Value> {
        PropWrite::Value((self.0)())
    }

    fn fork_dyn(&self) -> Box<dyn PropCore<Value=Self::Value>> {
        Box::new(self.fork())
    }
}

impl<F, T> ErasedPropCore for ComputedPropCore<F, T> where F: Fn() -> T + Send + Sync + Clone, T: Send + Sync {
    fn prop_type_id(&self) -> String {
        "computed".to_string()
    }

    fn try_update(&mut self, _str: &str) -> Result<(), String> {
        Err("Cannot update a computed property.".to_string())
    }

    fn value_serialize(&self) -> Box<dyn Serialize  + '_> {
        Box::new(())
    }
}