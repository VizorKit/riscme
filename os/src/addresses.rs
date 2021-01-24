#[repr(transparent)]
pub struct PhysAddress(usize);
#[repr(transparent)]
pub struct Offset(usize);

impl PhysAddress {
    pub const fn new(val: usize) -> PhysAddress {
        PhysAddress(val)
    }
    pub const fn add(&self, offset: Offset) -> PhysAddress {
        PhysAddress(offset.0 + self.0)
    }
    pub const fn value(&self) -> usize {
        self.0
    }
}

impl Offset {
    pub const fn new(val: usize) -> Offset {
        Offset(val)
    }
}

#[macro_export]
macro_rules! phys {
    ($e:expr) => {
        PhysAddress::new($e);
    };
}

#[macro_export]
macro_rules! m_offset {
    ($e:expr) => {
        Offset::new($e);
    };
}
