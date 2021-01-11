pub trait Address {
    fn value(self) -> usize;
}

#[derive(Clone, Copy)]
pub struct PhysAddress(usize);
#[derive(Clone, Copy)]
pub struct Offset(usize);

impl PhysAddress {
    pub const fn new(addr: usize) -> Self {
        PhysAddress(addr)
    }
    pub const fn add(self, offset: Offset) -> Self {
        PhysAddress(self.0 + offset.0)
    }
    pub const fn sub(self, offset: Offset) -> Self {
        PhysAddress(self.0 - offset.0)
    }
    pub const fn value(self) -> usize {
        self.0
    }
}

impl Offset {
    pub const fn new(offset: usize) -> Self {
        Offset(offset)
    }
}
