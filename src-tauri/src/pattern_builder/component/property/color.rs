use std::mem;
use std::str::FromStr;
use erased_serde::Serialize;
use palette::{LinSrgba, Srgb};
use palette::rgb::Rgb;
use crate::pattern_builder::component::property::{PropCore, ErasedPropCore, PropRead, PropWrite};

#[derive(Clone)]
pub struct ColorPropCore(LinSrgba);

impl ColorPropCore {
    pub fn new(color: LinSrgba) -> Self {
        Self(color)
    }
    pub fn fork(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PropCore for ColorPropCore {
    type Value = LinSrgba;

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

impl ErasedPropCore for ColorPropCore {
    fn prop_type_id(&self) -> String {
        "color".to_string()
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        let color = Rgb::from_str(str)
            .map_err(|e| e.to_string())?;
        self.0 = color.into();
        Ok(())
    }

    fn value_serialize(&self) -> Box<dyn Serialize + '_> {
        Box::new(Srgb::<u8>::from_linear(self.0.clone().premultiply().into()).into_components())
    }
}