use core::ptr::{read_volatile, write_volatile};
pub fn read_from(addr: usize) -> usize {
    unsafe { read_volatile(addr as *const usize) }
}
pub fn write_to(addr: usize, val: usize) {
    unsafe { write_volatile(addr as *mut usize, val) }
}

pub fn mask_to(addr: usize, mask: usize, value: usize) {
    let mut copy = read_from(addr);
    copy &= mask;
    unsafe {
        write_volatile(addr as *mut usize, copy | value & !mask);
    }
}
pub fn or_to(addr: usize, val: usize) {
    let mut copy = read_from(addr);
    copy |= val;
    unsafe {
        write_volatile(addr as *mut usize, copy);
    }
}
