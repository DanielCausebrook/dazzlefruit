use std::collections::HashMap;
use dyn_clone::{clone_trait_object, DynClone};
use rand::random;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::pattern_builder::component::RandId;
use crate::pattern_builder::component::layer::io_type::{ErasedIOType, ErasedIOValue, IOType};
use crate::pattern_builder::component::layer::layer_stack::StackTypeError;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::string::OptionStringPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

pub mod texture;
pub mod layer_stack;
pub mod io_type;
pub mod generic;
pub mod scalar_texture;

pub trait LayerCore: Send + Sync + DynClone + 'static {
    type Input;
    type Output;

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

pub trait Layer: LayerCore {
    fn type_info(&self) -> &LayerTypeInfo;
    fn input_type(&self) -> &IOType<Self::Input>;
    fn output_type(&self) -> &IOType<Self::Output>;
    fn info(&self) -> &LayerInfo;
    fn with_name(self, name: &str) -> Self where Self: Sized {
        *self.info().name().write() = Some(name.to_string());
        self
    }
    fn with_description(self, description: &str) -> Self where Self: Sized {
        *self.info().description().write() = Some(description.to_string());
        self
    }
    fn view(&self) -> LayerView {
        LayerView::new(self)
    }
}
clone_trait_object!(<I, O> Layer<Input=I, Output=O>);

impl<T> Layer for Box<T> where T: Layer + Clone + ?Sized {

    fn type_info(&self) -> &LayerTypeInfo {
        self.as_ref().type_info()
    }

    fn input_type(&self) -> &IOType<Self::Input> {
        self.as_ref().input_type()
    }

    fn output_type(&self) -> &IOType<Self::Output> {
        self.as_ref().output_type()
    }

    fn info(&self) -> &LayerInfo {
        self.as_ref().info()
    }
    fn with_name(self, name: &str) -> Self where Self: Sized {
        Box::new((*self).with_name(name))
    }
    fn with_description(self, description: &str) -> Self where Self: Sized {
        Box::new((*self).with_description(description))
    }
    fn view(&self) -> LayerView {
        self.as_ref().view()
    }
}

pub trait ErasedLayer: Send + Sync + DynClone + 'static {
    fn input_type(&self) -> &dyn ErasedIOType;
    fn output_type(&self) -> &dyn ErasedIOType;
    fn info(&self) -> &LayerInfo;
    fn type_info(&self) -> &LayerTypeInfo;
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

    fn type_info(&self) -> &LayerTypeInfo {
        L::type_info(self)
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
    pub fn new(layer: &(impl Layer + ?Sized)) -> Self {
        Self {
            type_info: layer.type_info().clone(),
            info: layer.info().clone(),
            property_views: layer.view_properties(),
            data: HashMap::new(),
        }
    }

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

pub mod standard_types {
    use once_cell::sync::Lazy;
    use crate::pattern_builder::component::frame::{ColorPixel, Frame, ScalarPixel};
    use crate::pattern_builder::component::layer::io_type::IOType;
    use crate::pattern_builder::component::layer::texture::TextureLayer;

    pub static VOID: Lazy<IOType<()>> = Lazy::new(|| IOType::new("()"));
    pub static COLOR_FRAME: Lazy<IOType<Frame<ColorPixel>>> = Lazy::new(|| {
        let mut ty = IOType::new("ColorFrame");
        ty.add_mapping_into(|frame| Some(frame));
        ty
    });
    pub static COLOR_FRAME_OPTION: Lazy<IOType<Option<Frame<ColorPixel>>>> = Lazy::new(|| {
        let mut ty = IOType::new("Option<ColorFrame>");
        ty.add_mapping_from(|frame| Some(frame));
        ty.add_mapping_from(|_: ()| None);
        ty
    });

    pub static TEXTURE_LAYER: Lazy<IOType<TextureLayer>> = Lazy::new(|| {
        let mut ty = IOType::new("TextureLayer");
        ty
    });

    pub static SCALAR_FRAME: Lazy<IOType<Frame<ScalarPixel>>> = Lazy::new(|| {
        let mut ty = IOType::new("ScalarFrame");
        ty.add_mapping_into(|frame| Some(frame));
        ty
    });

    pub static SCALAR_FRAME_OPTION: Lazy<IOType<Option<Frame<ScalarPixel>>>> = Lazy::new(|| {
        let mut ty = IOType::new("Option<ScalarFrame>");
        ty.add_mapping_from(|frame| Some(frame));
        ty.add_mapping_from(|_: ()| None);
        ty
    });
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
