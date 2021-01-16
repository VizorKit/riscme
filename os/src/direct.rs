use core::ptr::{read_volatile, write_volatile};

pub trait Address {
    fn get(&self) -> usize;
}

pub trait Mask {
    fn get(&self) -> usize;
}
pub trait Write {
    fn get(&self) -> usize;
}

pub fn read_from<T: Address + Copy + 'static>(addr: T) -> usize {
    unsafe { read_volatile(addr.get() as *const usize) }
}
pub fn write_to<T: Address + Copy + 'static, V: Write + 'static>(addr: T, val: V) {
    unsafe { write_volatile(addr.get() as *mut usize, val.get()) }
}

pub fn mask_to<T: Address + Copy + 'static, U: Mask + 'static, V: Write + 'static>(
    addr: T,
    mask: U,
    value: V,
) {
    let mut copy = read_from(addr);
    copy &= mask.get();
    unsafe {
        write_volatile(addr.get() as *mut usize, copy | value.get() & !mask.get());
    }
}
pub fn or_to<T: Address + Copy + 'static, V: Write + 'static>(addr: T, val: V) {
    let mut copy = read_from(addr);
    copy |= val.get();
    unsafe {
        write_volatile(addr.get() as *mut usize, copy);
    }
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
