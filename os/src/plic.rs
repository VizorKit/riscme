use crate::addresses::PhysAddress;
use crate::direct::{mask_to, or_to, write_to, Write};

const PLIC_ORIGIN: PhysAddress = PhysAddress::new(0x0C00_0000);
const PLIC_PRIORITY: PhysAddress = PhysAddress::new(0x0C00_0004);
const PLIC_PENDING1: PhysAddress = PhysAddress::new(0x0C001_0000);
const PLIC_PENDING2: PhysAddress = PhysAddress::new(0x0C001_0004);
const PLIC_ENABLE1: PhysAddress = PhysAddress::new(0x0C00_2000);
const PLIC_ENABLE2: PhysAddress = PhysAddress::new(0x0C00_2004);
const PLIC_THRESHOLD: PhysAddress = PhysAddress::new(0x0C20_0000);

pub fn init() {}

pub fn set_priorities() -> () {}

pub fn enable_register(id: PlicID) {
    or_to(PLIC_ENABLE1, id);
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
        PlicID::Pwm0 => Priority::Four,
        PlicID::Pwm1 => Priority::Four,
        PlicID::Pwm2 => Priority::Four,
        PlicID::I2C => Priority::Four,
        _ => Priority::Three,
    }
}
#[derive(Clone, Copy)]
#[repr(C)]
#[rustfmt::skip]
pub enum PlicID {
    AonWdg    = 0b1,
    AonRtc    = 0b10,
    Uart0     = 0b100,
    Uart1     = 0b1000,
    Qspi0     = 0b1_0000,
    Spi1      = 0b10_0000,
    Spi2      = 0b100_0000,
    GpioStart = 0b1000_0000,
    /// not correct end. todo!(fix);
    GpioEnd   = 0b1_0000_0000,
    Pwm0      = 0b10_0000_0000,
    Pwm1      = 0b100_0000_0000,
    Pwm2      = 0b1000_0000_0000,
    I2C       = 0b1_0000_0000_0000,
}

impl Write for PlicID {
    fn get(&self) -> usize {
        *self as usize
    }
}
pub enum GpioPlicID {
    GpioStart = 0x08,
    GpioEnd = 0x27,
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

pub fn set_priority(id: PlicID, priority: Priority) {}
