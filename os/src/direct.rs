use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

#[repr(transparent)]
pub struct Volatile<T, V>(UnsafeCell<T>, PhantomData<V>);
pub struct Mask<T, U, V>(Volatile<T, V>, PhantomData<U>);

impl<T: Copy + 'static, U: Copy + 'static> Volatile<T, U> {
    pub fn read_from(&self) -> T {
        unsafe { self.0.get().read_volatile() }
    }
    pub fn write_to(&self, val: U) {
        unsafe {
            let ptr = self.0.get().write_volatile(val);
        }
    }
}

impl<
        T: BitOrAssign<U>
            + BitOrAssign<T>
            + BitOrAssign<V>
            + BitAndAssign<U>
            + BitAndAssign<V>
            + BitAndAssign<T>
            + BitOr
            + BitAnd
            + Not
            + Copy
            + 'static,
        U: BitOrAssign + BitAndAssign + BitOr + BitAnd + Not + Copy + 'static,
        V: BitOrAssign + BitAndAssign + BitOr + BitAnd + Not + Copy + 'static,
    > Mask<T, U, V>
{
    pub fn mask_to(self, mask: U, mut value: V) {
        unsafe {
            let mut copy = self.0.read_from();
            copy &= mask;
            value &= !mask;
            self.0.write_to(copy | value)
        }
    }
}
impl<
        T: BitOrAssign<V> + BitOrAssign<T> + BitAndAssign + BitOr + BitAnd + Not + Copy + 'static,
        U: BitOrAssign + BitAndAssign + BitOr + BitAnd + Not + Copy + 'static,
        V: BitOrAssign + BitAndAssign + BitOr + BitAnd + Not + Copy + 'static,
    > Mask<T, U, V>
{
    pub fn or_to(self, value: V) {
        let mut copy: T = self.0.read_from();
        copy |= value;
        self.0.write_to(copy);
    }
}
