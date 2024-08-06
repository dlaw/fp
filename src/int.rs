use core::fmt::Debug;
use core::ops::{Shl, Shr};

pub trait Int:
    Clone
    + Copy
    + Debug
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
    + Sized
{
    const MIN: Self;
    const ZERO: Self;
    const MAX: Self;
    const BITS: u32;
    const SIGNED: bool;
    type Signed: Int;
    fn as_signed(self) -> Self::Signed;
    unsafe fn unchecked_add(self, other: Self) -> Self;
    unsafe fn unchecked_sub(self, other: Self) -> Self;
    unsafe fn from_f32_unchecked(val: f32) -> Self;
    unsafe fn from_f64_unchecked(val: f64) -> Self;
    fn into_f32(self) -> f32;
    fn into_f64(self) -> f64;
}

macro_rules! int_impl {
    ($T:ty, $Signed:ty) => {
        impl Int for $T {
            const MIN: $T = <$T>::MIN;
            const ZERO: $T = 0;
            const MAX: $T = <$T>::MAX;
            const BITS: u32 = <$T>::BITS;
            #[allow(unused_comparisons)]
            const SIGNED: bool = <$T>::MIN < 0;
            type Signed = $Signed;
            fn as_signed(self) -> $Signed {
                self as $Signed
            }
            unsafe fn unchecked_add(self, other: Self) -> Self {
                unsafe { self.unchecked_add(other) }
            }
            unsafe fn unchecked_sub(self, other: Self) -> Self {
                unsafe { self.unchecked_sub(other) }
            }
            unsafe fn from_f32_unchecked(val: f32) -> Self {
                unsafe { val.to_int_unchecked() }
            }
            unsafe fn from_f64_unchecked(val: f64) -> Self {
                unsafe { val.to_int_unchecked() }
            }
            fn into_f32(self) -> f32 {
                self as f32
            }
            fn into_f64(self) -> f64 {
                self as f64
            }
        }
    };
}

int_impl!(i8, i8);
int_impl!(u8, i8);
int_impl!(i16, i16);
int_impl!(u16, i16);
int_impl!(i32, i32);
int_impl!(u32, i32);
int_impl!(i64, i64);
int_impl!(u64, i64);
int_impl!(i128, i128);
int_impl!(u128, i128);
int_impl!(isize, isize);
int_impl!(usize, isize);

/// Every integer is also a fixed-point number, considered to have
/// the maximum number of bits and zero shift.
impl<T: Int> crate::Num for T {
    type Raw = Self;
    const BITS: u32 = Self::BITS;
    const SHIFT: i32 = 0;
    const MIN: Self = Self::MIN;
    const ZERO: Self = Self::ZERO;
    const MAX: Self = Self::MAX;
    const SIGNED: bool = Self::SIGNED;
    unsafe fn new_unchecked(val: Self) -> Self {
        val
    }
    fn raw(self) -> Self {
        self
    }
    unsafe fn from_f32_unchecked(val: f32) -> Self {
        Self::from_f32_unchecked(val)
    }
    unsafe fn from_f64_unchecked(val: f64) -> Self {
        Self::from_f64_unchecked(val)
    }
    fn into_f32(self) -> f32 {
        self.into_f32()
    }
    fn into_f64(self) -> f64 {
        self.into_f64()
    }
}
