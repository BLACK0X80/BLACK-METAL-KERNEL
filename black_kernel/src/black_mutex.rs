use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use crate::black_sync::BlackSpinlock;

pub struct BlackMutex<T> {
    black_lock: BlackSpinlock,
    black_data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for BlackMutex<T> {}
unsafe impl<T: Send> Send for BlackMutex<T> {}

impl<T> BlackMutex<T> {
    pub const fn black_new(black_val: T) -> Self {
        Self {
            black_lock: BlackSpinlock::black_new(),
            black_data: UnsafeCell::new(black_val),
        }
    }

    pub fn black_lock(&self) -> BlackMutexGuard<'_, T> {
        self.black_lock.black_acquire();
        BlackMutexGuard {
            black_mutex: self,
        }
    }
}

pub struct BlackMutexGuard<'a, T> {
    black_mutex: &'a BlackMutex<T>,
}

impl<T> Deref for BlackMutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.black_mutex.black_data.get() }
    }
}

impl<T> DerefMut for BlackMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.black_mutex.black_data.get() }
    }
}

impl<T> Drop for BlackMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.black_mutex.black_lock.black_release();
    }
}
