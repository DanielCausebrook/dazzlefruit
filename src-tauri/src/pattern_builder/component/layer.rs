use std::collections::HashMap;
use dyn_clone::{clone_trait_object, DynClone};
use rand::random;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::RandId;
use crate::pattern_builder::component::layer::io_type::IOType;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::string::StringPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

pub mod texture;
pub mod layer_stack;
pub mod io_type;
pub mod generic;
pub mod scalar_texture;

pub trait LayerCore: Component + Send + Sync + DynClone {
    type Input;
    type Output;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output;
}
clone_trait_object!(<I, O> LayerCore<Input=I, Output=O>);
impl<T> LayerCore for Box<T> where T: LayerCore + Clone + ?Sized {
    type Input = T::Input;
    type Output = T::Output;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        self.as_mut().next(input, t, ctx)
    }
}

pub trait Layer: LayerCore {
    fn layer_type(&self) -> LayerType {
        LayerType::Generic
    }
    fn input_type(&self) -> &IOType<Self::Input>;
    fn output_type(&self) -> &IOType<Self::Output>;
    fn info(&self) -> &LayerInfo;
    fn view(&self) -> LayerView {
        LayerView::new(self)
    }
}
clone_trait_object!(<I, O> Layer<Input=I, Output=O>);

impl<T> Layer for Box<T> where T: Layer + Clone + ?Sized {

    fn layer_type(&self) -> LayerType {
        self.as_ref().layer_type()
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

    fn view(&self) -> LayerView {
        self.as_ref().view()
    }
}

#[derive(Copy, Clone, Serialize)]
pub enum LayerType {
    Generic,
    Texture,
    Filter,
    Group,
    Transformer,
}

pub struct LayerView {
    layer_type: LayerType,
    info: LayerInfo,
    property_views: Vec<PropView>,
    data: HashMap<String, Box<dyn erased_serde::Serialize + 'static>>,
}

impl LayerView {
    pub fn new(layer: &(impl Layer + ?Sized)) -> Self {
        Self {
            layer_type: layer.layer_type(),
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
        struct_ser.serialize_field("type", &self.layer_type)?;
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

#[derive(Clone)]
pub struct LayerInfo {
    id: RandId,
    name: Prop<String>,
    description: Prop<Option<String>>,
}

impl LayerInfo {
    pub fn new(name: &str) -> Self {
        Self {
            id: random(),
            name: StringPropCore::new(name.to_string()).into_prop(PropertyInfo::unnamed()),
            description: RawPropCore::new(None).into_prop(PropertyInfo::unnamed()),
        }
    }

    pub fn set_description(self, value: &str) -> Self {
        *self.description.write() = Some(value.to_string());
        self
    }

    pub fn id(&self) -> RandId {
        self.id
    }

    pub fn name(&self) -> &Prop<String> {
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

#[derive(Copy, Clone, Serialize)]
pub enum DisplayPane {
    Tree,
    Config,
    TreeAndConfig,
}
