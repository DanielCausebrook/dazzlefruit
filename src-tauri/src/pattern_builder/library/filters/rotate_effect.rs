use palette::Mix;
use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::ComponentInfo;
use crate::pattern_builder::component::filter::Filter;
use crate::pattern_builder::component::data::PixelFrame;
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::cloning::BoolProperty;
use crate::pattern_builder::component::property::num::{NumProperty, NumSlider};

#[derive(Clone)]
pub struct RotateEffect {
    info: ComponentInfo,
    offset: NumProperty<f64>,
    speed: NumProperty<f64>,
    smoothing: BoolProperty,
}

impl RotateEffect {
    pub fn new(offset: impl Into<NumProperty<f64>>, speed: impl Into<NumProperty<f64>>, smoothing: bool) -> Self {
        Self {
            info: ComponentInfo::new("Rotate Effect"),
            offset: offset.into().set_info(PropertyInfo::new("Offset"))
                .set_slider(Some(NumSlider::new(0.0..500.0, 1.0))),
            speed: speed.into().set_info(PropertyInfo::new("Speed"))
                .set_slider(Some(NumSlider::new(0.0..20.0, 0.1))),
            smoothing: BoolProperty::new(smoothing, PropertyInfo::new("Smoothing")),
        }
    }
}

impl_component!(self: RotateEffect, *self, "filter");

impl_component_config!(self: RotateEffect, self.info, [
    self.offset,
    self.speed,
    self.smoothing,
]);

impl Filter for RotateEffect {
    fn next_frame(&mut self, t: f64, mut active: PixelFrame) -> PixelFrame {
        let translation: f64 = ((self.speed.get() % active.len() as f64) * (t % active.len() as f64) + self.offset.get()) % active.len() as f64;
        let shift = translation.abs() as usize;
        if translation > 0.0 {
            active.rotate_right(shift);
        } else {
            active.rotate_left(shift);
        }
        if self.smoothing.get() {
            let blend = translation.abs().fract() as f32;
            let mut blended = vec![];
            if translation > 0.0 {
                for i in 0..active.len() {
                    let next = (i + 1) % active.len();
                    blended.push(active[i].mix(active[next], 1.0 - blend))
                }
            } else {
                for i in 0..active.len() {
                    let prev = (i + active.len() - 1) % active.len();
                    blended.push(active[i].premultiply().mix(active[prev].premultiply(), 1.0 - blend).unpremultiply())
                }
            }
            blended
        } else {
            active
        }
    }
}
