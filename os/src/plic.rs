use crate::addresses::PhysAddress;
use crate::direct::{mask_to, set_to, or_to};

const PLIC_ORIGIN: PhysAddress = PhysAddress::new(0x0C00_0000);
const PLIC_PRIORITY: PhysAddress = PhysAddress::new(0x0C00_0004);
const PLIC_PENDING1: PhysAddress = PhysAddress::new(0x0C001_0000);
const PLIC_PENDING2: PhysAddress = PhysAddress::new(0x0C001_0004);
const PLIC_ENABLE1: PhysAddress = PhysAddress::new(0x0C00_2000);
const PLIC_ENABLE2: PhysAddress = PhysAddress::new(0x0C00_2004);
const PLIC_THRESHOLD: PhysAddress = PhysAddress::new(0x0C20_0000);

pub fn init() {
    set_to(PLIC_THRESHOLD, Threshold::Four as usize, 0);
    or_to(PLIC_ENABLE1, 1, PlicID::AonWdg as usize);
    set_priorities();
}

pub fn set_priorities() -> () {
    or_to(PLIC_ENABLE1, 1, ;
    }

    pub fn enable_register(id: PlicID) -> Priority {
        
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
pub enum PlicID {
    AonWdg = 0x01,
    AonRtc = 0x02,
    Uart0 = 0x03,
    Uart1 = 0x04,
    Qspi0 = 0x05,
    Spi1 = 0x06,
    Spi2 = 0x07,
    GpioStart = 0x08,
    GpioEnd = 0x27,
    Pwm0 = 0x28,
    Pwm1 = 0x29,
    Pwm2 = 0x2A,
    I2C = 0x2B,
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
