use crate::addresses::PhysAddr;

const PLIC_ORIGIN: PhysAddr = PhysAddr::new(0x0C00_0000);
const PLIC_PENDING1: PhysAddr = PhysAddr::new(0x0C001_0000);
const PLIC_PENDING2: PhysAddr = PhysAddr::new(0x0C001_0004);
const PLIC_ENABLE1: PhysAddr = PhysAddr::new(0x0C00_2000);
const PLIC_ENABLE2: PhysAddr = PhysAddr::new(0x0C00_2004);
const PLIC_THRESHOLD: PhysAddr = PhysAddr::new(0x0C20_0000);

pub enum Threshold {
    One = 0x01,
    Two = 0x02,
    Three = 0x03,
    Four = 0x04,
    Five = 0x05,
    Six = 0x06,
    Seven = 0x07,
}
pub fn set_threshold(threshold: Threshold) -> () {
    PLIC_THRESHOLD.set_to(threshold as usize);
}
