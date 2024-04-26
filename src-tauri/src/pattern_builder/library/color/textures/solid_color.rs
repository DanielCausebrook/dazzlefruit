use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{ColorPixel, Frame};
use crate::pattern_builder::component::layer::{DisplayPane, LayerCore, LayerTypeInfo};
use crate::pattern_builder::component::layer::texture::TextureLayer;
use crate::pattern_builder::component::property::color::ColorPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct SolidColor {
    color: Prop<ColorPixel>,
}

impl SolidColor {
    pub fn new(color: ColorPixel) -> Self {
        Self {
            color: ColorPropCore::new(color).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }

    pub fn into_layer(self) -> TextureLayer {
        TextureLayer::new(self, LayerTypeInfo::new("Color"))
    }
}

impl LayerCore for SolidColor {
    type Input = ();
    type Output = Frame<ColorPixel>;
    fn next(&mut self, _: (), _t: f64, ctx: &PatternContext) -> Frame<ColorPixel> {
        vec![self.color.read().clone(); ctx.num_pixels()].into()
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.color)
    }

    fn detach(&mut self) {
        fork_properties!(self.color);
    }
}