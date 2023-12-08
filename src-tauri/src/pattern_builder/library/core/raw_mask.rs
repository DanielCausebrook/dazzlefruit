use itertools::Itertools;
use palette::WithAlpha;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::filter::Filter;
use crate::pattern_builder::component::data::PixelFrame;
use crate::pattern_builder::component::property::PropView;

#[derive(Clone)]
pub struct RawMask {
    mask: Vec<f32>,
}

impl RawMask {
    pub fn new(mask: Vec<f32>) -> Self {
        RawMask { mask }
    }
}

impl Component for RawMask {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!()
    }

    fn detach(&mut self) {
        fork_properties!();
    }
}

impl Filter for RawMask {
    fn next_frame(&mut self, _t: f64, active: PixelFrame) -> PixelFrame {
        self.mask
            .iter()
            .pad_using(active.len(), |_| &0f32)
            .zip(active)
            .map(|(mask, c)| c.with_alpha(c.alpha * mask))
            .collect()
    }
}
