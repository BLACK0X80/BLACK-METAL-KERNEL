pub unsafe fn black_kernel_init() -> ! {
    let mut black_pml4 = crate::black_paging::BlackPageTable::black_zeroed();
    black_pml4.black_map_huge(0, 0, crate::black_paging::BLACK_PAGE_WRITABLE);
    black_pml4.black_load();

    core::arch::asm!("sti", options(nomem, nostack));

    loop {
        core::arch::asm!("hlt", options(nomem, nostack));
    }
}
