use std::collections::HashMap;
use std::mem;
use std::ops::Range;
use num_traits::Num;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use crate::pattern_builder::component::property::{PropCore, ErasedPropCore, PropRead, PropWrite};

#[derive(Clone, Serialize)]
pub struct Slider<T: Num + Copy + Serialize + Send + Sync + 'static> {
    range: Range<T>,
    step: T,
}

impl<T: Num + Copy + Serialize + Send + Sync + 'static> Slider<T> {
    pub fn new(range: Range<T>, step: T) -> Self {
        Self { range, step }
    }
}

#[derive(Clone)]
pub struct NumPropCore<T> where T: Num + Copy + Serialize + DeserializeOwned + Send + Sync + 'static {
    val: T,
    slider: Option<Slider<T>>,
}

impl<T> NumPropCore<T> where T: Num + Copy + Serialize + DeserializeOwned + Send + Sync + 'static {
    pub fn new(val: T) -> Self {
        Self {
            val,
            slider: None,
        }
    }
    pub fn new_slider(val: T, range: Range<T>, step: T) -> Self {
        Self {
            val,
            slider: Some(Slider::new(range, step)),
        }
    }

    pub fn fork(&self) -> Self {
        Self {
            val: self.val.clone(),
            slider: self.slider.clone(),
        }
    }
}

impl<T> PropCore for NumPropCore<T> where T: Num + Copy + Serialize + DeserializeOwned + Send + Sync + 'static {
    type Value = T;

    fn read(&self) -> PropRead<Self::Value> {
        PropRead::Ref(&self.val)
    }

    fn write(&mut self) -> PropWrite<Self::Value> {
        PropWrite::Ref(&mut self.val)
    }

    fn try_replace(&mut self, value: Self::Value) -> Result<Self::Value, String> where Self::Value: Sized {
        Ok(mem::replace(&mut self.val, value))
    }

    fn fork_dyn(&self) -> Box<dyn PropCore<Value=Self::Value>> {
        Box::new(self.fork())
    }
}

impl<T> ErasedPropCore for NumPropCore<T> where T: Num + Copy + Serialize + DeserializeOwned + Send + Sync + 'static {
    fn prop_type_id(&self) -> String {
        "num".to_string()
    }

    fn view_data(&self) -> HashMap<String, Box<dyn erased_serde::Serialize + 'static>> {
        HashMap::from([
            ("slider".to_string(), Box::new(self.slider.clone()) as Box<dyn erased_serde::Serialize>)
        ])
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        self.val = serde_json::from_str(str).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(&self.val)
    }
}
