use palette::Mix;
use crate::pattern_builder::component::{ComponentInfo, ComponentConfig, Component};
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

impl ComponentConfig for RotateEffect {
    fn info(&self) -> &ComponentInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        &mut self.info
    }

    fn properties(&self) -> Vec<&dyn Property> {
        vec![
            &self.offset,
            &self.speed,
            &self.smoothing,
        ]
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        vec![
            &mut self.offset,
            &mut self.speed,
            &mut self.smoothing,
        ]
    }
}

impl Component for RotateEffect {
    fn config(&self) -> &dyn ComponentConfig {
        self
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        self
    }

    fn component_type(&self) -> &'static str {
        "filter"
    }
}

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
