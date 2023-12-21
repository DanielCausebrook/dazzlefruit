use itertools::Itertools;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{ColorPixel, Frame, ScalarPixel};
use crate::pattern_builder::component::layer::{DisplayPane, LayerCore, LayerInfo, LayerType};
use crate::pattern_builder::component::layer::generic::GenericLayer;
use crate::pattern_builder::component::layer::standard_types::{PIXEL_FRAME, SCALAR_FRAME, VOID};
use crate::pattern_builder::component::property::layer_stack::LayerStackPropCore;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct ScalarToTexture {
    texture: Prop<LayerStack<(), Frame<ColorPixel>>>,
    lower_bound: Prop<f64>,
    upper_bound: Prop<f64>,
}

impl ScalarToTexture {
    pub fn new() -> Self {
        Self::from(LayerStack::new(&VOID, &PIXEL_FRAME))
    }
    
    pub fn from(texture: LayerStack<(), Frame<ColorPixel>>) -> Self {
        Self {
            texture: LayerStackPropCore::new(texture).into_prop(PropertyInfo::new("Texture").set_display_pane(DisplayPane::Tree)),
            lower_bound: NumPropCore::new(0.0).into_prop(PropertyInfo::new("Lower Bound")),
            upper_bound: NumPropCore::new(1.0).into_prop(PropertyInfo::new("Upper Bound")),
        }
    }
    
    pub fn texture(&self) -> &Prop<LayerStack<(), Frame<ColorPixel>>> {
        &self.texture
    }
    
    pub fn lower_bound(&self) -> &Prop<f64> {
        &self.lower_bound
    }
    
    pub fn upper_bound(&self) -> &Prop<f64> {
        &self.upper_bound
    }
    
    pub fn into_layer(self, info: LayerInfo) -> GenericLayer<Self> {
        GenericLayer::new(self, info, &SCALAR_FRAME, &PIXEL_FRAME)
            .set_layer_type(LayerType::Transformer)
    }
}

impl Component for ScalarToTexture {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.texture,
            self.lower_bound,
            self.upper_bound,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.texture,
            self.lower_bound,
            self.upper_bound,
        );
    }
}

impl LayerCore for ScalarToTexture {
    type Input = Frame<ScalarPixel>;
    
    type Output = Frame<ColorPixel>;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        let mut frame = self.texture.write().next((), t, ctx)
            .unwrap_or_else(|err| Frame::empty(ctx.num_pixels()));
        let lower_bound = *self.lower_bound.read();
        let upper_bound = *self.upper_bound.read();
        for (pixel, input_value) in frame.iter_mut().zip_eq(input) {
            let amount = ((input_value - lower_bound) / (upper_bound - lower_bound)).clamp(0.0, 1.0);
            pixel.alpha = pixel.alpha * amount;
        }
        frame
    }
}