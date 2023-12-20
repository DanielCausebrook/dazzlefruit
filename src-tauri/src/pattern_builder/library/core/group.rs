use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{DisplayPane, PixelFrame};
use crate::pattern_builder::component::layer::filter::FilterLayer;
use crate::pattern_builder::component::layer::LayerInfo;
use crate::pattern_builder::component::layer::layer_stack::LayerStack;
use crate::pattern_builder::component::layer::standard_types::{PIXEL_FRAME, VOID};
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::layer::texture::{Texture, TextureLayer};
use crate::pattern_builder::component::property::layer_stack::LayerStackPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Group {
    stack: Prop<LayerStack<(), PixelFrame>>,
}

impl Group {
    pub fn new() -> Self {
        Self::from(LayerStack::new(&VOID, &PIXEL_FRAME))
    }

    pub fn from(stack: LayerStack<(), PixelFrame>) -> Self {
        Self {
            stack: LayerStackPropCore::new(stack).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }

    pub fn stack(&self) -> &Prop<LayerStack<(), PixelFrame>> {
        &self.stack
    }

    pub fn add_texture(&self, texture: TextureLayer) {
        self.stack.write().push(texture)
    }

    pub fn add_filter(&self, filter: FilterLayer) {
        self.stack.write().push(filter)
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

impl Texture for Group {
    fn next_frame(&mut self, t: f64, ctx: &PatternContext) -> PixelFrame {
        let mut pixel_data = self.stack.write().next((), t, ctx)
            .unwrap_or_else(|err| vec![].into());
        pixel_data.resize_with_transparent(ctx.num_pixels());
        pixel_data
    }

    fn into_layer(self, info: LayerInfo) -> TextureLayer where Self: Sized {
        TextureLayer::new(self, info, "group")
    }
}

