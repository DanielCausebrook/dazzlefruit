use std::collections::HashMap;
use dyn_clone::{clone_trait_object, DynClone};
use rand::random;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::RandId;
use crate::pattern_builder::component::layer::io_type::IOType;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::string::StringProp;
use crate::pattern_builder::pattern_context::PatternContext;

pub mod texture;
pub mod filter;
pub mod texture_generator;
pub mod layer_stack;
pub mod io_type;

pub trait Layer: Component + Send + Sync + DynClone {
    type Input;
    type Output;
    fn layer_type(&self) -> String;
    fn input_type(&self) -> &IOType<Self::Input>;
    fn output_type(&self) -> &IOType<Self::Output>;
    fn info(&self) -> &LayerInfo;
    fn view(&self) -> LayerView {
        LayerView::new(self)
    }
    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output;
}
clone_trait_object!(<I, O> Layer<Input=I, Output=O>);

impl<T> Layer for Box<T> where T: Layer + Clone + ?Sized {
    type Input = T::Input;
    type Output = T::Output;

    fn layer_type(&self) -> String {
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

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        self.as_mut().next(input, t, ctx)
    }
}

pub struct LayerView {
    type_str: String,
    info: LayerInfo,
    property_views: Vec<PropView>,
    data: HashMap<String, Box<dyn erased_serde::Serialize + 'static>>,
}

impl LayerView {
    pub fn new(layer: &(impl Layer + ?Sized)) -> Self {
        Self {
            type_str: layer.layer_type(),
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
        struct_ser.serialize_field("type", &self.type_str)?;
        struct_ser.serialize_field("name", &self.info.name().view())?;
        struct_ser.serialize_field("description", &self.info.description().view())?;
        struct_ser.serialize_field("data", &self.data)?;
        struct_ser.serialize_field("properties", &self.property_views)?;
        struct_ser.end()
    }
}

pub mod standard_types {
    use once_cell::sync::Lazy;
    use crate::pattern_builder::component::data::PixelFrame;
    use crate::pattern_builder::component::layer::io_type::IOType;
    use crate::pattern_builder::component::layer::texture::TextureLayer;

    pub static VOID: Lazy<IOType<()>> = Lazy::new(|| IOType::new("()"));
    pub static  PIXEL_FRAME: Lazy<IOType<PixelFrame>> = Lazy::new(|| {
        let mut ty = IOType::new("PixelFrame");
        ty.add_mapper_into(|frame| Some(frame));
        ty
    });
    pub static PIXEL_FRAME_OPTION: Lazy<IOType<Option<PixelFrame>>> = Lazy::new(|| {
        let mut ty = IOType::new("Option<PixelFrame>");
        ty.add_mapper_from(|frame| Some(frame));
        ty.add_mapper_from(|_: ()| None);
        ty
    });

    pub static TEXTURE_LAYER: Lazy<IOType<TextureLayer>> = Lazy::new(|| {
        let mut ty = IOType::new("TextureLayer");
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
            name: StringProp::new(name.to_string()).into_prop(PropertyInfo::unnamed()),
            description: RawPropCore::new(None).into_prop(PropertyInfo::unnamed()),
        }
    }

    pub fn set_description(self, value: &str) -> Self {
        self.description.try_replace_value(Some(value.to_string())).unwrap();
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
