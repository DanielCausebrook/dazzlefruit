use nalgebra_glm::smoothstep;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::{LayerCore, LayerTypeInfo};
use crate::pattern_builder::component::layer::scalar_texture::ScalarTextureLayer;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{Frame, ScalarPixel};
use crate::pattern_builder::math_functions::triangle_sin;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Pulse {
    period: Prop<f64>,
    width: Prop<f64>,
    smoothness: Prop<f64>,
}

impl Pulse {
    pub fn new(period: f64, width: f64, smoothness: f64) -> Self {
        Self {
            period: NumPropCore::new_slider(period, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Period")),
            width: NumPropCore::new_slider(width, 1.0..20.0, 0.2).into_prop(PropertyInfo::new("Width")),
            smoothness: NumPropCore::new_slider(smoothness, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Smoothness")),
        }
    }

    pub fn period(&self) -> &Prop<f64> {
        &self.period
    }

    pub fn width(&self) -> &Prop<f64> {
        &self.width
    }

    pub fn smoothness(&self) -> &Prop<f64> {
        &self.smoothness
    }

    pub fn into_layer(self) -> ScalarTextureLayer {
        ScalarTextureLayer::new(self, LayerTypeInfo::new("Pulse"))
    }
}

impl Component for Pulse {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties![
            self.period,
            self.width,
            self.smoothness,
        ]
    }

    fn detach(&mut self) {
        fork_properties!(
            self.period,
            self.width,
            self.smoothness,
        );
    }
}

impl LayerCore for Pulse {
    type Input = ();
    type Output = Frame<ScalarPixel>;
    fn next(&mut self, _: (), t: f64, ctx: &PatternContext) -> Frame<ScalarPixel> {
        let pulse_pos = 0.5 * (triangle_sin(*self.smoothness.read(), *self.period.read(), t) + 1.0) * (ctx.num_pixels() as f64 - *self.width.read());
        let step1 = [pulse_pos - 0.5, pulse_pos + 0.5];
        let step2 = [pulse_pos + *self.width.read() - 0.5, pulse_pos + *self.width.read() + 0.5];
        (0..ctx.num_pixels()).into_iter()
            .map(|x| x as f64)
            .map(|x| {
                smoothstep(step1[0], step1[1], x) - smoothstep(step2[0], step2[1], x)
            })
            .collect()
    }
}
