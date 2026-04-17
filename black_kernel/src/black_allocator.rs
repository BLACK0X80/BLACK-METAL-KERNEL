use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

const BLACK_HEAP_SIZE: usize = 1024 * 1024 * 4;

#[repr(align(4096))]
struct BlackHeap([u8; BLACK_HEAP_SIZE]);

static mut BLACK_HEAP: BlackHeap = BlackHeap([0u8; BLACK_HEAP_SIZE]);
static BLACK_OFFSET: AtomicUsize = AtomicUsize::new(0);

pub struct BlackBumpAllocator;

unsafe impl GlobalAlloc for BlackBumpAllocator {
    unsafe fn alloc(&self, black_layout: Layout) -> *mut u8 {
        let black_align = black_layout.align();
        let black_size  = black_layout.size();

        loop {
            let black_current = BLACK_OFFSET.load(Ordering::Relaxed);
            let black_aligned = (black_current + black_align - 1) & !(black_align - 1);
            let black_next    = black_aligned + black_size;

            if black_next > BLACK_HEAP_SIZE {
                return core::ptr::null_mut();
            }

            if BLACK_OFFSET
                .compare_exchange_weak(
                    black_current,
                    black_next,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return BLACK_HEAP.0.as_mut_ptr().add(black_aligned) as *mut u8;
            }
        }
    }

    unsafe fn dealloc(&self, _black_ptr: *mut u8, _black_layout: Layout) {}
}

#[global_allocator]
static BLACK_ALLOC: BlackBumpAllocator = BlackBumpAllocator;
