use palette::{Alpha, Hsl, IntoColor, ShiftHue, Srgba};
use palette::encoding::Srgb;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::frame::{ColorPixel, Frame};
use crate::pattern_builder::component::layer::{DisplayPane, LayerCore, LayerInfo};
use crate::pattern_builder::component::layer::texture::TextureLayer;
use crate::pattern_builder::component::property::color::ColorPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::math_functions::triangle_sin;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct ColorRange {
    color: Prop<ColorPixel>,
    variance: Prop<f64>,
    period: Prop<f64>,
    smoothing: Prop<f64>,
}

impl ColorRange {
    pub fn new(color: ColorPixel) -> Self {
        Self {
            color: ColorPropCore::new(color).into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
            variance: NumPropCore::new_slider(80.0, 0.0..255.0, 1.0).into_prop(PropertyInfo::new("Variance")),
            period: NumPropCore::new_slider(5.0, 0.0..20.0, 0.05).into_prop(PropertyInfo::new("Period")),
            smoothing: NumPropCore::new_slider(1.0, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Smoothing")),
        }
    }

    pub fn color(&self) -> &Prop<ColorPixel> {
        &self.color
    }

    pub fn variance(&self) -> &Prop<f64> {
        &self.variance
    }

    pub fn period(&self) -> &Prop<f64> {
        &self.period
    }

    pub fn into_layer(self, info: LayerInfo) -> TextureLayer {
        TextureLayer::new(self, info)
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

impl LayerCore for ColorRange {
    type Input = ();
    type Output = Frame<ColorPixel>;
    fn next(&mut self, _: (), t: f64, ctx: &PatternContext) -> Frame<ColorPixel> {
        let hsla: Alpha<Hsl<Srgb, f64>, f64> = Srgba::from_linear(*self.color.read()).into_color();
        let hsla_mod: Alpha<Hsl<Srgb, f64>, f64> = hsla.shift_hue(triangle_sin(*self.smoothing.read(),1.0, t / *self.period.read()) * *self.variance.read());
        let srgba: Srgba<f64> = hsla_mod.into_color();
        vec![srgba.into_linear(); ctx.num_pixels()].into()
    }
}
