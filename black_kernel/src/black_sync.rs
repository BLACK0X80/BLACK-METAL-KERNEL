use core::sync::atomic::{AtomicBool, Ordering};

#[repr(C)]
pub struct BlackSpinlock {
    black_flag: AtomicBool,
}

impl BlackSpinlock {
    pub const fn black_new() -> Self {
        BlackSpinlock {
            black_flag: AtomicBool::new(false),
        }
    }

    #[inline(always)]
    pub fn black_acquire(&self) {
        while self.black_flag
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.black_flag.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }
        }
    }

    #[inline(always)]
    pub fn black_release(&self) {
        self.black_flag.store(false, Ordering::Release);
    }
}

unsafe impl Send for BlackSpinlock {}
unsafe impl Sync for BlackSpinlock {}
