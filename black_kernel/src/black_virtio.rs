#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlackVirtqDesc {
    pub black_addr:  u64,
    pub black_len:   u32,
    pub black_flags: u16,
    pub black_next:  u16,
}
