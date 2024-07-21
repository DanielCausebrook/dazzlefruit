use std::collections::HashMap;
use dyn_clone::{clone_trait_object, DynClone};
use rand::random;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::pattern_builder::component::frame::{Blend, Opacity};
use crate::pattern_builder::component::RandId;
use crate::pattern_builder::component::layer::io_type::{DynType, DynTypeDef, NoMappingError, DynTypeMapper, DynValue};
use crate::pattern_builder::component::layer::layer_stack::StackTypeError;
use crate::pattern_builder::component::layer::texture::BlendingLayerCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::pattern_builder::component::property::string::OptionStringPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

pub mod texture;
pub mod layer_stack;
pub mod io_type;

pub trait LayerCore: Send + Sync + DynClone + 'static {
    type Input: DynType;
    type Output: DynType;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output;
    fn view_properties(&self) -> Vec<PropView>;
    fn detach(&mut self);
}
clone_trait_object!(<I, O> LayerCore<Input=I, Output=O>);

impl<T> LayerCore for Box<T> where T: LayerCore + Clone + ?Sized {
    type Input = T::Input;
    type Output = T::Output;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        self.as_mut().next(input, t, ctx)
    }
    fn view_properties(&self) -> Vec<PropView> {
        self.as_ref().view_properties()
    }

    fn detach(&mut self) {
        self.as_mut().detach()
    }
}

trait DynLayerCore: Send + Sync + DynClone + 'static {
    fn try_next(&mut self, input: DynValue, t: f64, ctx: &PatternContext) -> Result<DynValue, NoMappingError>;
    fn eval_type(&self, input_type: Option<DynTypeDef>, type_errors: &mut Vec<StackTypeError>, type_mapper: &DynTypeMapper) -> Result<Option<DynTypeDef>, NoMappingError>;
    fn view_properties(&self) -> Vec<PropView>;
    fn detach(&mut self);
}

clone_trait_object!(DynLayerCore);

impl<L> DynLayerCore for L where L: LayerCore {
    fn try_next(&mut self, input: DynValue, t: f64, ctx: &PatternContext) -> Result<DynValue, NoMappingError> {
        let input = input.try_into(ctx.type_mapper())
            .map_err(|err| err.err())?;
        Ok(self.next(input, t, ctx).into_dyn_value())
    }

    fn eval_type(&self, input_type: Option<DynTypeDef>, _type_errors: &mut Vec<StackTypeError>, type_mapper: &DynTypeMapper) -> Result<Option<DynTypeDef>, NoMappingError> {
        input_type.map(|input_type| {
            type_mapper.assert_has_mapping(input_type, L::Input::dyn_type_def())
                .map(|_| L::Output::dyn_type_def())
        }).transpose()
    }

    fn view_properties(&self) -> Vec<PropView> {
        L::view_properties(self)
    }

    fn detach(&mut self) {
        L::detach(self);
    }
}

#[derive(Clone)]
pub struct Layer {
    info: LayerInfo,
    type_info: LayerTypeInfo,
    core: Box<dyn DynLayerCore>,
}

impl Layer {
    pub fn new<I: DynType, O: DynType>(core: impl LayerCore<Input=I, Output=O>, type_info: LayerTypeInfo) -> Self {
        Self {
            info: LayerInfo::new(),
            type_info,
            core: Box::new(core),
        }
    }

    pub fn new_texture<T>(core: impl LayerCore<Input=(), Output=T>, mut type_info: LayerTypeInfo) -> Self where T: DynType + Blend + Opacity {
        if type_info.icon().is_none() {
            type_info = type_info.with_icon(LayerIcon::Texture);
        }
        Self {
            info: LayerInfo::new(),
            type_info,
            core: Box::new(BlendingLayerCore::new(core)),
        }
    }

    pub fn new_filter<T: DynType>(core: impl LayerCore<Input=T, Output=T>, mut type_info: LayerTypeInfo) -> Self {
        if type_info.icon().is_none() {
            type_info = type_info.with_icon(LayerIcon::Filter);
        }
        Self {
            info: LayerInfo::new(),
            type_info,
            core: Box::new(core),
        }
    }

    pub fn with_name(self, name: &str) -> Self {
        *self.info.name().write() = Some(name.to_string());
        self
    }

    pub fn with_description(self, description: &str) -> Self {
        *self.info.description().write() = Some(description.to_string());
        self
    }

    pub fn info(&self) -> &LayerInfo {
        &self.info
    }

    pub fn type_info(&self) -> &LayerTypeInfo {
        &self.type_info
    }

    pub fn eval_type(&self, input_type: Option<DynTypeDef>, type_errors: &mut Vec<StackTypeError>, type_mapper: &DynTypeMapper) -> Option<DynTypeDef> {
        match self.core.eval_type(input_type, type_errors, type_mapper) {
            Ok(o) => o,
            Err(err) => {
                type_errors.push(StackTypeError::LayerInput(self.info.clone(), err));
                None
            }
        }
    }

    pub fn try_next(&mut self, input: DynValue, t: f64, ctx: &PatternContext) -> Result<DynValue, StackTypeError> {
        self.core.try_next(input, t, ctx)
            .map_err(|err| StackTypeError::LayerInput(self.info().clone(), err))
    }

    pub fn view(&self) -> LayerView {
        LayerView {
            type_info: self.type_info().clone(),
            info: self.info().clone(),
            property_views: self.core.view_properties(),
            data: HashMap::new(),
        }
    }

    pub fn detach(&mut self) {
        self.info.detach();
        self.core.detach();
    }
}

#[derive(Copy, Clone, Serialize)]
pub enum LayerIcon {
    Texture,
    Filter,
    Group,
    Transformer,
}

pub struct LayerView {
    type_info: LayerTypeInfo,
    info: LayerInfo,
    property_views: Vec<PropView>,
    data: HashMap<String, Box<dyn erased_serde::Serialize + 'static>>,
}

impl LayerView {
    pub fn add_data(mut self, key: &str, value: impl Serialize + 'static) -> Self {
        self.data.insert(key.to_string(), Box::new(value));
        self
    }

    pub fn info(&self) -> &LayerInfo {
        &self.info
    }

    pub fn property_views(&self) -> &Vec<PropView> {
        &self.property_views
    }
}

impl Serialize for LayerView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut struct_ser = serializer.serialize_struct("Layer", 6)?;
        struct_ser.serialize_field("id", &self.info.id())?;
        struct_ser.serialize_field("type", &self.type_info)?;
        struct_ser.serialize_field("name", &self.info.name().view())?;
        struct_ser.serialize_field("description", &self.info.description().view())?;
        struct_ser.serialize_field("data", &self.data)?;
        struct_ser.serialize_field("properties", &self.property_views)?;
        struct_ser.end()
    }
}

#[derive(Clone, Serialize)]
pub struct LayerTypeInfo {
    name: String,
    description: Option<String>,
    icon: Option<LayerIcon>,
}

impl LayerTypeInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            icon: None,
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_icon(mut self, icon: LayerIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn icon(&self) -> &Option<LayerIcon> {
        &self.icon
    }
}

#[derive(Clone)]
pub struct LayerInfo {
    id: RandId,
    name: Prop<Option<String>>,
    description: Prop<Option<String>>,
}

impl LayerInfo {
    pub fn new() -> Self {
        Self {
            id: random(),
            name: OptionStringPropCore::new(None).into_prop(PropertyInfo::unnamed()),
            description: OptionStringPropCore::new(None).into_prop(PropertyInfo::unnamed()),
        }
    }

    pub fn named(name: &str) -> Self {
        Self {
            id: random(),
            name: OptionStringPropCore::new(Some(name.to_string())).into_prop(PropertyInfo::unnamed()),
            description: OptionStringPropCore::new(None).into_prop(PropertyInfo::unnamed()),
        }
    }

    pub fn with_name(self, value: &str) -> Self {
        *self.name.write() = Some(value.to_string());
        self
    }

    pub fn with_description(self, value: &str) -> Self {
        *self.description.write() = Some(value.to_string());
        self
    }

    pub fn id(&self) -> RandId {
        self.id
    }

    pub fn name(&self) -> &Prop<Option<String>> {
        &self.name
    }

    pub fn description(&self) -> &Prop<Option<String>> {
        &self.description
    }

    pub fn detach(&mut self) {
        self.id = random();
        self.name = self.name.fork();
        self.description = self.description.fork();
    }
}

impl Default for LayerInfo {
    fn default() -> Self {
        LayerInfo::new()
    }
}

#[derive(Copy, Clone, Serialize)]
pub enum DisplayPane {
    Tree,
    Config,
    TreeAndConfig,
}
