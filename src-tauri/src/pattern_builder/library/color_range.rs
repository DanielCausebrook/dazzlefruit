use std::iter::repeat;
use palette::{Hsla, IntoColor, ShiftHue, Srgba, WithAlpha};
use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::ComponentInfo;
use crate::pattern_builder::component::data::{BlendMode, DisplayPane, FrameSize, Pixel, PixelFrame};
use crate::pattern_builder::component::property::cloning::{BlendModeProperty, ColorProperty};
use crate::pattern_builder::component::property::num::{NumProperty, NumSlider};
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::texture::Texture;
use crate::pattern_builder::math_functions::triangle_sin;

#[derive(Clone)]
pub struct ColorRange {
    info: ComponentInfo,
    color: ColorProperty,
    variance: NumProperty<f32>,
    period: NumProperty<f32>,
    smoothing: NumProperty<f32>,
}

impl ColorRange {
    pub fn new(color: Pixel) -> Self {
        Self {
            info: ComponentInfo::new("Color Range"),
            color: ColorProperty::new(color, PropertyInfo::unnamed().display_pane(DisplayPane::Tree)),
            variance: NumProperty::new(80.0, PropertyInfo::new("Variance")).set_slider(Some(NumSlider::new(0.0..255.0, 1.0))),
            period: NumProperty::new(5.0, PropertyInfo::new("Period")).set_slider(Some(NumSlider::new(0.0..20.0, 0.05))),
            smoothing: NumProperty::new(1.0, PropertyInfo::new("Smoothing")).set_slider(Some(NumSlider::new(0.0..20.0, 0.1))),
        }
    }
}

impl_component!(self: ColorRange, *self, "pixel");

impl_component_config!(self: ColorRange, self.info, [
    self.color,
    self.variance,
    self.period,
]);

impl Texture for ColorRange {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let hsla: Hsla = Srgba::from_linear(self.color.get()).into_color();
        let hsla_mod = hsla.shift_hue(triangle_sin(self.smoothing.get(),1.0, t as f32 / self.period.get()) * self.variance.get());
        let srgba: Srgba = hsla_mod.into_color();
        repeat(srgba.into_linear()).take(num_pixels as usize).collect()
    }
}
