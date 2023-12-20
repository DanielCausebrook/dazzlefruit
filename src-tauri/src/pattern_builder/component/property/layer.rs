use std::mem;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::filter::FilterLayer;
use crate::pattern_builder::component::layer::{Layer, LayerView};
use crate::pattern_builder::component::property::{ErasedPropCore, PropCore, PropRead, PropWrite};
use crate::pattern_builder::component::layer::texture::TextureLayer;
use crate::pattern_builder::component::layer::texture_generator::TextureGeneratorLayer;

pub struct LayerPropCore<T>(Box<T>) where T: Layer + Clone;

impl<T> LayerPropCore<T> where T: Layer + Clone {
    pub fn new(value: T) -> Self {
        Self (Box::new(value))
    }

    pub fn fork(&self) -> Self {
        let mut clone = self.0.clone();
        clone.detach();
        Self(clone)
    }
}

impl<T> Clone for LayerPropCore<T> where T: Layer + Clone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> PropCore for LayerPropCore<T> where T: Layer + Clone {
    type Value = T;

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

impl<T> ErasedPropCore for LayerPropCore<T> where T: Layer + Clone {
    fn prop_type_id(&self) -> String {
        "layer".to_string()
    }

    fn child_layer_views(&self) -> Vec<LayerView> {
        vec![self.0.view()]
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        todo!("Waiting for library")
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(self.0.info().id())
    }
}

pub struct LayerVecPropCore<T> (Vec<T>) where T: Layer + Clone;

impl<T> LayerVecPropCore<T> where T: Layer + Clone {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn fork(&self) -> Self {
        let mut vec_clone = self.0.clone();
        for component in vec_clone.iter_mut() {
            component.detach();
        }
        Self(vec_clone)
    }
}

impl<T> Clone for LayerVecPropCore<T> where T: Layer + Clone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> PropCore for LayerVecPropCore<T> where T: Layer + Clone {
    type Value = Vec<T>;

    fn read(&self) -> PropRead<Self::Value> {
        PropRead::Ref(&self.0)
    }

    fn write(&mut self) -> PropWrite<Self::Value> {
        PropWrite::Ref(&mut self.0)
    }

    fn try_replace(&mut self, value: Self::Value) -> Result<Self::Value, String> where Self::Value: Sized {
        if value.is_empty() {
            Ok(mem::replace(&mut self.0, vec![]))
        } else {
            Err("Can only replace the value of a ComponentVecProp with an empty vec.".to_string())
        }
    }

    fn fork_dyn(&self) -> Box<dyn PropCore<Value=Self::Value>> {
        Box::new(self.fork())
    }
}

impl<T> ErasedPropCore for LayerVecPropCore<T> where T: Layer + Clone {
    fn prop_type_id(&self) -> String {
        "layer-vec".to_string()
    }

    fn child_layer_views(&self) -> Vec<LayerView> {
        self.0.iter()
            .map(|layer| LayerView::new(layer))
            .collect()
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        todo!("Waiting for library")
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(self.0.iter().map(|l| l.info().id()).collect::<Vec<_>>())
    }
}

pub type TexturePropCore = LayerPropCore<TextureLayer>;
pub type TextureVecPropCore = LayerVecPropCore<TextureLayer>;

pub type FilterPropCore = LayerPropCore<FilterLayer>;

pub type FilterVecPropCore = LayerVecPropCore<FilterLayer>;

pub type TextureGeneratorPropCore = LayerPropCore<TextureGeneratorLayer>;

pub type TextureGeneratorVecPropCore = LayerVecPropCore<TextureGeneratorLayer>;

