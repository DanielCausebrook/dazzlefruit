use std::mem;
use serde::{Serialize, Serializer};
use serde::ser::SerializeSeq;
use crate::pattern_builder::component::{Layer, Component, LayerInfo};
use crate::pattern_builder::component::filter::FilterLayer;
use crate::pattern_builder::component::property::{PropCore, ErasedPropCore, PropRead, PropWrite};
use crate::pattern_builder::component::texture::TextureLayer;
use crate::pattern_builder::component::texture_generator::TextureGeneratorLayer;

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
        "component".to_string()
    }

    fn for_each_child_layer<'a>(&self, func: &mut (dyn FnMut(&dyn Layer) + 'a)) {
        func(self.0.as_ref())
    }

    fn for_each_child_layer_mut<'a>(&mut self, func: &mut (dyn FnMut(&mut dyn Layer) + 'a)) {
        func(self.0.as_mut())
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        todo!("Waiting for library")
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(ComponentSerializer(&self.0))
    }
}

struct ComponentSerializer<'a, T> (&'a T) where T: Layer;

impl<T> Serialize for ComponentSerializer<'_, T> where T: Layer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.0.info().id().serialize(serializer)
    }
}

pub struct ComponentVecPropCore<T> (Vec<T>) where T: Layer + Clone;

impl<T> ComponentVecPropCore<T> where T: Layer + Clone {
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

impl<T> Clone for ComponentVecPropCore<T> where T: Layer + Clone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> PropCore for ComponentVecPropCore<T> where T: Layer + Clone {
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

impl<T> ErasedPropCore for ComponentVecPropCore<T> where T: Layer + Clone {
    fn prop_type_id(&self) -> String {
        "component-vec".to_string()
    }

    fn for_each_child_layer<'a>(&self, func: &mut (dyn FnMut(&dyn Layer) + 'a)) {
        for component in self.0.iter() {
            func(component);
        }
    }

    fn for_each_child_layer_mut<'a>(&mut self, func: &mut (dyn FnMut(&mut dyn Layer) + 'a)) {
        for component in self.0.iter_mut() {
            func(component);
        }
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        todo!("Waiting for library")
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(ComponentVecSerializer(&self.0))
    }
}


struct ComponentVecSerializer<'a, T> (&'a Vec<T>) where T: Layer;

impl<T> Serialize for ComponentVecSerializer<'_, T> where T: Layer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut seq_ser = serializer.serialize_seq(Some(self.0.len()))?;
        for component in &*self.0 {
            seq_ser.serialize_element(&component.info().id())?;
        }
        seq_ser.end()
    }
}

pub trait LayerPropMetadata: 'static {
    type Layer: Layer + Clone;
    type Component;

    fn layer_ref_as_value(layer: &Box<Self::Layer>) -> &Self::Component;
    fn layer_mut_as_value(layer: &mut Box<Self::Layer>) -> &mut Self::Component;
    fn component_into_layer(component: Self::Component, info: LayerInfo) -> Self::Layer;
}

pub type TexturePropCore = LayerPropCore<TextureLayer>;
pub type TextureVecPropCore = ComponentVecPropCore<TextureLayer>;

pub type FilterPropCore = LayerPropCore<FilterLayer>;

pub type FilterVecPropCore = ComponentVecPropCore<FilterLayer>;

pub type TextureGeneratorPropCore = LayerPropCore<TextureGeneratorLayer>;

pub type TextureGeneratorVecPropCore = ComponentVecPropCore<TextureGeneratorLayer>;

