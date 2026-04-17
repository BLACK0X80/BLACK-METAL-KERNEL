#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlackInterruptFrame {
    pub black_rip:    u64,
    pub black_cs:     u64,
    pub black_rflags: u64,
    pub black_rsp:    u64,
    pub black_ss:     u64,
}

#[naked]
pub unsafe extern "C" fn black_page_fault_handler() {
    core::arch::asm!(
        "push rbp",
        "push r15",
        "push r14",
        "push r13",
        "push r12",
        "push r11",
        "push r10",
        "push r9",
        "push r8",
        "push rdi",
        "push rsi",
        "push rdx",
        "push rcx",
        "push rbx",
        "push rax",
        "mov rdi, rsp",
        "call black_page_fault_router",
        "pop rax",
        "pop rbx",
        "pop rcx",
        "pop rdx",
        "pop rsi",
        "pop rdi",
        "pop r8",
        "pop r9",
        "pop r10",
        "pop r11",
        "pop r12",
        "pop r13",
        "pop r14",
        "pop r15",
        "pop rbp",
        "add rsp, 8",
        "iretq",
        options(noreturn)
    );
}

#[no_mangle]
extern "C" fn black_page_fault_router(_black_frame: *const BlackInterruptFrame) {
    let black_cr2: u64;
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) black_cr2);
    }
    if black_cr2 == 0 {
        panic!("black ptr fault");
    }
}
