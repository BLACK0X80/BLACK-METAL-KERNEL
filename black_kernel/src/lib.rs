#![no_std]
#![feature(asm_const)]
#![feature(allocator_api)]
#![feature(naked_functions)]

pub mod black_sync;
pub mod black_allocator;
pub mod black_interrupts;
pub mod black_simd;
pub mod black_virtio;
pub mod black_mutex;
pub mod black_paging;
pub mod black_init;
