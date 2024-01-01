use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::property::PropView;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{ColorPixel, Frame, ScalarPixel};
use crate::pattern_builder::component::layer::{LayerCore, LayerIcon, LayerTypeInfo};
use crate::pattern_builder::component::layer::generic::GenericLayer;
use crate::pattern_builder::component::layer::standard_types::{COLOR_FRAME, SCALAR_FRAME};
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct ExtractAlpha {}

impl ExtractAlpha {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into_layer(self) -> GenericLayer<Self> {
        GenericLayer::new(self, LayerTypeInfo::new("Extract Alpha").with_icon(LayerIcon::Transformer), &COLOR_FRAME, &SCALAR_FRAME)
    }
}

impl Component for ExtractAlpha {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!()
    }

    fn detach(&mut self) {
        fork_properties!();
    }
}

impl LayerCore for ExtractAlpha {
    type Input = Frame<ColorPixel>;
    type Output = Frame<ScalarPixel>;

    fn next(&mut self, input: Self::Input, _t: f64, _ctx: &PatternContext) -> Self::Output {
        input.into_iter()
            .map(|pixel| pixel.alpha)
            .collect()
    }
}