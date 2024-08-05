use crate::*;

// Because Rust does not provide suitable traits over the integer types,
// we have to use a macro for the impls instead of writing one generic impl.
macro_rules! num_impl {
    ($Name:ident, $T:ty) => {
        /// Every integer is also a fixed-point number, considered to have
        /// the maximum number of bits and zero shift.
        impl Num for $T {
            type Raw = $T;
            type Output<const B: u32, const S: i32> = $Name<B, S>;
            const BITS: u32 = <$T>::BITS;
            const SHIFT: i32 = 0;
            const MIN: $T = <$T>::MIN;
            const MAX: $T = <$T>::MAX;
            #[allow(unused_comparisons)]
            const SIGNED: bool = <$T>::MIN < 0;
            unsafe fn new_unchecked(val: $T) -> Self {
                val
            }
            unsafe fn from_f32_unchecked(val: f32) -> Self {
                val.to_int_unchecked()
            }
            unsafe fn from_f64_unchecked(val: f64) -> Self {
                val.to_int_unchecked()
            }
            fn raw(self) -> $T {
                self
            }
            /// Conversion to f32 is guaranteed to be exact.  Therefore, this function only
            /// works for integer types which are no more than 24 bits wide.
            fn into_f32(self) -> f32 {
                assert!(
                    Self::BITS <= f32::MANTISSA_DIGITS,
                    "number could be truncated in f32"
                );
                self as f32
            }
            /// Conversion to f64 is guaranteed to be exact.  Therefore, this function only
            /// works for integer types which are no more than 53 bits wide.
            fn into_f64(self) -> f64 {
                assert!(
                    Self::BITS <= f32::MANTISSA_DIGITS,
                    "number could be truncated in f64"
                );
                self as f64
            }
        }

        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        /// [`#[repr(transparent)]`](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation)
        /// struct containing
        #[doc = concat!("[`", stringify!($T), "`]")]
        /// interpreted as a fixed-point number.
        ///
        /// Implements the trait [`fp::Num`](Num) for fixed-point manipulation.
        pub struct $Name<const BITS: u32, const SHIFT: i32>($T);

        impl<const BITS: u32, const SHIFT: i32> Num for $Name<BITS, SHIFT> {
            type Raw = $T;
            type Output<const B: u32, const S: i32> = $Name<B, S>;
            const BITS: u32 = {
                assert!(BITS <= <$T>::BITS, concat!("too many bits for ", stringify!($T)));
                BITS
            };
            const SHIFT: i32 = SHIFT;
            const MIN: Self = Self({
                if Self::BITS == 0 {
                    0
                } else {
                    // n.b. shifting by >= T::BITS is undefined for integer types!
                    <$T>::MIN >> (<$T>::BITS - Self::BITS)
                }
            });
            const MAX: Self = Self({
                if Self::BITS == 0 {
                    0
                } else {
                    // n.b. shifting by >= T::BITS is undefined for integer types!
                    <$T>::MAX >> (<$T>::BITS - Self::BITS)
                }
            });
            const SIGNED: bool = <$T>::SIGNED;
            unsafe fn new_unchecked(val: $T) -> Self {
                let _ = Self::BITS;  // force the compile-time check that T is wide enough for BITS
                Self(val)
            }
            /// The caller must ensure that `val` is within the range of this fixed-point type,
            /// and that the quantity `val / 2_f32.powi(-SHIFT)` is finite.
            unsafe fn from_f32_unchecked(val: f32) -> Self {
                unsafe { Self::new_unchecked((val / f32_lsb::<SHIFT>()).to_int_unchecked()) }
            }
            /// The caller must ensure that `val` is within the range of this fixed-point type,
            /// and that the quantity `val / 2_f64.powi(-SHIFT)` is finite.
            unsafe fn from_f64_unchecked(val: f64) -> Self {
                unsafe { Self::new_unchecked((val / f64_lsb::<SHIFT>()).to_int_unchecked()) }
            }
            fn raw(self) -> $T {
                self.0
            }
            /// Conversion to f32 is guaranteed to be exact.  Therefore, this function requires
            /// `BITS <= 24` (to prevent truncation), `SHIFT <= 149` (to prevent underflow),
            /// and `BITS - SHIFT <= 128` (to prevent overflow).
            ///
            /// For fixed-point numbers which do not meet these requirements, use `raw_shr()`
            /// to reduce the number of bits and `logical_shr()` to adjust the shift as needed.
            fn into_f32(self) -> f32 {
                assert!(
                    BITS <= f32::MANTISSA_DIGITS,
                    "number could be truncated in f32"
                );
                assert!(
                    SHIFT <= f32::MANTISSA_DIGITS as i32 - f32::MIN_EXP,
                    "number could underflow f32"
                );
                assert!(
                    BITS as i32 - SHIFT <= f32::MAX_EXP as i32,
                    "number could overflow f32"
                );
                // `BITS == 0` requires special handling because, in this case only,
                // `f32_lsb::<SHIFT>()` can overflow f32 (resulting in 0 * infinity).
                if BITS == 0 { 0. } else { self.0 as f32 * f32_lsb::<SHIFT>() }
            }
            /// Conversion to f64 is guaranteed to be exact.  Therefore, this function requires
            /// `BITS <= 53` (to prevent truncation), `SHIFT <= 1074` (to prevent underflow),
            /// and `BITS - SHIFT <= 1024` (to prevent overflow).
            ///
            /// For fixed-point numbers which do not meet these requirements, use `raw_shr()`
            /// to reduce the number of bits and `logical_shr()` to adjust the shift as needed.
            fn into_f64(self) -> f64 {
                assert!(
                    BITS <= f64::MANTISSA_DIGITS,
                    "number could be truncated in f64"
                );
                assert!(
                    SHIFT <= f64::MANTISSA_DIGITS as i32 - f64::MIN_EXP,
                    "number could underflow f64"
                );
                assert!(
                    BITS as i32 - SHIFT <= f64::MAX_EXP as i32,
                    "number could overflow f64"
                );
                // `BITS == 0` requires special handling because, in this case only,
                // `f64_lsb::<SHIFT>()` can overflow f64 (resulting in 0 * infinity).
                if BITS == 0 { 0. } else { self.0 as f64 * f64_lsb::<SHIFT>() }
            }
        }

        impl<const BITS: u32, const SHIFT: i32> From<$Name<BITS, SHIFT>> for f32 {
            fn from(val: $Name<BITS, SHIFT>) -> f32 {
                val.into_f32()
            }
        }

        impl<const BITS: u32, const SHIFT: i32> From<$Name<BITS, SHIFT>> for f64 {
            fn from(val: $Name<BITS, SHIFT>) -> f64 {
                val.into_f64()
            }
        }

        impl<const BITS: u32, const SHIFT: i32> TryFrom<f32> for $Name<BITS, SHIFT> {
            type Error = RangeError;
            fn try_from(val: f32) -> Result<Self, Self::Error> {
                Self::from_f32(val)
            }
        }

        impl<const BITS: u32, const SHIFT: i32> TryFrom<f64> for $Name<BITS, SHIFT> {
            type Error = RangeError;
            fn try_from(val: f64) -> Result<Self, Self::Error> {
                Self::from_f64(val)
            }
        }

        #[doc = concat!("`", stringify!($T), "` is the same as `", stringify!($Name), "<", stringify!($T) ,"::BITS, 0>`.")]
        impl From<$T> for $Name<{ <$T>::BITS }, 0> {
            fn from(val: $T) -> Self {
                unsafe { Self::new_unchecked(val) }
            }
        }

        #[doc = concat!("`", stringify!($T), "` is the same as `", stringify!($Name), "<", stringify!($T) ,"::BITS, 0>`.")]
        impl From<$Name<{ <$T>::BITS }, 0>> for $T {
            fn from(val: $Name<{ <$T>::BITS }, 0>) -> Self {
                val.raw()
            }
        }
    };
}

num_impl!(I8, i8);
num_impl!(U8, u8);
num_impl!(I16, i16);
num_impl!(U16, u16);
num_impl!(I32, i32);
num_impl!(U32, u32);
num_impl!(I64, i64);
num_impl!(U64, u64);
num_impl!(I128, i128);
num_impl!(U128, u128);
num_impl!(Isize, isize);
num_impl!(Usize, usize);

macro_rules! num_signed_unsigned_impl {
    ($Uname:ident, $Iname:ident) => {
        impl<const B: u32, const S: i32> $Uname<B, S> {
            pub fn into_signed(self) -> $Iname<{ B + 1 }, S>
            where
                [(); (B + 1) as usize]:,
            {
                unsafe { $Iname::new_unchecked(self.raw() as <$Iname<{ B + 1 }, S> as Num>::Raw) }
            }
        }
        impl<const B: u32, const S: i32> $Iname<B, S> {
            pub unsafe fn into_unsigned_unchecked(self) -> $Uname<{ B - 1 }, S>
            where
                [(); (B - 1) as usize]:,
            {
                unsafe { $Uname::new_unchecked(self.raw() as <$Uname<B, S> as Num>::Raw) }
            }
            pub fn into_unsigned(self) -> Option<$Uname<{ B - 1 }, S>>
            where
                [(); (B - 1) as usize]:,
            {
                if self.raw() >= 0 {
                    Some(unsafe { self.into_unsigned_unchecked() })
                } else {
                    None
                }
            }
        }
    };
}

num_signed_unsigned_impl!(U8, I8);
num_signed_unsigned_impl!(U16, I16);
num_signed_unsigned_impl!(U32, I32);
num_signed_unsigned_impl!(U64, I64);
num_signed_unsigned_impl!(U128, I128);
num_signed_unsigned_impl!(Usize, Isize);

fn f32_lsb<const SHIFT: i32>() -> f32 {
    // This function returns the exact value of `2_f32.powi(-SHIFT)`.
    // (On some architectures, powi() rounds subnormal numbers to zero,
    // so we must construct the raw float manually.) Bounds checking is
    // not performed; return value is undefined if SHIFT is out of range.
    let exp = 2 - f32::MIN_EXP - SHIFT;
    f32::from_bits(if exp > 0 {
        // normal float
        (exp as u32) << (f32::MANTISSA_DIGITS - 1)
    } else {
        // subnormal float
        1u32 << (f32::MANTISSA_DIGITS as i32 + exp - 2)
    })
}

fn f64_lsb<const SHIFT: i32>() -> f64 {
    // This function returns the exact value of `2_f64.powi(-SHIFT)`.
    // (On some architectures, powi() rounds subnormal numbers to zero,
    // so we must construct the raw float manually.) Bounds checking is
    // not performed; return value is undefined if SHIFT is out of range.
    let exp = 2 - f64::MIN_EXP - SHIFT;
    f64::from_bits(if exp > 0 {
        // normal float
        (exp as u64) << (f64::MANTISSA_DIGITS - 1)
    } else {
        // subnormal float
        1u64 << (f64::MANTISSA_DIGITS as i32 + exp - 2)
    })
}
