#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(custom_test_frameworks, asm, llvm_asm)]
#![test_runner(crate::test_runner)]

mod addresses;
mod cpu;
mod registers;

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
        asm!("li sp, {}", const __HIGH_MEM.value());
    }
    abort();
}

#[no_mangle]
extern "C" fn _set_mtvect() -> () {
    unsafe {
        asm!(
            "
csrrci t0, mstatus, 3
la a0, _mtvect
csrrw t2, mtvec, a0
csrrci t1, mtvec, 1
csrrsi t0, mstatus, 3
"
        )
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn abort() -> ! {
    loop {
        unsafe {
            llvm_asm!("wfi"::::"volatile");
        }
    }
}
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}
