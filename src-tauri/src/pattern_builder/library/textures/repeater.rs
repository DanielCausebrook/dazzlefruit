use std::iter::repeat_with;
use crate::pattern_builder::component::data::{DisplayPane, PixelFrame};
use crate::pattern_builder::component::property::component::TexturePropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::layer::texture::{Texture, TextureLayer};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Repeater {
    texture: Prop<TextureLayer>,
    pixels_per_repeat: Prop<usize>,
}

impl Repeater {
    pub fn new(pixels_per_repeat: usize, texture: TextureLayer) -> Self {
        Self {
            texture: TexturePropCore::new(texture).into_prop(PropertyInfo::new("Texture").set_display_pane(DisplayPane::Tree)),
            pixels_per_repeat: NumPropCore::new(pixels_per_repeat).into_prop(PropertyInfo::new("Pixels per Repeat")),
        }
    }
    
    pub fn texture(&self) -> &Prop<TextureLayer> {
        &self.texture
    }
    
    pub fn pixels_per_repeat(&self) -> &Prop<usize> {
        &self.pixels_per_repeat
    }
}

impl Component for Repeater {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties![
            self.texture,
            self.pixels_per_repeat,
        ]
    }

    fn detach(&mut self) {
        fork_properties!(
            self.texture,
            self.pixels_per_repeat,
        );
    }
}

impl Texture for Repeater {
    fn next_frame(&mut self, t: f64, ctx: &PatternContext) -> PixelFrame {
        let repeating_fragment = ctx.slice(0..*self.pixels_per_repeat().read());
        let mini_frame = self.texture.write().next_frame(t, &repeating_fragment);
        repeat_with(|| mini_frame.clone()).flatten().take(ctx.num_pixels()).collect()
    }
}
