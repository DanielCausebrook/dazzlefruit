use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::math_functions::skew_sin;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::frame::{Frame, ScalarPixel};
use crate::pattern_builder::component::layer::{LayerCore, LayerInfo};
use crate::pattern_builder::component::layer::scalar_texture::ScalarTextureLayer;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Wave {
    wave1_speed: Prop<f64>,
    wave1_scale: Prop<f64>,
    wave1_skew: Prop<f64>,
    wave2_speed: Prop<f64>,
    wave2_scale: Prop<f64>,
    wave2_skew: Prop<f64>,
}

impl Wave {
    pub fn new() -> Self {
        Self {
            wave1_speed: NumPropCore::new_slider(9.0, -30.0..30.0, 0.1).into_prop(PropertyInfo::new("Wave 1 Speed")),
            wave1_scale: NumPropCore::new_slider(28.5, 0.0..50.0, 0.5).into_prop(PropertyInfo::new("Wave 1 Scale")),
            wave1_skew: NumPropCore::new_slider(0.6, -1.0..1.0, 0.01).into_prop(PropertyInfo::new("Wave 1 Skew")),
            wave2_speed: NumPropCore::new_slider(-11.5, -30.0..30.0, 0.1).into_prop(PropertyInfo::new("Wave 2 Speed")),
            wave2_scale: NumPropCore::new_slider(34.0, 0.0..50.0, 0.5).into_prop(PropertyInfo::new("Wave 2 Scale")),
            wave2_skew: NumPropCore::new_slider(-0.5, -1.0..1.0, 0.01).into_prop(PropertyInfo::new("Wave 2 Skew")),
        }
    }

    pub fn into_layer(self, layer_info: LayerInfo) -> ScalarTextureLayer {
        ScalarTextureLayer::new(self, layer_info)
    }
}

impl Component for Wave {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.wave1_speed,
            self.wave1_scale,
            self.wave1_skew,
            self.wave2_speed,
            self.wave2_scale,
            self.wave2_skew,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.wave1_speed,
            self.wave1_scale,
            self.wave1_skew,
            self.wave2_speed,
            self.wave2_scale,
            self.wave2_skew,
        );
    }
}

impl LayerCore for Wave {
    type Input = ();
    type Output = Frame<ScalarPixel>;
    fn next(&mut self, _: (), t: f64, ctx: &PatternContext) -> Frame<ScalarPixel> {
        (0..ctx.num_pixels()).map(|x_int| {
            let x = x_int as f64;
            // let t = t + ((x/10.0 + t).sin() / 2.0);
            let mut wave1_val = skew_sin(*self.wave1_skew.read(), 1.0, (x + *self.wave1_speed.read() * t) / *self.wave1_scale.read());
            let mut wave2_val = skew_sin(*self.wave2_skew.read(), 1.0, (x + *self.wave2_speed.read() * t) / *self.wave2_scale.read());
            wave1_val = wave1_val / 2.0 + 0.5;
            wave2_val = wave2_val / 2.0 + 0.5;
            (wave1_val + wave2_val) / 2.0
        }).collect()
    }
}

