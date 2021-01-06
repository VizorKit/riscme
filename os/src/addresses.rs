#[derive(Clone, Copy)]
pub struct PhysAddr(usize);
#[derive(Clone, Copy)]
pub struct Offset(usize);

impl PhysAddr {
    pub const fn new(addr: usize) -> Self {
        PhysAddr(addr)
    }

    pub const fn add_offset(self, offset: Offset) -> Self {
        PhysAddr(self.0 + offset.0)
    }

    pub const fn value(self) -> usize {
        self.0
    }
    pub fn write_to(self, set_to: usize) -> () {
        unsafe {
            let ptr = self.value() as *mut usize;
            *ptr = set_to;
        }
    }
    pub fn read_from(self) -> usize {
        unsafe { *(self.value() as *const usize) }
    }
}

impl Offset {
    pub const fn new(offset: usize) -> Self {
        Offset(offset)
    }
}
