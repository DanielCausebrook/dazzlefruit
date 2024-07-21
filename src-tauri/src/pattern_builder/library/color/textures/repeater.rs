use std::iter::repeat_with;
use std::marker::PhantomData;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{Frame, Pixel};
use crate::pattern_builder::component::layer::{DisplayPane, Layer, LayerCore, LayerTypeInfo};
use crate::pattern_builder::component::layer::io_type::DynType;
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::property::layer_stack::LayerStackPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Repeater<T> where T: Pixel, Frame<T>: DynType {
    layer_stack: Prop<LayerStack>,
    pixels_per_repeat: Prop<usize>,
    phantom_data: PhantomData<T>,
}

impl<T> Repeater<T> where T: Pixel, Frame<T>: DynType {
    pub fn new(pixels_per_repeat: usize) -> Self {
        Self {
            layer_stack: LayerStackPropCore::new(LayerStack::new()).into_prop(PropertyInfo::new("Texture").set_display_pane(DisplayPane::Tree)),
            pixels_per_repeat: NumPropCore::new(pixels_per_repeat).into_prop(PropertyInfo::new("Pixels per Repeat")),
            phantom_data: PhantomData::default(),
        }
    }
    
    pub fn stack(&self) -> &Prop<LayerStack> {
        &self.layer_stack
    }
    
    pub fn pixels_per_repeat(&self) -> &Prop<usize> {
        &self.pixels_per_repeat
    }

    pub fn into_layer(self) -> Layer {
        Layer::new_texture(self, LayerTypeInfo::new("Repeater"))
    }
}

impl<T> LayerCore for Repeater<T> where T: Pixel, Frame<T>: DynType {
    type Input = ();
    type Output = Frame<T>;
    fn next(&mut self, _: (), t: f64, ctx: &PatternContext) -> Frame<T> {
        let repeating_fragment = ctx.slice(0..*self.pixels_per_repeat().read());
        let mini_frame: Frame<T> = self.layer_stack.write().next((), t, &repeating_fragment)
            .unwrap_or_else(|_err| Frame::empty(ctx.num_pixels()));
        repeat_with(|| mini_frame.clone()).flatten().take(ctx.num_pixels()).collect()
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties![
            self.layer_stack,
            self.pixels_per_repeat,
        ]
    }

    fn detach(&mut self) {
        fork_properties!(
            self.layer_stack,
            self.pixels_per_repeat,
        );
    }
}
