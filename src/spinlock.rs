/// A spinklock implementation for ensuring safe concurrent access to system data structures
///
/// By: Arthur Riechert
/// Date: Sun. Jan 18, 2026

use core::{
    ops::{
        Deref,
        DerefMut,
    },
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    cell::UnsafeCell,
    hint,
};

/// The following spinlock is a test and test-and-set (TTAS) lock.
///
/// Standard test-and-set locks suffers from cache contention.
/// Cache lines permit multiple readers and only a single writer,
/// so when a core would like to acquire a lock we must continuously
/// change which core that can write, leading to slow downs when multiple
/// threads are spinning on the same lock. The TTAS lock resolves this by
/// only doing a single write operation and only reading the flag while
/// spinning. This implementation also uses the x86 pause mechanism to
/// throttle CPU when spinning is detected.
///
/// Refer here for more info: https://rigtorp.se/spinlock/
#[derive(Debug, Default)]
pub struct SpinLock<T> {
    flag: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    pub fn new(item: T) -> Self {
        Self {
            flag: AtomicBool::new(false),
            // Use UnsafeCell for interior mutability.
            // This means that we can modify the data even with an immutable reference.
            // We need this for static references to ensure multiple parts
            // of our program can use this lock at runtime.
            // In this case, thread safety is manually guaranteed by 
            // the locking mechanisms, so we can do this.
            data: UnsafeCell::new(item),
        } 
    }

    pub fn acquire(&self) -> SpinLockGuard<T> {
        while self.flag.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            while self.flag.load(Ordering::Relaxed) {
                hint::spin_loop();
            }
        }

        SpinLockGuard::<T> { lock: self }
    }

    pub fn release(&self) {
        self.flag.store(false, Ordering::Release);
    }
}

/// We want to use a smart pointer pattern similar to the Box type,
/// so we implement the Deref and DerefMut to achieve this.
impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() } 
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
       unsafe { &mut *self.lock.data.get() } 
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        SpinLock::<T>::release(self.lock); 
    }
}
