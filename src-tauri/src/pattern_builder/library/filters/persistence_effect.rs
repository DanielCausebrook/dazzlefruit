use palette::WithAlpha;

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{BlendMode, Frame, PixelFrame};
use crate::pattern_builder::component::filter::Filter;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;

#[derive(Clone)]
pub struct PersistenceEffectConfig {
    decay_rate: Prop<f64>,
}

impl PersistenceEffectConfig {
    pub fn new(decay_rate: f64) -> Self {
        Self {
            decay_rate: NumPropCore::new_slider(decay_rate, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Decay Rate")),
        }
    }

    pub fn into_filter(self) -> PersistenceEffect {
        PersistenceEffect::new(self)
    }

    pub fn decay_rate(&self) -> &Prop<f64> {
        &self.decay_rate
    }
}

#[derive(Clone)]
pub struct PersistenceEffect {
    pixel_data: PixelFrame,
    config: PersistenceEffectConfig,
    last_t: f64,
}

impl PersistenceEffect {
    pub fn new(config: PersistenceEffectConfig) -> Self {
        PersistenceEffect {
            pixel_data: vec![],
            config,
            last_t: 0.0,
        }
    }
}

impl Component for PersistenceEffect {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties![
            self.config.decay_rate,
        ]
    }

    fn detach(&mut self) {
        fork_properties!(
            self.config.decay_rate,
        );
    }
}

impl Filter for PersistenceEffect {
    fn next_frame(&mut self, t: f64, active: PixelFrame) -> PixelFrame {
        self.pixel_data.resize_with(active.len(), || palette::named::BLACK.with_alpha(0.0).into_linear());

        let delta_t = (t - self.last_t).max(0.0);
        self.last_t = t;

        for pixel in self.pixel_data.iter_mut() {
            pixel.alpha = (pixel.alpha - (*self.config.decay_rate.read() * delta_t) as f32).clamp(0.0, 1.0)
        }

        self.pixel_data = active.blend(self.pixel_data.clone(), BlendMode::Normal);
        self.pixel_data.clone()
    }
}