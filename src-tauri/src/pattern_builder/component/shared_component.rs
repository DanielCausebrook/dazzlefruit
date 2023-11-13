use std::sync::Arc;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::{impl_component};
use crate::pattern_builder::component::{Component, ComponentConfig, ComponentInfo};
use crate::pattern_builder::component::property::Property;

#[derive(Clone)]
pub struct SharedComponent<T: Component + Clone> {
    info: ComponentInfo,
    component: Arc<RwLock<T>>,
}

impl<T: Component + Clone> SharedComponent<T> {
    pub fn new(component: T) -> Self {
        Self {
            info: ComponentInfo::new("Texture Reference"),
            component: Arc::new(RwLock::new(component)),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.component.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.component.write()
    }

    pub fn clone_inner(&self) -> T {
        self.component.read().clone()
    }
}

impl_component!(<T: Component + Clone> self: SharedComponent<T>, *self, self.component.read().component_type());

impl<T: Component + Clone> ComponentConfig for SharedComponent<T> {
    fn info(&self) -> &ComponentInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        &mut self.info
    }

    fn properties(&self) -> Vec<&dyn Property> {
        vec![]
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        vec![]
    }

    fn detach(&mut self) {
        self.info.detach()
    }
}