use crate::pattern_builder::component::property::PropView;

pub mod data;
mod macros;
// pub mod shared_component;
pub mod property;
pub mod layer;

pub trait Component: Send + Sync + 'static {
    fn view_properties(&self) -> Vec<PropView>;
    fn detach(&mut self);
}

impl<T> Component for Box<T> where T: Component + ?Sized {
    fn view_properties(&self) -> Vec<PropView> {
        self.as_ref().view_properties()
    }
    fn detach(&mut self) {
        self.as_mut().detach()
    }
}
