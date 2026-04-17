pub const BLACK_PAGE_PRESENT:  u64 = 1 << 0;
pub const BLACK_PAGE_WRITABLE: u64 = 1 << 1;
pub const BLACK_PAGE_HUGE:     u64 = 1 << 7;

#[repr(C, align(4096))]
pub struct BlackPageTable {
    black_entries: [u64; 512],
}

impl BlackPageTable {
    pub const fn black_zeroed() -> Self {
        Self {
            black_entries: [0; 512],
        }
    }

    pub fn black_map_huge(&mut self, black_index: usize, black_phys: u64, black_flags: u64) {
        self.black_entries[black_index] = black_phys | black_flags | BLACK_PAGE_HUGE | BLACK_PAGE_PRESENT;
    }

    pub unsafe fn black_load(&self) {
        let black_addr = self as *const _ as u64;
        core::arch::asm!(
            "mov cr3, {}",
            in(reg) black_addr,
            options(nostack, preserves_flags)
        );
    }
}
