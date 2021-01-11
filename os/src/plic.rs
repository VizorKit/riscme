use crate::addresses::PhysAddress;
// use crate::direct::{set_to, Shift};

const PLIC_ORIGIN: PhysAddress = PhysAddress::new(0x0C00_0000);
const PLIC_PRIORITY: PhysAddress = PhysAddress::new(0x0C00_0004);
const PLIC_PENDING1: PhysAddress = PhysAddress::new(0x0C001_0000);
const PLIC_PENDING2: PhysAddress = PhysAddress::new(0x0C001_0004);
const PLIC_ENABLE1: PhysAddress = PhysAddress::new(0x0C00_2000);
const PLIC_ENABLE2: PhysAddress = PhysAddress::new(0x0C00_2004);
const PLIC_THRESHOLD: PhysAddress = PhysAddress::new(0x0C20_0000);

pub struct Plic {
    threshold: Threshold,
}

impl Plic {
    pub fn init<F>(&self, func: F)
    where
        F: FnOnce() -> Plic,
    {
        func();
    }
}

pub enum PlicID {
    AonWdg = 0x01,
    AonRtc = 0x02,
    Uart0 = 0x03,
    Uart1 = 0x04,
    Qspio = 0x05,
    Spi1 = 0x06,
    Spi2 = 0x07,
    GpioStart = 0x08,
    GpioEnd = 0x27,
    Pwm0 = 0x28,
    Pwm1 = 0x29,
    Pwm2 = 0x2A,
    I2C = 0x2B,
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
