use erased_serde::Serialize;
use crate::pattern_builder::component::property::{PropCore, ErasedPropCore, PropRead, PropWrite};

#[derive(Clone)]
pub struct StringPropCore(String);

impl StringPropCore {
    pub fn new(value: String) -> Self {
        Self (value)
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }
}

impl PropCore for StringPropCore {
    type Value = String;

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

impl ErasedPropCore for StringPropCore {
    fn prop_type_id(&self) -> String {
        "string".to_string()
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        self.0 = serde_json::from_str(str)
            .map_err(|err| err.to_string())?;
        Ok(())
    }

    fn value_serialize(&self) -> Box<dyn Serialize + '_> {
        Box::new(&self.0)
    }
}

#[derive(Clone)]
pub struct OptionStringPropCore(Option<String>);

impl OptionStringPropCore {
    pub fn new(value: Option<String>) -> Self {
        Self (value)
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }
}

impl PropCore for OptionStringPropCore {
    type Value = Option<String>;

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

impl ErasedPropCore for OptionStringPropCore {
    fn prop_type_id(&self) -> String {
        "option-string".to_string()
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        self.0 = serde_json::from_str(str)
            .map_err(|err| err.to_string())?;
        Ok(())
    }

    fn value_serialize(&self) -> Box<dyn Serialize + '_> {
        Box::new(&self.0)
    }
}
