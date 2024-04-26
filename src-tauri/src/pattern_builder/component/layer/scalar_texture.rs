use crate::pattern_builder::component::frame::{Blend, BlendMode, Frame, ScalarPixel};
use crate::pattern_builder::component::layer::{Layer, LayerCore, LayerInfo, LayerIcon, LayerView, LayerTypeInfo};
use crate::pattern_builder::component::layer::io_type::IOType;
use crate::pattern_builder::component::layer::standard_types::{SCALAR_FRAME, SCALAR_FRAME_OPTION};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;

use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct ScalarTextureLayer {
    info: LayerInfo,
    type_info: LayerTypeInfo,
    texture: Box<dyn LayerCore<Input=(), Output=Frame<ScalarPixel>>>,
    blend_mode: Prop<BlendMode>,
}

impl ScalarTextureLayer {
    pub fn new(texture: impl LayerCore<Input=(), Output=Frame<ScalarPixel>>, mut type_info: LayerTypeInfo) -> Self {
        if type_info.icon().is_none() {
            type_info = type_info.with_icon(LayerIcon::Texture);
        }
        Self {
            info: LayerInfo::default(),
            type_info,
            texture: Box::new(texture),
            blend_mode: RawPropCore::new(BlendMode::Normal).into_prop(PropertyInfo::new("Blend Mode")),
        }
    }

    pub fn blend_mode(&self) -> &Prop<BlendMode> {
        &self.blend_mode
    }
}

impl LayerCore for ScalarTextureLayer {
    type Input = Option<Frame<ScalarPixel>>;
    type Output = Frame<ScalarPixel>;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        let frame = self.texture.next((), t, ctx);
        if let Some(active) = input {
            frame.blend(active, *self.blend_mode.read())
        } else {
            frame
        }
    }

    fn view_properties(&self) -> Vec<PropView> {
        self.texture.view_properties()
    }

    fn detach(&mut self) {
        self.info.detach();
        self.texture.detach();
        self.blend_mode = self.blend_mode.fork();
    }
}

impl Layer for ScalarTextureLayer {
    fn type_info(&self) -> &LayerTypeInfo {
        &self.type_info
    }

    fn input_type(&self) -> &IOType<Self::Input> {
        &SCALAR_FRAME_OPTION
    }

    fn output_type(&self) -> &IOType<Self::Output> {
        &SCALAR_FRAME
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }

    fn view(&self) -> LayerView {
        LayerView::new(self)
            .add_data("blend_mode", *self.blend_mode.read())
    }
}
