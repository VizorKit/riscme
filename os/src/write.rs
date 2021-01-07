use crate::addresses::Address;
use core::ptr::{read_volatile, write_volatile};

pub fn set_to<T: Address>(item: &T, set_val: usize) {
    unsafe {
        let ptr = item.value() as *mut usize;
        write_volatile(ptr, set_val);
    }
}
pub fn mask_to<T: Address>(item: &T, mask_val: usize, mut set_val: usize) {
    unsafe {
        let ptr = item.value() as *mut usize;
        let mut copy = read_volatile(ptr);
        copy &= mask_val;
        set_val &= !mask_val;
        write_volatile(ptr, copy | set_val);
    }
}
pub fn or_to<T: Address>(item: &T, or_val: usize) {
    unsafe {
        let ptr = item.value() as *mut usize;
        write_volatile(ptr, read_volatile(ptr) | or_val);
    }
}
fn read_from<T: Address>(item: &T) -> usize {
    unsafe { read_volatile(item.value() as *const usize) }
}
