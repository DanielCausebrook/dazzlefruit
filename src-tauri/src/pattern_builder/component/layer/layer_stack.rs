use std::fmt::{Debug, Formatter};
use itertools::{FoldWhile, Itertools};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::pattern_builder::component::RandId;
use crate::pattern_builder::component::layer::io_type::{DynTypeDef, NoMappingError, DynTypeMapper, DynValue, DynType};
use crate::pattern_builder::component::layer::{Layer, LayerInfo, LayerView};
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct LayerStack {
    stack: Vec<Layer>,
}

impl LayerStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn layer_views(&self) -> Vec<LayerView> {
        self.stack.iter()
            .map(|l| l.view())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn insert(&mut self, index: usize, layer: Layer) {
        self.stack.insert(index, layer)
    }

    pub fn push(&mut self, layer: Layer) {
        self.stack.push(layer)
    }

    fn remove(&mut self, index: usize) -> Layer {
        self.stack.remove(index)
    }

    pub fn eval_type(&self, input_type: Option<DynTypeDef>, type_errors: &mut Vec<StackTypeError>, type_mapper: &DynTypeMapper) -> Option<DynTypeDef> {
        let mut ty = input_type;
        for layer in self.stack.iter() {
            ty = layer.eval_type(ty, type_errors, type_mapper)
        }
        ty
    }

    pub fn next_dyn(&mut self, input: DynValue, t: f64, ctx: &PatternContext) -> Result<DynValue, StackTypeError> {
        self.stack.iter_mut()
            .fold_while(Ok(input), |value, layer| {
                match layer.try_next(value.unwrap(), t, &ctx) {
                    Ok(value) => FoldWhile::Continue(Ok(value)),
                    Err(err) => FoldWhile::Done(Err(err)),
                }
            })
            .into_inner()
    }

    pub fn next<I: DynType, O: DynType>(&mut self, input: I, t: f64, ctx: &PatternContext) -> Result<O, StackTypeError> {
        self.next_dyn(input.into_dyn_value(), t, &ctx)
            .and_then(|out_dyn| out_dyn.try_into(ctx.type_mapper()).map_err(|err| StackTypeError::StackOutput(err.err())))
    }

    pub fn detach(&mut self) {
        for layer in self.stack.iter_mut() {
            layer.detach();
        }
    }
}

pub enum StackTypeError {
    StackOutput(NoMappingError),
    LayerInput(LayerInfo, NoMappingError),
}

impl Debug for StackTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StackTypeError::StackOutput(err) => write!(f, "Invalid Stack output: Expected {}, got {}", err.into().name(), err.from().name()),
            StackTypeError::LayerInput(layer_info, err) => write!(f, "Invalid input for Stack layer {} ({:?}): Expected {}, got {}", layer_info.id(), *layer_info.name().read() , err.into().name(), err.from().name()),
        }
    }
}

impl Serialize for StackTypeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("StackTypeError", 3)?;
        match self {
            StackTypeError::StackOutput(err) => {
                struct_ser.serialize_field("layer_id", &Option::<RandId>::None)?;
                struct_ser.serialize_field("from_type_name", &err.from().name())?;
                struct_ser.serialize_field("into_type_name", &err.into().name())?;
            }
            StackTypeError::LayerInput(layer_info, err) => {
                struct_ser.serialize_field("layer_id", &Some(layer_info.id()))?;
                struct_ser.serialize_field("from_type_name", &err.from().name())?;
                struct_ser.serialize_field("into_type_name", &err.into().name())?;
            }
        }
        struct_ser.end()
    }
}