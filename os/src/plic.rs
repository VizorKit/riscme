use crate::addresses::PhysAddress;
use crate::direct::set_to;

const PLIC_ORIGIN: PhysAddress = PhysAddress::new(0x0C00_0000);
const PLIC_PRIORITY: PhysAddress = PhysAddress::new(0x0C00_0004);
const PLIC_PENDING1: PhysAddress = PhysAddress::new(0x0C001_0000);
const PLIC_PENDING2: PhysAddress = PhysAddress::new(0x0C001_0004);
const PLIC_ENABLE1: PhysAddress = PhysAddress::new(0x0C00_2000);
const PLIC_ENABLE2: PhysAddress = PhysAddress::new(0x0C00_2004);
const PLIC_THRESHOLD: PhysAddress = PhysAddress::new(0x0C20_0000);


pub struct Plic {
    threshold: Threshold,
    threshold_mask: Mask,
    
}

pub enum Threshold {
    One = 0x01,
    Two = 0x02,
    Three = 0x03,
    Four = 0x04,
    Five = 0x05,
    Six = 0x06,
    Seven = 0x07,
}

pub enum Enables {}

pub fn set_threshold(threshold: Threshold) -> () {
    set_to(&PLIC_THRESHOLD, threshold as usize, Shift { 0 });
}
