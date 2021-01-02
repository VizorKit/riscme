#![no_std]
#![no_main]
#![feature(custom_test_frameworks, asm)]
#![test_runner(crate::test_runner)]
mod addresses {
    #[derive(Clone, Copy)]
    pub struct PhysAddr(u32);
    #[derive(Clone, Copy)]
    pub struct Offset(u32);

    impl PhysAddr {
        pub const fn new(addr: u32) -> Self {
            PhysAddr(addr)
        }

        pub const fn add_offset(self, offset: Offset) -> Self {
            PhysAddr(self.0 + offset.0)
        }

        pub const fn value(self) -> u32 {
            self.0
        }
    }

    impl Offset {
        pub const fn new(offset: u32) -> Self {
            Offset(offset)
        }
    }
}

use addresses::{Offset, PhysAddr};
use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn eh_personality() {}

const __ROM_ORIGIN: PhysAddr = PhysAddr::new(0x20010000);
const __ROM_LENGTH: Offset = Offset::new(0x1000);
const __RAM_ORIGIN: PhysAddr = PhysAddr::new(0x80000000);
const __RAM_LENGTH: Offset = Offset::new(0x4000);
const __RESET_VECT: PhysAddr = PhysAddr::new(0x10040000);

const __LOW_MEM: PhysAddr = __RAM_ORIGIN;
const __HIGH_MEM: PhysAddr = PhysAddr::new(__RAM_ORIGIN.add_offset(__RAM_LENGTH).value());

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!("lui sp, %pcrel_hi({})", const __HIGH_MEM.value());
    }
    loop {}
}

#[no_mangle]
extern "C" fn _set_mtvec() -> () {}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}