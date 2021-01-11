use crate::addresses::PhysAddress;
use core::ptr::{read_volatile, write_volatile};

pub fn set_to(item: PhysAddress, set_val: usize, shift_val: usize) {
    unsafe {
        let ptr = item.value() as *mut usize;
        write_volatile(ptr, set_val << shift_val);
    }
}

pub fn mask_to(item: PhysAddress, mask_val: usize, set_val: usize, shift_val: usize) {
    unsafe {
        let ptr = item.value() as *mut usize;
        let mut copy = read_volatile(ptr);
        copy &= mask_val;
        write_volatile(ptr, copy | (set_val & (!mask_val)) << shift_val);
    }
}
pub fn or_to(item: PhysAddress, or_val: usize, shift_val: usize) {
    unsafe {
        let ptr = item.value() as *mut usize;
        write_volatile(ptr, read_volatile(ptr) | or_val << shift_val);
    }
}

fn read_from(item: PhysAddress) -> usize {
    unsafe { read_volatile(item.value() as *const usize) }
}
