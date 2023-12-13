use palette::Mix;

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::PixelFrame;
use crate::pattern_builder::component::layer::filter::Filter;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct RotateEffect {
    offset: Prop<f64>,
    speed: Prop<f64>,
    smoothing: Prop<bool>,
}

impl RotateEffect {
    pub fn new(offset: f64, speed: f64, smoothing: bool) -> Self {
        Self {
            offset: NumPropCore::new_slider(offset, 0.0..500.0, 1.0).into_prop(PropertyInfo::new("Offset")),
            speed: NumPropCore::new_slider(speed, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Speed")),
            smoothing: RawPropCore::new(smoothing).into_prop(PropertyInfo::new("Smoothing")),
        }
    }
    
    pub fn offset(&self) -> &Prop<f64> {
        &self.offset
    }
    
    pub fn speed(&self) -> &Prop<f64> {
        &self.speed
    }
    
    pub fn smoothing(&self) -> &Prop<bool> {
        &self.smoothing
    }
}

impl Component for RotateEffect {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.offset, self.speed, self.smoothing)
    }

    fn detach(&mut self) {
        fork_properties!(self.offset, self.speed, self.smoothing);
    }
}

impl Filter for RotateEffect {
    fn next_frame(&mut self, t: f64, mut active: PixelFrame, _ctx: &PatternContext) -> PixelFrame {
        let translation: f64 = ((*self.speed.read() % active.len() as f64) * (t % active.len() as f64) + *self.offset.read()) % active.len() as f64;
        let shift = translation.abs() as usize;
        if translation > 0.0 {
            active.rotate_right(shift);
        } else {
            active.rotate_left(shift);
        }
        if *self.smoothing.read() {
            let blend = translation.abs().fract() as f32;
            let next_index_addition = if translation > 0.0 { 1 } else { active.len() - 1 };
            (0..active.len()).map(|i| {
                let next = (i + next_index_addition) % active.len();
                // Was active[i].premultiply().mix(active[prev].premultiply(), 1.0 - blend).unpremultiply()
                active[i].mix(active[next], 1.0 - blend)
            }).collect()
        } else {
            active
        }
    }
}
