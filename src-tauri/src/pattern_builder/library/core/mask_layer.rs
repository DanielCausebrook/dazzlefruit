use itertools::Itertools;
use palette::WithAlpha;
use crate::pattern_builder::component::filter::Filter;
use crate::pattern_builder::component::basic_config::BasicConfig;
use crate::pattern_builder::component::{Component, ComponentConfig};
use crate::pattern_builder::component::data::PixelFrame;

#[derive(Clone)]
pub struct MaskLayer {
    config: BasicConfig,
    mask: Vec<f32>,
}

impl MaskLayer {
    pub fn new(mask: Vec<f32>) -> Self {
        MaskLayer{ config: BasicConfig::new("Mask", None), mask }
    }
}

impl Component for MaskLayer {
    fn config(&self) -> &dyn ComponentConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        &mut self.config
    }

    fn component_type(&self) -> &'static str { "filter" }
}

impl Filter for MaskLayer {
    fn next_frame(&mut self, _t: f64, active: PixelFrame) -> PixelFrame {
        self.mask
            .iter()
            .pad_using(active.len(), |_| &0f32)
            .zip(active)
            .map(|(mask, c)| c.with_alpha(c.alpha * mask))
            .collect()
    }
}
