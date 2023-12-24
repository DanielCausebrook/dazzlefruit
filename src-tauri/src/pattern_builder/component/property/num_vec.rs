use std::collections::HashMap;
use std::ops::Range;
use nalgebra_glm::TVec;
use num_traits::Num;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::pattern_builder::component::property::num::Slider;
use crate::pattern_builder::component::property::{ErasedPropCore, PropCore, PropRead, PropWrite};

#[derive(Clone)]
pub struct NumVecPropCore<T, const D: usize> where T: Num + Copy + Serialize + DeserializeOwned + Send + Sync + 'static {
    val: TVec<T, D>,
    sliders: [Option<Slider<T>>; D],
}
impl<T, const D: usize> NumVecPropCore<T, D> where T: Num + Copy + Serialize + DeserializeOwned + Send + Sync + 'static {
    pub fn new(val: TVec<T, D>) -> Self {
        Self {
            val,
            sliders: [const {None}; D],
        }
    }

    pub fn new_slider(val: TVec<T, D>, range: Range<T>, step: T) -> Self {
        let mut sliders = [const {None}; D];
        let slider = Slider::new(range, step);
        for entry in sliders.iter_mut() {
            *entry = Some(slider.clone());
        }
        Self { val, sliders }
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }
}

impl<T, const D: usize> PropCore for NumVecPropCore<T, D> where T: Num + Copy + Serialize + DeserializeOwned + Send + Sync + 'static {
    type Value = TVec<T, D>;

    fn read(&self) -> PropRead<Self::Value> {
        PropRead::Ref(&self.val)
    }

    fn write(&mut self) -> PropWrite<Self::Value> {
        PropWrite::Ref(&mut self.val)
    }

    fn fork_dyn(&self) -> Box<dyn PropCore<Value=Self::Value>> {
        Box::new(self.fork())
    }
}

impl<T, const D: usize> ErasedPropCore for NumVecPropCore<T, D> where T: Num + Copy + Serialize + DeserializeOwned + Send + Sync + 'static {
    fn prop_type_id(&self) -> String {
        "num-vec".to_string()
    }

    fn view_data(&self) -> HashMap<String, Box<dyn erased_serde::Serialize + 'static>> {
        HashMap::from([
            ("sliders".to_string(), Box::new(self.sliders.to_vec()) as Box<dyn erased_serde::Serialize>)
        ])
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        let vec: Vec<T> = serde_json::from_str(str).map_err(|e| e.to_string())?;
        for i in 0..D {
            self.val[i] = *vec.get(i).ok_or("Provided vector is not long enough.")?;
        }
        Ok(())
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(self.val.as_slice())
    }
}