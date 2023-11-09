use dyn_clone::{clone_trait_object, DynClone};
use crate::pattern_builder::component::{Component, ComponentConfig};
use crate::pattern_builder::component::data::PixelFrame;

pub trait Filter: Component + DynClone {
    fn next_frame(&mut self, t: f64, active: PixelFrame) -> PixelFrame;
}
clone_trait_object!(Filter);

impl Component for Box<dyn Filter> {
    fn config(&self) -> &dyn ComponentConfig {
        self.as_ref().config()
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        self.as_mut().config_mut()
    }

    fn component_type(&self) -> &'static str {
        self.as_ref().component_type()
    }
}

impl Filter for Box<dyn Filter> {
    fn next_frame(&mut self, t: f64, active: PixelFrame) -> PixelFrame {
        self.as_mut().next_frame(t, active)
    }
}
