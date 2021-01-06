use crate::addresses::PhysAddr;

const PLIC_ORIGIN: PhysAddr = PhysAddr::new(0x0C00_0000);
const PLIC_PENDING1: PhysAddr = PhysAddr::new(0x0C001_0000);
const PLIC_PENDING2: PhysAddr = PhysAddr::new(0x0C001_0004);
const PLIC_ENABLE1: PhysAddr = PhysAddr::new(0x0C00_2000);
const PLIC_ENABLE2: PhysAddr = PhysAddr::new(0x0C00_2004);
const PLIC_THRESHOLD: PhysAddr = PhysAddr::new(0x0C20_0000);

pub enum Threshold {
    One = 0b001,
    Two = 0b010,
    Three = 0b011,
    Four = 0b100,
    Five = 0b101,
    Six = 0b110,
    Seven = 0b111,
}
pub fn set_threshold(threshold: Threshold) -> () {
    PLIC_THRESHOLD.write_to(threshold as usize);
}
