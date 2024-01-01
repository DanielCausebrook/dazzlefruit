use num_traits::Zero;
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::layer::{LayerCore, LayerIcon, LayerTypeInfo};
use crate::pattern_builder::component::layer::generic::GenericLayer;
use crate::pattern_builder::component::layer::io_type::IOType;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Stutter<T> where T: Clone + Send + Sync + 'static {
    period: Prop<f64>,
    show_for_config: Option<(Prop<f64>, fn(&PatternContext) -> T)>,
    last_frame_t: f64,
    frame: Option<T>,
}

impl<T> Stutter<T> where T: Clone + Send + Sync + 'static {
    pub fn new(period: f64) -> Self {
        Self {
            period: NumPropCore::new_slider(period, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Period")),
            show_for_config: None,
            last_frame_t: 0.0,
            frame: None,
        }
    }

    pub fn new_partially_empty(period: f64, show_for: f64, empty_fn: fn(&PatternContext) -> T) -> Self {
        Self {
            period: NumPropCore::new_slider(period, 0.0..10.0, 0.1).into_prop(PropertyInfo::new("Period")),
            show_for_config: Some((
                NumPropCore::new_slider(show_for, 0.0..1.0, 0.05).into_prop(PropertyInfo::new("Show For")),
                empty_fn
            )),
            last_frame_t: 0.0,
            frame: None,
        }
    }

    pub fn into_layer(self, io_type: &'static IOType<T>) -> GenericLayer<Self> {
        GenericLayer::new(self, LayerTypeInfo::new("Stutter").with_icon(LayerIcon::Filter), io_type, io_type)
    }
}

impl<T> Component for Stutter<T> where T: Clone + Send + Sync + 'static {
    fn view_properties(&self) -> Vec<PropView> {
        if let Some((prop, _)) = &self.show_for_config {
            view_properties!(
                self.period,
                prop,
            )
        } else {
            view_properties!(
                self.period
            )
        }
    }

    fn detach(&mut self) {
        fork_properties!(
            self.period,
        );
        if let Some((prop, _)) = &mut self.show_for_config {
            fork_properties!(*prop);
        }
    }
}

impl<T> LayerCore for Stutter<T> where T: Clone + Send + Sync + 'static {
    type Input = T;
    type Output = T;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        if self.period.read().is_zero() {
            self.frame = None;
            self.last_frame_t = t;
            return input;
        }
        let mut delta_t = t - self.last_frame_t;
        if delta_t > *self.period.read() {
            self.frame = None;
            self.last_frame_t = t;
            delta_t = 0.0;
        }
        if let Some(empty_fn) = self.show_for_config.as_ref().and_then(|(show_for, empty_fn)| {
            if delta_t.is_zero() || delta_t / *self.period.read() < show_for.read().clamp(0.0, 1.0) {
                None
            } else {
                Some(empty_fn)
            }
        }) {
            empty_fn(ctx)
        } else {
            if let Some(frame) = &self.frame {
                frame.clone()
            } else {
                self.frame = Some(input.clone());
                input
            }
        }
    }
}
