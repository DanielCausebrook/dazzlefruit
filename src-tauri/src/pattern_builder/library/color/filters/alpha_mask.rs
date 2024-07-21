use itertools::Itertools;
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{ColorPixel, Frame, ScalarPixel};
use crate::pattern_builder::component::layer::{DisplayPane, Layer, LayerCore, LayerTypeInfo};
use crate::pattern_builder::component::property::layer_stack::LayerStackPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct AlphaMask {
    mask: Prop<LayerStack>,
}

impl AlphaMask {
    pub fn new() -> Self {
        Self {
            mask: LayerStackPropCore::new(LayerStack::new()).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }

    pub fn stack(&self) -> &Prop<LayerStack> {
        &self.mask
    }

    pub fn into_layer(self) -> Layer {
        Layer::new_filter(self, LayerTypeInfo::new("Alpha Mask"))
    }
}

impl LayerCore for AlphaMask {
    type Input = Frame<ColorPixel>;
    type Output = Frame<ColorPixel>;

    fn next(&mut self, mut active: Frame<ColorPixel>, t: f64, ctx: &PatternContext) -> Frame<ColorPixel> {
        let mask: Frame<ScalarPixel> = self.mask.write().next((), t, ctx)
            .unwrap_or_else(|_err| Frame::empty(ctx.num_pixels()));
        for (pixel, value) in active.iter_mut().zip_eq(mask) {
            pixel.alpha = pixel.alpha * value;
        }
        active
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.mask)
    }

    fn detach(&mut self) {
        fork_properties!(self.mask);
    }
}