use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::frame::{Blend, BlendMode, ColorPixel, Frame};
use crate::pattern_builder::component::layer::{Layer, LayerCore, LayerInfo, LayerType, LayerView};
use crate::pattern_builder::component::layer::io_type::IOType;
use crate::pattern_builder::component::layer::standard_types::{COLOR_FRAME, COLOR_FRAME_OPTION};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;

use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct TextureLayer {
    info: LayerInfo,
    layer_type: LayerType,
    texture: Box<dyn LayerCore<Input=(), Output=Frame<ColorPixel>>>,
    blend_mode: Prop<BlendMode>,
    opacity: Prop<f32>,
    cache: Option<Frame<ColorPixel>>,
}

impl TextureLayer {
    pub fn new(texture: impl LayerCore<Input=(), Output=Frame<ColorPixel>>, info: LayerInfo) -> Self {
        Self::new_from_boxed(Box::new(texture), info)
    }

    pub fn new_from_boxed(texture: Box<impl LayerCore<Input=(), Output=Frame<ColorPixel>>>, info: LayerInfo) -> Self {
        Self {
            info,
            layer_type: LayerType::Texture,
            texture,
            blend_mode: RawPropCore::new(BlendMode::Normal).into_prop(PropertyInfo::new("Blend Mode")),
            opacity: NumPropCore::new_slider(1.0, 0.0..1.0, 0.01).into_prop(PropertyInfo::new("Opacity")),
            cache: None,
        }
    }

    pub fn set_layer_type(mut self, layer_type: LayerType) -> Self {
        self.layer_type = layer_type;
        self
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
        self.texture.view_properties().into_iter()
            .chain([self.opacity.view()])
            .collect()
    }

    fn detach(&mut self) {
        self.info.detach();
        self.texture.detach();
        self.blend_mode = self.blend_mode.fork();
        self.opacity = self.opacity.fork();
    }
}

impl LayerCore for TextureLayer {
    type Input = Option<Frame<ColorPixel>>;
    type Output = Frame<ColorPixel>;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        let frame = self.texture.next((), t, ctx);
        self.cache = Some(frame.clone());
        if let Some(active) = input {
            frame.blend(active, *self.blend_mode().read())
        } else {
            frame
        }
    }
}

impl Layer for TextureLayer {
    fn layer_type(&self) -> LayerType {
        self.layer_type
    }

    fn input_type(&self) -> &IOType<Self::Input> {
        &COLOR_FRAME_OPTION
    }

    fn output_type(&self) -> &IOType<Self::Output> {
        &COLOR_FRAME
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }

    fn view(&self) -> LayerView {
        LayerView::new(self)
            .add_data("blend_mode", *self.blend_mode.read())
            .add_data("opacity", *self.opacity.read())
    }
}
