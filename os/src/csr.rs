pub mod mtvec {
    #[inline(always)]
    pub fn set(trap: extern "C" fn()) {
        unsafe {
            asm!(
            "csrrci t0, mstatus, 1 << 3",
            "csrrw t2, mtvec, {}",
            "csrrci t1, mtvec, 1",
            "csrrsi t0, mstatus, 1 << 3",
            in(reg) trap)
        }
    }
}
