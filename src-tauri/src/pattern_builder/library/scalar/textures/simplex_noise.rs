use nalgebra_glm::DVec3;
use noise::{NoiseFn, OpenSimplex};
use rand::random;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{Frame, ScalarPixel};
use crate::pattern_builder::component::layer::{LayerCore, LayerInfo};
use crate::pattern_builder::component::layer::scalar_texture::ScalarTextureLayer;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::num_vec::NumVecPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct SimplexNoise {
    flow_speed: Prop<f64>,
    scale: Prop<DVec3>,
    travel_vel: Prop<DVec3>,
    simplex_noise: OpenSimplex,
}

impl SimplexNoise {
    pub fn new(flow_speed: f64) -> Self {
        Self {
            flow_speed: NumPropCore::new_slider(flow_speed, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Flow Speed")),
            scale: NumVecPropCore::new_slider(DVec3::repeat(1.0), 0.0..1.0, 0.02).into_prop(PropertyInfo::new("Scale")),
            travel_vel: NumVecPropCore::new_slider(DVec3::repeat(0.0), -100.0..100.0, 0.25).into_prop(PropertyInfo::new("Travel Velocity")),
            simplex_noise: OpenSimplex::new(random()),
        }
    }

    pub fn flow_speed(&self) -> &Prop<f64> {
        &self.flow_speed
    }

    pub fn scale(&self) -> &Prop<DVec3> {
        &self.scale
    }

    pub fn travel_vel(&self) -> &Prop<DVec3> {
        &self.travel_vel
    }

    pub fn into_layer(self, layer_info: LayerInfo) -> ScalarTextureLayer {
        ScalarTextureLayer::new(self, layer_info)
    }
}

impl Component for SimplexNoise {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.flow_speed,
            self.scale,
            self.travel_vel,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.flow_speed,
            self.scale,
            self.travel_vel,
        );
    }
}

impl LayerCore for SimplexNoise {
    type Input = ();
    type Output = Frame<ScalarPixel>;

    fn next(&mut self, _: (), t: f64, ctx: &PatternContext) -> Self::Output {
        (0..ctx.num_pixels())
            .map(|i| {
                let pos = ctx.position_map().pos(i).unwrap();
                let noise_pos = (pos - t * *self.travel_vel.read()).component_mul(&*self.scale.read());
                self.simplex_noise.get([
                    t * *self.flow_speed.read(),
                    noise_pos.x,
                    noise_pos.y,
                    noise_pos.z,
                ])
            })
            .collect()
    }
}