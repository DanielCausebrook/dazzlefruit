use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::frame::{ColorPixel, Frame};
use crate::pattern_builder::component::layer::{DisplayPane, LayerCore, LayerInfo, LayerType};
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::layer::standard_types::{COLOR_FRAME, VOID};
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::layer::texture::TextureLayer;
use crate::pattern_builder::component::property::layer_stack::LayerStackPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Group {
    stack: Prop<LayerStack<(), Frame<ColorPixel>>>,
}

impl Group {
    pub fn new() -> Self {
        Self::from(LayerStack::new(&VOID, &COLOR_FRAME))
    }

    pub fn from(stack: LayerStack<(), Frame<ColorPixel>>) -> Self {
        Self {
            stack: LayerStackPropCore::new(stack).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }

    pub fn stack(&self) -> &Prop<LayerStack<(), Frame<ColorPixel>>> {
        &self.stack
    }

    pub fn into_layer(self, info: LayerInfo) -> TextureLayer where Self: Sized {
        TextureLayer::new(self, info).set_layer_type(LayerType::Group)
    }
}

impl Component for Group {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.stack)
    }

    fn detach(&mut self) {
        fork_properties!(self.stack);
    }
}

impl LayerCore for Group {
    type Input = ();
    type Output = Frame<ColorPixel>;
    fn next(&mut self, _: (), t: f64, ctx: &PatternContext) -> Frame<ColorPixel> {
        let mut pixel_data = self.stack.write().next((), t, ctx)
            .unwrap_or_else(|_err| vec![].into());
        pixel_data.resize_with_empty(ctx.num_pixels());
        pixel_data
    }
}

