use palette::{Hsla, IntoColor, LinSrgba, ShiftHue, Srgba};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{DisplayPane, Pixel, PixelFrame};
use crate::pattern_builder::component::property::color::ColorPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::layer::texture::Texture;
use crate::pattern_builder::math_functions::triangle_sin;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct ColorRange {
    color: Prop<LinSrgba>,
    variance: Prop<f32>,
    period: Prop<f32>,
    smoothing: Prop<f32>,
}

impl ColorRange {
    pub fn new(color: Pixel) -> Self {
        Self {
            color: ColorPropCore::new(color).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            variance: NumPropCore::new_slider(80.0, 0.0..255.0, 1.0).into_prop(PropertyInfo::new("Variance")),
            period: NumPropCore::new_slider(5.0, 0.0..20.0, 0.05).into_prop(PropertyInfo::new("Period")),
            smoothing: NumPropCore::new_slider(1.0, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Smoothing")),
        }
    }
}

impl Component for ColorRange {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.color, self.variance, self.period, self.smoothing)
    }

    fn detach(&mut self) {
        fork_properties!(self.color, self.variance, self.period, self.smoothing);
    }
}

impl Texture for ColorRange {
    fn next_frame(&mut self, t: f64, ctx: &PatternContext) -> PixelFrame {
        let hsla: Hsla = Srgba::from_linear(*self.color.read()).into_color();
        let hsla_mod = hsla.shift_hue(triangle_sin(*self.smoothing.read(),1.0, t as f32 / *self.period.read()) * *self.variance.read());
        let srgba: Srgba = hsla_mod.into_color();
        vec![srgba.into_linear(); ctx.num_pixels()].into()
    }
}
