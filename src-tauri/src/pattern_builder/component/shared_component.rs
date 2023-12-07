use std::sync::Arc;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::pattern_builder::component::{Component, ComponentInfo};

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