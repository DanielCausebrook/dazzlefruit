use crate::pattern_builder::component::property::{PropCore, ErasedPropCore, PropRead, PropWrite};

#[derive(Clone)]
pub struct RawPropCore<T: Clone + Send + Sync> (T);

impl<T> RawPropCore<T> where T: Clone + Send + Sync {
    pub fn new(value: T) -> Self {
        Self (value)
    }

    pub fn fork(&self) -> Self {
        Self (self.0.clone())
    }
}

impl<T> PropCore for RawPropCore<T> where T: Clone + Send + Sync + 'static {
    type Value = T;

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

impl<T> ErasedPropCore for RawPropCore<T>  where T: Clone + Send + Sync {
    fn prop_type_id(&self) -> String {
        "raw".to_string()
    }

    fn try_update(&mut self, _str: &str) -> Result<(), String> {
        Err("Cannot update a raw property".to_string())
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(())
    }
}

impl<T> From<T> for RawPropCore<T> where T: Clone + Send + Sync {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

