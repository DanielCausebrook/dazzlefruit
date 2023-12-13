use std::collections::HashMap;
use dyn_clone::{clone_trait_object, DynClone};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{BlendMode, PixelFrame};
use crate::pattern_builder::component::layer::{Layer, LayerInfo};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;

use crate::pattern_builder::library::core::RawPixels;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct TextureLayer {
    info: LayerInfo,
    layer_type: String,
    texture: Box<dyn Texture>,
    blend_mode: Prop<BlendMode>,
    opacity: Prop<f32>,
    cache: Option<PixelFrame>,
}

impl TextureLayer {
    pub fn new(texture: impl Texture, info: LayerInfo, layer_type: &str) -> Self {
        Self::new_from_boxed(Box::new(texture), info, layer_type)
    }

    pub fn new_from_boxed(texture: Box<impl Texture>, info: LayerInfo, layer_type: &str) -> Self {
        Self {
            info,
            layer_type: layer_type.to_string(),
            texture,
            blend_mode: RawPropCore::new(BlendMode::Normal).into_prop(PropertyInfo::new("Blend Mode")),
            opacity: NumPropCore::new_slider(1.0, 0.0..1.0, 0.01).into_prop(PropertyInfo::new("Opacity")),
            cache: None,
        }
    }

    pub fn blend_mode(&self) -> &Prop<BlendMode> {
        &self.blend_mode
    }

    pub fn opacity(&self) -> &Prop<f32> {
        &self.opacity
    }
}

impl Component for TextureLayer {
    fn view_properties(&self) -> Vec<PropView> {
        self.texture.view_properties()
    }

    fn detach(&mut self) {
        self.info.detach();
        self.texture.detach();
        self.blend_mode = self.blend_mode.fork();
        self.opacity = self.opacity.fork();
    }
}

impl Layer for TextureLayer {
    fn layer_type(&self) -> String {
        self.layer_type.clone()
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }

    fn view_data(&self) -> HashMap<String, Box<dyn erased_serde::Serialize + 'static>> {
        HashMap::from([
            ("blend_mode".to_string(), Box::new(*self.blend_mode.read()) as Box<dyn erased_serde::Serialize>),
            ("opacity".to_string(), Box::new(*self.opacity.read()) as Box<dyn erased_serde::Serialize>),
        ])
    }
}

impl Texture for TextureLayer {
    fn next_frame(&mut self, t: f64, ctx: &PatternContext) -> PixelFrame {
        let frame = self.texture.next_frame(t, ctx);
        self.cache = Some(frame.clone());
        frame
    }
}

pub trait Texture: Component + DynClone + Send + Sync {
    fn next_frame(&mut self, t: f64, ctx: &PatternContext) -> PixelFrame;

    fn into_layer(self, info: LayerInfo) -> TextureLayer where Self: Sized {
        TextureLayer::new(self, info, "texture")
    }
}
clone_trait_object!(Texture);

impl<T> Texture for Box<T> where T: Texture + Clone + ?Sized {
    fn next_frame(&mut self, t: f64, ctx: &PatternContext) -> PixelFrame {
        self.as_mut().next_frame(t, ctx)
    }

    fn into_layer(self, info: LayerInfo) -> TextureLayer where Self: Sized {
        (*self).into_layer(info)
    }
}

impl From<PixelFrame> for Box<dyn Texture> {
    fn from(value: PixelFrame) -> Self {
        Box::new(RawPixels::new(value))
    }
}
