use crate::addresses::Address;
use core::ptr::{read_volatile, write_volatile};
#[derive(Copy, Clone)]
pub struct Mask(usize);
#[derive(Copy, Clone)]
pub struct Value(usize);
#[derive(Copy, Clone)]
pub struct Shift(usize);
impl Mask {
    pub const fn new(mask: usize) -> Self {
        Mask(mask)
    }
}
impl Value {
    pub const fn new(value: usize) -> Self {
        Value(value)
    }
}
impl Shift {
    pub const fn new(shift: usize) -> Self {
        Shift(shift)
    }
}
pub fn set_to<T: Address>(item: T, set_val: Value, shift_val: Shift) {
    unsafe {
        let ptr = item.value() as *mut usize;
        write_volatile(ptr, set_val.0 << shift_val.0);
    }
}

pub fn mask_to<T: Address>(item: T, mask_val: Mask, mut set_val: Value, shift_val: Shift) {
    unsafe {
        let ptr = item.value() as *mut usize;
        let mut copy = read_volatile(ptr);
        copy &= mask_val.0;
        set_val.0 &= !mask_val.0;
        write_volatile(ptr, copy | set_val.0 << shift_val.0);
    }
}
pub fn or_to<T: Address>(item: T, or_val: Value, shift_val: Shift) {
    unsafe {
        let ptr = item.value() as *mut usize;
        write_volatile(ptr, read_volatile(ptr) | or_val.0 << shift_val.0);
    }
}

fn read_from<T: Address>(item: T) -> usize {
    unsafe { read_volatile(item.value() as *const usize) }
}
