use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{ColorPixel, Frame};
use crate::pattern_builder::component::layer::{Layer, LayerCore, LayerTypeInfo};
use crate::pattern_builder::component::property::PropView;
use crate::pattern_builder::pattern_context::PatternContext;

pub fn empty_texture_layer() -> Layer {
    Layer::new(EmptyTexture, LayerTypeInfo::new("Empty"))
}

#[derive(Clone)]
struct EmptyTexture;
#[derive(Clone)]
struct EmptyFilter;

impl LayerCore for EmptyTexture {
    type Input = Option<Frame<ColorPixel>>;
    type Output = Frame<ColorPixel>;
    fn next(&mut self, input: Self::Input, _t: f64, ctx: &PatternContext) -> Frame<ColorPixel> {
        input.unwrap_or_else(|| Frame::<ColorPixel>::empty(ctx.num_pixels()))
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!()
    }

    fn detach(&mut self) {
        fork_properties!();
    }
}

impl LayerCore for EmptyFilter {
    type Input = Frame<ColorPixel>;
    type Output = Frame<ColorPixel>;
    fn next(&mut self, active: Frame<ColorPixel>, _t: f64, _ctx: &PatternContext) -> Frame<ColorPixel> {
        active
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!()
    }

    fn detach(&mut self) {
        fork_properties!();
    }
}