use dyn_clone::{clone_trait_object, DynClone};

use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::{Layer, LayerInfo};
use crate::pattern_builder::component::layer::io_type::IOType;
use crate::pattern_builder::component::layer::standard_types::{TEXTURE_LAYER, VOID};
use crate::pattern_builder::component::property::PropView;
use crate::pattern_builder::component::layer::texture::TextureLayer;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct TextureGeneratorLayer {
    info: LayerInfo,
    layer_type: String,
    generator: Box<dyn TextureGenerator>,
}

impl TextureGeneratorLayer {
    pub fn new(generator: impl TextureGenerator, info: LayerInfo, layer_type: &str) -> Self {
        Self::new_from_boxed(Box::new(generator), info, layer_type)
    }

    pub fn new_from_boxed(generator: Box<impl TextureGenerator>, info: LayerInfo, layer_type: &str) -> Self {
        Self {
            info,
            layer_type: layer_type.to_string(),
            generator,
        }
    }
}

impl Component for TextureGeneratorLayer {
    fn view_properties(&self) -> Vec<PropView> {
        self.generator.view_properties()
    }

    fn detach(&mut self) {
        self.info.detach();
        self.generator.detach();
    }
}

impl Layer for TextureGeneratorLayer {
    type Input = ();
    type Output = TextureLayer;

    fn layer_type(&self) -> String {
        self.layer_type.clone()
    }

    fn input_type(&self) -> &IOType<Self::Input> {
        &VOID
    }

    fn output_type(&self) -> &IOType<Self::Output> {
        &TEXTURE_LAYER
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        self.generator.next_texture()
    }
}

pub trait TextureGenerator: Component + DynClone + Send + Sync {
    fn next_texture(&mut self) -> TextureLayer;

    fn into_layer(self, info: LayerInfo) -> TextureGeneratorLayer where Self: Sized {
        TextureGeneratorLayer::new(self, info, "texture-generator")
    }
}
clone_trait_object!(TextureGenerator);

impl<T> TextureGenerator for Box<T> where T: TextureGenerator + Clone + ?Sized {
    fn next_texture(&mut self) -> TextureLayer {
        self.as_mut().next_texture()
    }

    fn into_layer(self, info: LayerInfo) -> TextureGeneratorLayer where Self: Sized {
        (*self).into_layer(info)
    }
}