#[inline]
pub fn nop() {
    // no operations
    unsafe { asm!("nop" :::: "volatile") };
}

#[inline]
pub fn wfe() {
    // wait for events
    unsafe { asm!("wfe" :::: "volatile") };
}

#[inline]
pub fn wfi() {
    // wait for interrupts
    unsafe { asm!("wfi" :::: "volatile") };
}
