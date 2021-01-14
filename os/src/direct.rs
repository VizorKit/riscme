use crate::addresses::PhysAddress;
use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};

#[repr(transparent)]
pub struct Volatile<T>(UnsafeCell<T>);
#[repr(transparent)]
pub struct Maskable<U>(Volatile<U>);
unsafe impl<T> Sync for Volatile<T> {}
unsafe impl<T> Send for Volatile<T> {}

impl<T: Copy + 'static> Volatile<T> {
    pub fn read_from(&self) -> T {
        unsafe {
            let ptr = &self.0.get();
            read_volatile(*ptr)
        }
    }
    pub fn write_to(&self, val: T) {
        unsafe {
            let ptr = &self.0.get();
            write_volatile(*ptr, val)
        }
    }
}

pub fn mask_to(&self, val: T, mask: usize, shift: usize) {
    unsafe {
        let ptr = &self.0.get();
        let mut copy = read_volatile(*ptr);
        copy &= mask;
        write_volatile(*ptr, copy | (val & (!mask)) << shift);
    }
}
pub fn or_to(&self, val: T) {
    unsafe {
        let ptr = &self.0.get();
        write_volatile(*ptr, read_volatile(*ptr) | val)
    }
}

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
