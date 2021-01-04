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
