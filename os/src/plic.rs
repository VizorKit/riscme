use crate::addresses::{Offset, PhysAddress};
use crate::direct::{mask_to, or_to, write_to};
use crate::{m_offset, phys};

const PLIC_ORIGIN: PhysAddress = phys!(0x0C00_0000);
const PLIC_PRIORITY: PhysAddress = phys!(0x0C00_0004);
const PLIC_PENDING1: PhysAddress = phys!(0x0C001_0000);
const PLIC_PENDING2: PhysAddress = phys!(0x0C001_0004);
const PLIC_ENABLE1: PhysAddress = phys!(0x0C00_2000);
const PLIC_ENABLE2: PhysAddress = phys!(0x0C00_2004);
const PLIC_THRESHOLD: PhysAddress = phys!(0x0C20_0000);

pub fn init() {
    set_threshold();
    for n in 1..10 {
        or_to(
            PLIC_PRIORITY.add(m_offset!(4 * n)).value(),
            get_base_priorities(PlicID::create(n)) as usize,
        );
    }
    enable_register1(PlicEnable::AllUpper);
}

pub fn set_threshold() -> () {
    write_to(PLIC_THRESHOLD.value(), 4);
}

pub fn enable_register1(id: PlicEnable) {
    or_to(PLIC_ENABLE1.value(), id as usize);
}

fn get_base_priorities(id: PlicID) -> Priority {
    match id {
        PlicID::AonWdg => Priority::Seven,
        PlicID::AonRtc => Priority::Seven,
        PlicID::Uart0 => Priority::Six,
        PlicID::Uart1 => Priority::Six,
        PlicID::Qspi0 => Priority::Four,
        PlicID::Spi1 => Priority::Six,
        PlicID::Spi2 => Priority::Six,
        PlicID::Gpio0 => Priority::Five,
        PlicID::Gpio1 => Priority::Five,
        PlicID::Gpio2 => Priority::Five,
        PlicID::Pwm0 => Priority::Four,
        PlicID::Pwm1 => Priority::Four,
        PlicID::Pwm2 => Priority::Four,
        PlicID::I2C => Priority::Four,
    }
}

#[repr(C)]
#[rustfmt::skip]
pub enum PlicID {
    AonWdg    = 0x1,
    AonRtc    = 0x2,
    Uart0     = 0x3,
    Uart1     = 0x4,
    Qspi0     = 0x5,
    Spi1      = 0x6,
    Spi2      = 0x7,
    Gpio0     = 0x8,
    Gpio1     = 0x9,
    Gpio2     = 0x10,
    Pwm0      = 0x40,
    Pwm1      = 0x44,
    Pwm2      = 0x48,
    I2C       = 0x52
}
impl PlicID {
    pub fn create(val: usize) -> PlicID {
        match val {
            1 => PlicID::AonWdg,
            2 => PlicID::AonRtc,
            3 => PlicID::Uart0,
            4 => PlicID::Uart1,
            5 => PlicID::Qspi0,
            6 => PlicID::Spi1,
            7 => PlicID::Spi2,
            8 => PlicID::Gpio0,
            9 => PlicID::Gpio1,
            10 => PlicID::Gpio2,
            _ => PlicID::I2C,
        }
    }
}
#[repr(C)]
#[rustfmt::skip]
pub enum PlicEnable{
    AllUpper  = 0b11_1111_1111,
    AonWdg    = 0b1,
    AonRtc    = 0b10,
    Uart0     = 0b100,
    Uart1     = 0b1000,
    Qspi0     = 0b1_0000,
    Spi1      = 0b10_0000,
    Spi2      = 0b100_0000,
    Gpio0     = 0b1000_0000,
    Gpio1     = 0b1_0000_0000,
    Gpio2     = 0b10_0000_0000
}

pub enum Threshold {
    Zero = 0x00,
    One = 0x01,
    Two = 0x02,
    Three = 0x03,
    Four = 0x04,
    Five = 0x05,
    Six = 0x06,
    Seven = 0x07,
}

pub enum Priority {
    Zero = 0x00,
    One = 0x01,
    Two = 0x02,
    Three = 0x03,
    Four = 0x04,
    Five = 0x05,
    Six = 0x06,
    Seven = 0x07,
}
