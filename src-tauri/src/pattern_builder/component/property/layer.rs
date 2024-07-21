use crate::pattern_builder::component::layer::{Layer, LayerView};
use crate::pattern_builder::component::property::{ErasedPropCore, PropCore, PropRead, PropWrite};

#[derive(Clone)]
pub struct LayerPropCore(Layer);

impl LayerPropCore {
    pub fn new(value: Layer) -> Self {
        Self (value)
    }

    pub fn fork(&self) -> Self {
        let mut clone = self.0.clone();
        clone.detach();
        Self(clone)
    }
}

impl PropCore for LayerPropCore {
    type Value = Layer;

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

impl ErasedPropCore for LayerPropCore {
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

pub struct LayerVecPropCore (Vec<Layer>);

impl LayerVecPropCore {
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

impl Clone for LayerVecPropCore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PropCore for LayerVecPropCore {
    type Value = Vec<Layer>;

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

impl ErasedPropCore for LayerVecPropCore {
    fn prop_type_id(&self) -> String {
        "layer-vec".to_string()
    }

    fn child_layer_views(&self) -> Vec<LayerView> {
        self.0.iter()
            .map(|layer| layer.view())
            .collect()
    }

    fn try_update(&mut self, str: &str) -> Result<(), String> {
        todo!("Waiting for library")
    }

    fn value_serialize(&self) -> Box<dyn erased_serde::Serialize + '_> {
        Box::new(self.0.iter().map(|l| l.info().id()).collect::<Vec<_>>())
    }
}