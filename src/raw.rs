// This file is a bunch of boilerplate which we need because the Rust
// standard library doesn't include a trait for basic integer functions.

pub trait Add<Other, Output> {
    fn add(self, other: Other) -> Output;
}

pub trait Sub<Other, Output> {
    fn sub(self, other: Other) -> Output;
}

pub trait Mul<Other, Output> {
    fn mul(self, other: Other) -> Output;
}

pub trait Div<Other, Output> {
    fn div(self, other: Other) -> Output;
}

pub trait Neg<Output> {
    fn neg(self) -> Output;
}

macro_rules! int_impl {
    ($T:ty, $Signed:ty) => {
        /// Every integer is also a fixed-point number, considered to have
        /// the maximum number of bits and zero shift.
        impl crate::Num for $T {
            type Raw = $T;
            const BITS: u32 = <$T>::BITS;
            const SHIFT: i32 = 0;
            const MIN: Self = <$T>::MIN;
            const ZERO: Self = 0;
            const MAX: Self = <$T>::MAX;
            #[allow(unused_comparisons)]
            const SIGNED: bool = <$T>::MIN < 0;
            unsafe fn new_unchecked(val: Self) -> Self {
                val
            }
            fn raw(self) -> Self {
                self
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
        impl Add<$T, $T> for $T {
            fn add(self, other: $T) -> $T {
                self.wrapping_add(other)
            }
        }
        impl Sub<$T, $Signed> for $T {
            fn sub(self, other: $T) -> $Signed {
                self.wrapping_sub(other) as $Signed
            }
        }
        impl Neg<$Signed> for $T {
            fn neg(self) -> $Signed {
                -(self as $Signed)
            }
        }
        impl Mul<$T, $T> for $T {
            fn mul(self, other: $T) -> $T {
                self.wrapping_mul(other)
            }
        }
        impl Div<$T, $T> for $T {
            fn div(self, other: $T) -> $T {
                self / other
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

macro_rules! int_signed_unsigned_impl {
    ($Unsigned:ty, $Signed:ty) => {
        impl Mul<$Signed, $Signed> for $Unsigned {
            fn mul(self, other: $Signed) -> $Signed {
                (self as $Signed).wrapping_mul(other)
            }
        }
        impl Mul<$Unsigned, $Signed> for $Signed {
            fn mul(self, other: $Unsigned) -> $Signed {
                self.wrapping_mul(other as $Signed)
            }
        }
        impl Div<$Signed, $Signed> for $Unsigned {
            fn div(self, other: $Signed) -> $Signed {
                (self as $Signed) / other
            }
        }
        impl Div<$Unsigned, $Signed> for $Signed {
            fn div(self, other: $Unsigned) -> $Signed {
                self / (other as $Signed)
            }
        }
    };
}

int_signed_unsigned_impl!(u8, i8);
int_signed_unsigned_impl!(u16, i16);
int_signed_unsigned_impl!(u32, i32);
int_signed_unsigned_impl!(u64, i64);
int_signed_unsigned_impl!(u128, i128);
int_signed_unsigned_impl!(usize, isize);
