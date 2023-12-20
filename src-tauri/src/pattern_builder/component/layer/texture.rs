use dyn_clone::{clone_trait_object, DynClone};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{BlendMode, PixelFrame};
use crate::pattern_builder::component::layer::{Layer, LayerInfo, LayerView};
use crate::pattern_builder::component::layer::io_type::IOType;
use crate::pattern_builder::component::layer::standard_types::{PIXEL_FRAME, PIXEL_FRAME_OPTION};
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
    type Input = Option<PixelFrame>;
    type Output = PixelFrame;
    fn layer_type(&self) -> String {
        self.layer_type.clone()
    }

    fn input_type(&self) -> &IOType<Self::Input> {
        &PIXEL_FRAME_OPTION
    }

    fn output_type(&self) -> &IOType<Self::Output> {
        &PIXEL_FRAME
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }

    fn view(&self) -> LayerView {
        LayerView::new(self)
            .add_data("blend_mode", *self.blend_mode.read())
            .add_data("opacity", *self.opacity.read())
    }

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        let frame = self.texture.next_frame(t, ctx);
        self.cache = Some(frame.clone());
        if let Some(active) = input {
            frame.blend(active, *self.blend_mode().read())
        } else {
            frame
        }
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
