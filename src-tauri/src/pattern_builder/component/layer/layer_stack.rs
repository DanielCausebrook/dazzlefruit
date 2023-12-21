use std::fmt::{Debug, Formatter};
use dyn_clone::{clone_trait_object, DynClone};
use itertools::{FoldWhile, Itertools};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::pattern_builder::component::RandId;
use crate::pattern_builder::component::layer::io_type::{ErasedIOType, ErasedIOValue, IOType, NoMappingError};
use crate::pattern_builder::component::layer::{Layer, LayerInfo, LayerView};
use crate::pattern_builder::pattern_context::PatternContext;

trait ErasedLayer: Send + Sync + DynClone + 'static {
    fn input_type(&self) -> &dyn ErasedIOType;
    fn output_type(&self) -> &dyn ErasedIOType;
    fn info(&self) -> &LayerInfo;
    fn try_next(&mut self, input: ErasedIOValue, t: f64, ctx: &PatternContext) -> Result<ErasedIOValue, StackTypeError>;
    fn view(&self) -> LayerView;
    fn detach(&mut self);
}
clone_trait_object!(ErasedLayer);

impl<L> ErasedLayer for L where L: Layer + Clone {
    fn input_type(&self) -> &dyn ErasedIOType {
        L::input_type(self)
    }

    fn output_type(&self) -> &dyn ErasedIOType {
        L::output_type(self)
    }

    fn info(&self) -> &LayerInfo {
        L::info(self)
    }

    fn try_next(&mut self, input: ErasedIOValue, t: f64, ctx: &PatternContext) -> Result<ErasedIOValue, StackTypeError> {
        let input = input.try_into(self.input_type())
            .map_err(|err| StackTypeError::LayerInput(self.info().id(), err))?;
        Ok(ErasedIOValue::new(self.next(input, t, ctx), self.output_type()))
    }

    fn view(&self) -> LayerView {
        L::view(self)
    }

    fn detach(&mut self) {
        L::detach(self);
    }
}

pub struct LayerStack<I, O> where I: 'static, O: 'static {
    stack: Vec<Box<dyn ErasedLayer>>,
    input_type: &'static IOType<I>,
    output_type: &'static IOType<O>,
}

impl<I, O> Clone for LayerStack<I, O> where I: 'static, O: 'static {
    fn clone(&self) -> Self {
        Self {
            stack: self.stack.clone(),
            input_type: self.input_type,
            output_type: self.output_type,
        }
    }
}

impl<I, O> LayerStack<I, O> where I: 'static, O: 'static {
    pub fn new(input_type: &'static IOType<I>, output_type: &'static IOType<O>) -> Self {
        Self {
            stack: Vec::new(),
            input_type,
            output_type,
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

    pub fn insert(&mut self, index: usize, layer: impl Layer + Clone) {
        self.stack.insert(index, Box::new(layer))
    }

    pub fn push(&mut self, layer: impl Layer + Clone) {
        self.stack.push(Box::new(layer))
    }

    fn remove(&mut self, index: usize) -> Box<dyn ErasedLayer> {
        self.stack.remove(index)
    }

    pub fn type_errors(&self) -> Vec<StackTypeError> {
        let mut errors = vec![];
        let mut ty: &dyn ErasedIOType = self.input_type;
        for layer in self.stack.iter() {
            if let Err(err) = ty.can_map_into(layer.input_type()) {
                errors.push(StackTypeError::LayerInput(layer.info().id(), err));
            }
            ty = layer.output_type();
        }
        if let Err(err) = ty.can_map_into(self.output_type) {
            errors.push(StackTypeError::StackOutput(err))
        }
        errors
    }

    pub fn next(&mut self, input: I, t: f64, ctx: &PatternContext) -> Result<O, StackTypeError> {
        let input = ErasedIOValue::new(input, self.input_type);
        self.stack.iter_mut()
            .fold_while(Ok(input), |value, layer| {
                match layer.try_next(value.unwrap(), t, &ctx) {
                    Ok(value) => FoldWhile::Continue(Ok(value)),
                    Err(err) => FoldWhile::Done(Err(err)),
                }
            })
            .into_inner()
            .and_then(|value| value.try_into(self.output_type)
                .map_err(|err| StackTypeError::StackOutput(err))
            )
    }

    pub fn detach(&mut self) {
        for layer in self.stack.iter_mut() {
            layer.detach();
        }
    }
}

pub enum StackTypeError {
    StackOutput(NoMappingError),
    LayerInput(RandId, NoMappingError),
}

impl Debug for StackTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StackTypeError::StackOutput(err) => write!(f, "Invalid Stack output: Expected {}, got {}", err.get_into_type_name(), err.get_from_type_name()),
            StackTypeError::LayerInput(layer_id, err) => write!(f, "Invalid input for Stack layer {}: Expected {}, got {}", layer_id, err.get_into_type_name(), err.get_from_type_name()),
        }
    }
}

impl Serialize for StackTypeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("StackTypeError", 3)?;
        match self {
            StackTypeError::StackOutput(err) => {
                struct_ser.serialize_field("layer_id", &Option::<RandId>::None)?;
                struct_ser.serialize_field("from_type_name", err.get_from_type_name())?;
                struct_ser.serialize_field("into_type_name", err.get_into_type_name())?;
            }
            StackTypeError::LayerInput(layer_id, err) => {
                struct_ser.serialize_field("layer_id", &Some(layer_id))?;
                struct_ser.serialize_field("from_type_name", err.get_from_type_name())?;
                struct_ser.serialize_field("into_type_name", err.get_into_type_name())?;
            }
        }
        struct_ser.end()
    }
}