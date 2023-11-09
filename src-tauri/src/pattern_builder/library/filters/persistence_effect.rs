use palette::WithAlpha;
use crate::pattern_builder::component::{Component, ComponentConfig, ComponentInfo};
use crate::pattern_builder::component::data::{BlendMode, Frame, PixelFrame};
use crate::pattern_builder::component::filter::Filter;
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::num::{NumProperty, NumSlider};

#[derive(Clone)]
pub struct PersistenceEffectConfig {
    info: ComponentInfo,
    decay_rate: NumProperty<f64>,
}

impl PersistenceEffectConfig {
    pub fn new(decay_rate: impl Into<NumProperty<f64>>) -> Self {
        Self {
            info: ComponentInfo::new("Persistence Effect"),
            decay_rate: decay_rate.into().set_info(PropertyInfo::new("Decay Rate"))
                .set_slider(Some(NumSlider::new(0.0..20.0, 0.1, ))),
        }
    }

    pub fn into_filter(self) -> PersistenceEffect {
        PersistenceEffect::new(self)
    }

    pub fn decay_rate(&self) -> NumProperty<f64> {
        self.decay_rate.clone()
    }
}

impl ComponentConfig for PersistenceEffectConfig {
    fn info(&self) -> &ComponentInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        &mut self.info
    }

    fn properties(&self) -> Vec<&dyn Property> {
        vec![&self.decay_rate]
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        vec![
            &mut self.decay_rate
        ]
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
    fn config(&self) -> &dyn ComponentConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        &mut self.config
    }

    fn component_type(&self) -> &'static str {
        "filter"
    }
}

impl Filter for PersistenceEffect {
    fn next_frame(&mut self, t: f64, active: PixelFrame) -> PixelFrame {
        self.pixel_data.resize_with(active.len(), || palette::named::BLACK.with_alpha(0.0).into_linear());

        let delta_t = (t - self.last_t).max(0.0);
        self.last_t = t;

        for pixel in self.pixel_data.iter_mut() {
            pixel.alpha = (pixel.alpha - (self.config.decay_rate.get() * delta_t) as f32).clamp(0.0, 1.0)
        }

        self.pixel_data = active.blend(self.pixel_data.clone(), BlendMode::Normal);
        self.pixel_data.clone()
    }
}