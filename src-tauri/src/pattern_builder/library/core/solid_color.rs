use palette::LinSrgba;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{DisplayPane, FrameSize, Pixel, PixelFrame};
use crate::pattern_builder::component::property::color::ColorPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::texture::Texture;

#[derive(Clone)]
pub struct SolidColor {
    color: Prop<LinSrgba>,
}

impl SolidColor {
    pub fn new(color: Pixel) -> Self {
        Self {
            color: ColorPropCore::new(color).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }
}

impl Component for SolidColor {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.color)
    }

    fn detach(&mut self) {
        fork_properties!(self.color);
    }
}

impl Texture for SolidColor {
    fn next_frame(&mut self, _t: f64, num_pixels: FrameSize) -> PixelFrame {
        vec![self.color.read().clone(); num_pixels as usize]
    }
}