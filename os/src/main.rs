#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(custom_test_frameworks, asm, llvm_asm)]
#![test_runner(crate::test_runner)]
mod registers {
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    pub enum REGISTERS {
        x0 = 0,
        ra,
        sp,
        gp,
        tp,
        t0,
        t1,
        t2,
        s0,
        s1,
        a0, /* x10 */
        a1,
        a2,
        a3,
        a4,
        a5,
        a6,
        a7,
        s2,
        s3,
        s4, /* x20 */
        s5,
        s6,
        s7,
        s8,
        s9,
        s10,
        s11,
        t3,
        t4,
        t5, /* x30 */
        t6,
    }
}
mod cpu {
    pub enum CpuMode {
        User = 0,
        Supervisor = 1,
        Machine = 3,
    }
}
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
        asm!("li sp, {}", const __HIGH_MEM.value());
    }
    abort();
}

#[no_mangle]
extern "C" fn _set_mtvect() -> () {
    unsafe {
        asm!(
            "
csrci t0, mstatus, 3
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
