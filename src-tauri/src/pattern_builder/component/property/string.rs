use std::mem;
use erased_serde::Serialize;
use crate::pattern_builder::component::property::{PropCore, ErasedPropCore, PropRead, PropWrite};

#[derive(Clone)]
pub struct StringProp(String);

impl StringProp {
    pub fn new(value: String) -> Self {
        Self (value)
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }
}

impl PropCore for StringProp {
    type Value = String;

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

impl ErasedPropCore for StringProp {
    fn prop_type_id(&self) -> String {
        "string".to_string()
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        self.try_replace(str.to_string()).map(|_| ())
    }

    fn value_serialize(&self) -> Box<dyn Serialize + '_> {
        Box::new(&self.0)
    }
}