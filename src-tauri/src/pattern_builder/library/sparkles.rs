use palette::WithAlpha;
use rand::distributions::{Distribution, Uniform};
use rand_distr::Poisson;

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{DisplayPane, FrameSize, PixelFrame};
use crate::pattern_builder::component::filter::Filter;
use crate::pattern_builder::component::property::component::{TexturePropCore};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::computed::ComputedPropCore;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::texture::{Texture, TextureLayer};
use crate::pattern_builder::library::core::empty::Empty;
use crate::pattern_builder::library::filters::persistence_effect::{PersistenceEffect, PersistenceEffectConfig};

#[derive(Clone)]
pub struct SparklesConfig {
    texture: Prop<TextureLayer>,
    density: Prop<f64>,
    decay_rate: Prop<f64>,
    persistence_effect_config: PersistenceEffectConfig,
}

impl SparklesConfig {
    pub fn new(texture: TextureLayer, density: f64, decay_rate: f64) -> Self {
        let decay_rate_prop = NumPropCore::new_slider(decay_rate, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Decay Rate"));
        let config = Self {
            texture: TexturePropCore::new(texture).into_prop(PropertyInfo::new("Texture").set_display_pane(DisplayPane::Tree)),
            density: NumPropCore::new_slider(density, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Density")),
            decay_rate: decay_rate_prop.clone(),
            persistence_effect_config: PersistenceEffectConfig::new(decay_rate),
        };
        config.sync_decay_rate();
        config
    }
    pub fn into_texture(self) -> Sparkles {
        Sparkles::new(self)
    }
    
    fn sync_decay_rate(&self) {
        let decay_rate = self.decay_rate.clone();
        self.persistence_effect_config.decay_rate().replace_core(ComputedPropCore::new(move || *decay_rate.read()));
    }

    pub fn texture(&self) -> &Prop<TextureLayer> {
        &self.texture
    }
    
    pub fn density(&self) -> &Prop<f64> {
        &self.density
    }
    
    pub fn decay_rate(&self) -> &Prop<f64> {
        &self.decay_rate
    }
}

impl Default for SparklesConfig {
    fn default() -> Self {
        Self::new(Empty::new_texture_component(), 6.0, 1.5)
    }
}

#[derive(Clone)]
pub struct Sparkles {
    config: SparklesConfig,
    persistence: PersistenceEffect,
    last_t: f64,
    num_sparkles_remainder: f64,
    weights: Vec<f64>,
}

impl Sparkles {
    pub fn new(config: SparklesConfig) -> Self {
        Self {
            persistence: config.persistence_effect_config.clone().into_filter(),
            config,
            last_t: 0.0,
            num_sparkles_remainder: 0.0,
            weights: vec![],
        }
    }
}

impl Component for Sparkles {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.config.texture,
            self.config.density,
            self.config.decay_rate
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.config.texture,
            self.config.density,
            self.config.decay_rate
        );
        self.config.sync_decay_rate();
    }
}

impl Texture for Sparkles {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let delta_t = (t - self.last_t).max(0.0);
        self.last_t = t;
        self.weights.resize(num_pixels as usize, 1.0);
        let texture_frame = self.config.texture.write().next_frame(t, num_pixels);
        let num_sparkles = if let Ok(poisson) =
            Poisson::new(delta_t * *self.config.density.read() * num_pixels as f64) {
            poisson.sample(&mut rand::thread_rng()) + self.num_sparkles_remainder
        } else {
            self.num_sparkles_remainder
        };
        self.num_sparkles_remainder = num_sparkles.fract();
        let num_sparkles = num_sparkles.round() as i64;
        let mut pixels = vec![palette::named::BLACK.into_linear().transparent(); num_pixels as usize];
        for weight in self.weights.iter_mut() {
            *weight += delta_t;
        }
        let weighted_index = rand::distributions::WeightedIndex::new(self.weights.clone()).unwrap();
        for _ in 0..num_sparkles {
            let x = weighted_index.sample(&mut rand::thread_rng());
            let strength = Uniform::new(0f32, 1f32).sample(&mut rand::thread_rng());
            pixels[x] = texture_frame[x];
            pixels[x].alpha = pixels[x].alpha * strength;
            self.weights[x] /= 1.0 + strength as f64;
        }
        self.persistence.next_frame(t, pixels)
    }
}