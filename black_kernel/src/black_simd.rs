#[repr(C, align(32))]
pub struct BlackAlignedBuffer {
    black_data: [u8; 4096],
}

impl BlackAlignedBuffer {
    pub fn black_zero_nt(&mut self) {
        let black_ptr = self.black_data.as_mut_ptr() as *mut u8;
        let black_len = self.black_data.len() / 32;

        unsafe {
            core::arch::asm!(
                "vpxor ymm0, ymm0, ymm0",
                "2:",
                "vmovntdq ymmword ptr [{0}], ymm0",
                "add {0}, 32",
                "dec {1}",
                "jnz 2b",
                "sfence",
                inout(reg) black_ptr => _,
                inout(reg) black_len => _,
                out("ymm0") _,
                options(nostack, preserves_flags)
            );
        }
    }
}
