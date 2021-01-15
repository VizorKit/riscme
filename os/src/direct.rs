use core::ptr::{read_volatile, write_volatile};

pub trait Address {
    const ADDR: usize;
    fn get(&self) -> usize {
        Self::ADDR
    }
}

pub trait Mask {
    const MASK: usize;
}
pub trait Write {
    const WRITE: usize;
    fn get(&self) -> usize {
        Self::WRITE
    }
}

pub fn read_from<T: Address + 'static>(addr: T) -> usize {
    unsafe { read_volatile(addr.get() as *const usize) }
}
pub fn write_to<T: Address + 'static, V: Write + 'static>(addr: T, val: V) {
    unsafe { write_volatile(addr.get() as *mut usize, val.get()) }
}
// impl<
//         T: Address> Mask<T, U, V>
// {
//     pub fn mask_to(self, mask: U, mut value: V) {
//         unsafe {
//             let mut copy = self.0.read_from();
//             copy &= mask;
//             value &= !mask;
//             self.0.write_to(copy | value)
//         }
//     }
// }
// impl<
//         T: BitOrAssign<V> + BitOrAssign<T> + BitAndAssign + BitOr + BitAnd + Not + Copy + 'static,
//         U: BitOrAssign + BitAndAssign + BitOr + BitAnd + Not + Copy + 'static,
//         V: BitOrAssign + BitAndAssign + BitOr + BitAnd + Not + Copy + 'static,
//     > Mask<T, U, V>
// {
//     pub fn or_to(self, value: V) {
//         let mut copy: T = self.0.read_from();
//         copy |= value;
//         self.0.write_to(copy);
//     }
// }
