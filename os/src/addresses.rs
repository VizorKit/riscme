use core::ptr::{read_volatile, write_volatile};

#[derive(Clone, Copy)]
pub struct PhysAddr(usize);
#[derive(Clone, Copy)]
pub struct Offset(usize);

impl PhysAddr {
    pub const fn new(addr: usize) -> Self {
        PhysAddr(addr)
    }

    pub const fn add_offset(self, offset: Offset) -> Self {
        PhysAddr(self.0 + offset.0)
    }

    pub const fn value(self) -> usize {
        self.0
    }
    pub fn set_to(self, set_val: usize) -> () {
        unsafe {
            let ptr = self.value() as *mut usize;
            write_volatile(ptr, set_val);
        }
    }
    pub fn mask_to(self, mask_val: usize, mut set_val: usize) -> () {
        unsafe {
            let ptr = self.value() as *mut usize;
            let mut copy = read_volatile(ptr);
            copy &= mask_val;
            set_val &= !mask_val;
            write_volatile(ptr, copy | set_val);
        }
    }
    pub fn or_to(self, or_val: usize) -> () {
        unsafe {
            let ptr = self.value() as *mut usize;
            write_volatile(ptr, read_volatile(ptr) | or_val);
        }
    }
    pub fn read_from(self) -> usize {
        unsafe { read_volatile(self.value() as *const usize) }
    }
}

impl Offset {
    pub const fn new(offset: usize) -> Self {
        Offset(offset)
    }
}
