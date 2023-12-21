use rand::distributions::{Distribution, Uniform};
use rand_distr::Poisson;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::{LayerCore, LayerInfo};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::pattern_builder::component::property::computed::ComputedPropCore;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{Frame, ScalarPixel};
use crate::pattern_builder::component::layer::scalar_texture::ScalarTextureLayer;
use crate::pattern_builder::library::scalar_filters::persistence::Persistence;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Sparkles {
    density: Prop<f64>,
    decay_rate: Prop<f64>,
    persistence: Persistence<Frame<ScalarPixel>>,
    last_t: f64,
    num_sparkles_remainder: f64,
    weights: Vec<f64>,
}

impl Sparkles {
    pub fn new(density: f64, decay_rate: f64) -> Self {
        let new = Self {
            density: NumPropCore::new_slider(density, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Density")),
            decay_rate: NumPropCore::new_slider(decay_rate, 0.0..25.0, 0.1).into_prop(PropertyInfo::new("Decay Rate")),
            persistence: Persistence::new(decay_rate),
            last_t: 0.0,
            num_sparkles_remainder: 0.0,
            weights: vec![],
        };
        new.sync_decay_rate();
        new
    }

    fn sync_decay_rate(&self) {
        let decay_rate = self.decay_rate.clone();
        self.persistence.decay_rate().replace_core(ComputedPropCore::new(move || *decay_rate.read()));
    }

    pub fn density(&self) -> &Prop<f64> {
        &self.density
    }

    pub fn decay_rate(&self) -> &Prop<f64> {
        &self.decay_rate
    }

    pub fn into_layer(self, info: LayerInfo) -> ScalarTextureLayer {
        ScalarTextureLayer::new(self, info)
    }
}

impl Component for Sparkles {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.density,
            self.decay_rate
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.density,
            self.decay_rate
        );
        self.sync_decay_rate();
    }
}

impl LayerCore for Sparkles {
    type Input = ();
    type Output = Frame<ScalarPixel>;

    fn next(&mut self, _: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        let delta_t = (t - self.last_t).max(0.0);
        self.last_t = t;
        self.weights.resize(ctx.num_pixels(), 1.0);
        let num_sparkles = if let Ok(poisson) =
            Poisson::new(delta_t * *self.density.read() * ctx.num_pixels() as f64) {
            poisson.sample(&mut rand::thread_rng()) + self.num_sparkles_remainder
        } else {
            self.num_sparkles_remainder
        };
        self.num_sparkles_remainder = num_sparkles.fract();
        let num_sparkles = num_sparkles.round() as i64;
        let mut values = vec![0.0; ctx.num_pixels()];
        for weight in self.weights.iter_mut() {
            *weight += delta_t;
        }
        let weighted_index = rand::distributions::WeightedIndex::new(self.weights.clone()).unwrap();
        for _ in 0..num_sparkles {
            let x = weighted_index.sample(&mut rand::thread_rng());
            let strength = Uniform::new(0.0, 1.0).sample(&mut rand::thread_rng());
            values[x] = strength;
            self.weights[x] /= 1.0 + strength;
        }
        self.persistence.next(values.into(), t, ctx)
    }
}
