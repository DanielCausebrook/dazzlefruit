use itertools::Itertools;
use palette::Mix;
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{ColorPixel, Frame, ScalarPixel};
use crate::pattern_builder::component::layer::{DisplayPane, Layer, LayerCore, LayerIcon, LayerTypeInfo};
use crate::pattern_builder::component::property::layer_stack::LayerStackPropCore;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct ScalarToDualTexture {
    texture_a: Prop<LayerStack>,
    texture_b: Prop<LayerStack>,
    lower_bound: Prop<f64>,
    upper_bound: Prop<f64>,
}

impl ScalarToDualTexture {
    pub fn new() -> Self {
        Self {
            texture_a: LayerStackPropCore::new(LayerStack::new()).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            texture_b: LayerStackPropCore::new(LayerStack::new()).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            lower_bound: NumPropCore::new(0.0).into_prop(PropertyInfo::new("Lower Bound")),
            upper_bound: NumPropCore::new(1.0).into_prop(PropertyInfo::new("Upper Bound")),
        }
    }

    pub fn texture_a(&self) -> &Prop<LayerStack> {
        &self.texture_a
    }

    pub fn texture_b(&self) -> &Prop<LayerStack> {
        &self.texture_b
    }

    pub fn lower_bound(&self) -> &Prop<f64> {
        &self.lower_bound
    }
    
    pub fn upper_bound(&self) -> &Prop<f64> {
        &self.upper_bound
    }
    
    pub fn into_layer(self) -> Layer {
        Layer::new(self, LayerTypeInfo::new("To Dual Texture").with_icon(LayerIcon::Transformer))
    }
}

impl LayerCore for ScalarToDualTexture {
    type Input = Frame<ScalarPixel>;
    
    type Output = Frame<ColorPixel>;

    fn next(&mut self, input: Frame<ScalarPixel>, t: f64, ctx: &PatternContext) -> Frame<ColorPixel> {
        let frame_a = self.texture_a.write().next((), t, ctx)
            .unwrap_or_else(|_err| Frame::<ColorPixel>::empty(ctx.num_pixels()));
        let frame_b = self.texture_b.write().next((), t, ctx)
            .unwrap_or_else(|_err| Frame::<ColorPixel>::empty(ctx.num_pixels()));
        let lower_bound = *self.lower_bound.read();
        let upper_bound = *self.upper_bound.read();
        frame_a.into_iter()
            .zip_eq(frame_b.into_iter())
            .zip_eq(input)
            .map(|((pixel_a, pixel_b), input_value)| {
                let amount = ((input_value - lower_bound) / (upper_bound - lower_bound)).clamp(0.0, 1.0);
                pixel_a.mix(pixel_b, amount)
            })
            .collect()
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.texture_a,
            self.texture_b,
            self.lower_bound,
            self.upper_bound,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.texture_a,
            self.texture_b,
            self.lower_bound,
            self.upper_bound,
        );
    }
}