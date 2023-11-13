use palette::WithAlpha;
use rand::distributions::{Distribution, Uniform};
use rand_distr::Poisson;

use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::ComponentInfo;
use crate::pattern_builder::component::data::{BlendMode, DisplayPane, FrameSize, PixelFrame};
use crate::pattern_builder::component::filter::Filter;
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::cloning::BlendModeProperty;
use crate::pattern_builder::component::property::num::{NumProperty, NumSlider};
use crate::pattern_builder::component::texture::{Texture, TextureProperty};
use crate::pattern_builder::library::filters::persistence_effect::{PersistenceEffect, PersistenceEffectConfig};

#[derive(Clone)]
pub struct SparklesConfig {
    info: ComponentInfo,
    texture: TextureProperty,
    density: NumProperty<f64>,
    decay_rate: NumProperty<f64>,
    blend_mode: BlendModeProperty,
    persistence_effect_config: PersistenceEffectConfig,
}

impl SparklesConfig {
    pub fn new(texture: impl Texture, density: impl Into<NumProperty<f64>>, decay_rate: f64) -> Self {
        let persistence_effect_config = PersistenceEffectConfig::new(decay_rate);
        Self {
            info: ComponentInfo::new("Sparkles"),
            texture: TextureProperty::new(Box::new(texture), PropertyInfo::new("Texture").display_pane(DisplayPane::Tree)),
            density: density.into().set_info(PropertyInfo::new("Density")).set_slider(Some(NumSlider::new(0.0..10.0, 0.1))),
            decay_rate: persistence_effect_config.decay_rate().clone().set_info(PropertyInfo::new("Decay Rate")),
            blend_mode: BlendModeProperty::default(),
            persistence_effect_config,
        }
    }
    pub fn blend_mode(&self) -> &BlendModeProperty {
        &self.blend_mode
    }
    pub fn into_texture(self) -> Sparkles {
        Sparkles::new(self)
    }
}

impl_component_config!(self: SparklesConfig, self.info, [
    self.texture,
    self.density,
    self.decay_rate,
]);

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

impl_component!(self: Sparkles, self.config, "pixel");

impl Texture for Sparkles {
    fn blend_mode(&self) -> BlendMode {
        self.config.blend_mode.get()
    }

    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let delta_t = (t - self.last_t).max(0.0);
        self.last_t = t;
        self.weights.resize(num_pixels as usize, 1.0);
        let texture_frame = self.config.texture.write().next_frame(t, num_pixels);
        let poisson = Poisson::new(delta_t * self.config.density.get() * num_pixels as f64).unwrap();
        let num_sparkles = poisson.sample(&mut rand::thread_rng()) + self.num_sparkles_remainder;
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