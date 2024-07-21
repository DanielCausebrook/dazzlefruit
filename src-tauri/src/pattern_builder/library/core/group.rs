use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{ColorPixel, Frame};
use crate::pattern_builder::component::layer::{DisplayPane, Layer, LayerCore, LayerIcon, LayerTypeInfo};
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::layer_stack::LayerStackPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Group {
    stack: Prop<LayerStack>,
}

impl Group {
    pub fn new() -> Self {
        Self::from(LayerStack::new())
    }

    pub fn from(stack: LayerStack) -> Self {
        Self {
            stack: LayerStackPropCore::new(stack).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }

    pub fn stack(&self) -> &Prop<LayerStack> {
        &self.stack
    }

    pub fn into_layer(self) -> Layer where Self: Sized {
        Layer::new_texture(self, LayerTypeInfo::new("Group").with_icon(LayerIcon::Group))
    }
}

impl LayerCore for Group {
    type Input = ();
    type Output = Frame<ColorPixel>;

    fn next(&mut self, _: (), t: f64, ctx: &PatternContext) -> Frame<ColorPixel> {
        let mut pixel_data: Frame<ColorPixel> = self.stack.write().next((), t, ctx)
            .unwrap_or_else(|_err| vec![].into());
        pixel_data.resize_with_empty(ctx.num_pixels());
        pixel_data
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.stack)
    }

    fn detach(&mut self) {
        fork_properties!(self.stack);
    }
}

