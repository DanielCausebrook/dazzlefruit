use std::mem;
use std::ops::{Deref, DerefMut};
use parking_lot::RwLock;
use tokio::sync::watch;

pub trait RWLockWatchSender<T> {
    fn read(&self) -> RWLockWatchReadGuard<'_, T>;
    fn write(&self) -> RWLockWatchWriteGuard<'_, T>;
}

impl<T> RWLockWatchSender<T> for watch::Sender<RwLock<T>> {
    fn read(&self) -> RWLockWatchReadGuard<T> {
        RWLockWatchReadGuard::new(self.borrow())
    }
    fn write(&self) -> RWLockWatchWriteGuard<T> {
        RWLockWatchWriteGuard::new(self.borrow())
    }
}

pub trait RWLockWatchReceiver<T> {
    fn read(&self) -> RWLockWatchReadGuard<T>;
    fn write(&self) -> RWLockWatchWriteGuard<T>;
}

impl<T> RWLockWatchReceiver<T> for watch::Receiver<RwLock<T>> {
    fn read(&self) -> RWLockWatchReadGuard<T> {
        RWLockWatchReadGuard::new(self.borrow())
    }
    fn write(&self) -> RWLockWatchWriteGuard<T> {
        RWLockWatchWriteGuard::new(self.borrow())
    }
}

pub struct RWLockWatchReadGuard<'a, T> {
    reference: watch::Ref<'a, RwLock<T>>,
}

impl<'a, T> RWLockWatchReadGuard<'a, T> {
    fn new(reference: watch::Ref<'a, RwLock<T>>) -> Self {
        mem::forget(reference.read());
        Self { reference }
    }
}

impl<T> Drop for RWLockWatchReadGuard<'_, T> {
    fn drop(&mut self) {
        unsafe { self.reference.force_unlock_read(); }
    }
}

impl<T> Deref for RWLockWatchReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.reference.data_ptr() }
    }
}

pub struct RWLockWatchWriteGuard<'a, T> {
    reference: watch::Ref<'a, RwLock<T>>,
}

impl<'a, T> RWLockWatchWriteGuard<'a, T> {
    fn new(reference: watch::Ref<'a, RwLock<T>>) -> Self {
        mem::forget(reference.write());
        Self { reference }
    }
}

impl<T> Drop for RWLockWatchWriteGuard<'_, T> {
    fn drop(&mut self) {
        unsafe { self.reference.force_unlock_write(); }
    }
}

impl<T> Deref for RWLockWatchWriteGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.reference.data_ptr() }
    }
}

impl<T> DerefMut for RWLockWatchWriteGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.reference.data_ptr() }
    }
}