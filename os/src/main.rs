#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(custom_test_frameworks, asm, naked_functions)]
#![test_runner(crate::test_runner)]

mod addresses;
mod cpu;
mod direct;
mod plic;
mod registers;
use addresses::{Offset, PhysAddress};
use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn eh_personality() {}

const __ROM_ORIGIN: PhysAddress = phys!(0x20010000);
const __ROM_LENGTH: Offset = m_offset!(0x1000);
const __RAM_ORIGIN: PhysAddress = phys!(0x80000000);
const __RAM_LENGTH: Offset = m_offset!(0x4000);
const __RESET_VECT: PhysAddress = phys!(0x10040000);

const __LOW_MEM: PhysAddress = __RAM_ORIGIN;
const __HIGH_MEM: PhysAddress = __RAM_ORIGIN.add(__RAM_LENGTH);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!("
la t0, _early_trap_vector_lbl
csrw mtvec, t0
li sp, {}", const __HIGH_MEM.value()
        );
    }
    abort();
}

#[naked]
#[no_mangle]
#[rustfmt::skip]
unsafe extern "C" fn _early_trap_vector() -> ! {
        asm!("
.align 2
_early_trap_vector_lbl:
    csrr t0, mcause
    csrr t1, mepc
    csrr t2, mtval
    j _early_trap_vector_lbl", options(noreturn))
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
            asm!("wfi");
        }
    }
}
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}
