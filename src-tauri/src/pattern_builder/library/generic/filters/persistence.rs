use crate::pattern_builder::component::layer::{Layer, LayerCore, LayerTypeInfo};
use crate::pattern_builder::component::property::{Prop, PropCore, PropertyInfo, PropView};
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{Blend, BlendMode, Opacity};
use crate::pattern_builder::component::layer::io_type::DynType;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Persistence<T> where T: Blend + Opacity + Clone {
    state: Option<T>,
    decay_rate: Prop<f64>,
    last_t: f64,
}

impl<T> Persistence<T> where T: Blend + Opacity + DynType + Clone {
    pub fn new(decay_rate: f64) -> Self {
        Self {
            state: None,
            decay_rate: NumPropCore::new_slider(decay_rate, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Decay Rate")),
            last_t: 0.0,
        }
    }

    pub fn decay_rate(&self) -> &Prop<f64> {
        &self.decay_rate
    }
    
    pub fn into_layer(self) -> Layer where T: Send + Sync + 'static {
        Layer::new_filter(self, LayerTypeInfo::new("Persistence"))
    }
}

impl<T> LayerCore for Persistence<T> where T: Blend + Opacity + DynType + Clone {
    type Input = T;
    type Output = T;

    fn next(&mut self, active: Self::Input, t: f64, _ctx: &PatternContext) -> Self::Output {
        let delta_t = (t - self.last_t).max(0.0);
        self.last_t = t;

        let decay_rate = *self.decay_rate.read();
        let result = match self.state.take() {
            Some(state) => active.blend(state.decay_opacity(decay_rate, delta_t), BlendMode::Normal),
            None => active,
        };
        self.state = Some(result.clone());

        result
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.decay_rate,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.decay_rate,
        );
    }
}