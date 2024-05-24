use core::ops::{Div, Mul};

use crate::Num;

macro_rules! fp_impl {
    ($Name:ident, $T:ty) => {
        use crate::$Name;
        impl<const BITS: u32, const SHIFT: i32> $Name<BITS, SHIFT> {
            /// Returns the bit width of the return type from `mul_const`.
            pub const fn mul_const_bits(val: $T) -> u32 {
                #[allow(unused_comparisons)] // (this code runs for both signed and unsigned types)
                if val == 0 {
                    0
                } else if val < 0 {
                    // negative value of signed type
                    BITS + <$T>::BITS - val.wrapping_sub(1).leading_ones()
                } else {
                    // positive value
                    BITS + <$T>::BITS - (val - 1).leading_zeros()
                }
            }
            pub fn mul_const<const VAL: $T>(self) -> $Name<{ Self::mul_const_bits(VAL) }, SHIFT> {
                unsafe { $Name::new_unchecked(self.raw() * VAL) }
            }
            /// Returns the bit width of the return type from `div_const`.
            pub const fn div_const_bits(val: $T) -> u32 {
                #[allow(unused_comparisons)] // (this code runs for both signed and unsigned types)
                if val == 0 {
                    panic!("divide by 0")
                } else if val < 0 {
                    // negative value of signed type
                    1 + BITS - (<$T>::BITS - val.leading_ones())
                } else {
                    // positive value
                    1 + BITS - (<$T>::BITS - val.leading_zeros())
                }
            }
            pub fn div_const<const VAL: $T>(self) -> $Name<{ Self::div_const_bits(VAL) }, SHIFT> {
                unsafe { $Name::new_unchecked(self.raw() / VAL) }
            }
        }
        impl<const B0: u32, const B1: u32, const S0: i32, const S1: i32> Mul<$Name<B1, S1>>
            for $Name<B0, S0>
        where
            [(); (B0 + B1) as usize]:,
            [(); (S0 + S1) as usize]:,
        {
            type Output = $Name<{ B0 + B1 }, { S0 + S1 }>;
            fn mul(self: $Name<B0, S0>, other: $Name<B1, S1>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.raw() * other.raw()) }
            }
        }
        impl<const B0: u32, const B1: u32, const S0: i32, const S1: i32> Div<$Name<B1, S1>>
            for $Name<B0, S0>
        where
            [(); (B0 + Self::SIGNED as u32) as usize]:,
            [(); (S0 - S1) as usize]:,
        {
            // Division is tricky:
            // 1. T is unsigned or B0 = 0: worst case output bits is simply B0.
            // 2. T is signed and B0 != 0: worst case output bits (MIN / -1) is B0 + 1.
            type Output = $Name<{ B0 + Self::SIGNED as u32 }, { S0 - S1 }>;
            fn div(self: $Name<B0, S0>, other: $Name<B1, S1>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(self.raw() / other.raw()) }
            }
        }
    };
}

fp_impl!(I8, i8);
fp_impl!(U8, u8);
fp_impl!(I16, i16);
fp_impl!(U16, u16);
fp_impl!(I32, i32);
fp_impl!(U32, u32);
fp_impl!(I64, i64);
fp_impl!(U64, u64);
fp_impl!(I128, i128);
fp_impl!(U128, u128);
fp_impl!(Isize, isize);
fp_impl!(Usize, usize);

macro_rules! fp_signed_unsigned_impl {
    ($Uname:ident, $Iname:ident) => {
        impl<const B0: u32, const B1: u32, const S0: i32, const S1: i32> Mul<$Uname<B1, S1>>
            for $Iname<B0, S0>
        where
            [(); (B0 + B1) as usize]:,
            [(); (S0 + S1) as usize]:,
        {
            type Output = $Iname<{ B0 + B1 }, { S0 + S1 }>;
            fn mul(self: $Iname<B0, S0>, other: $Uname<B1, S1>) -> Self::Output {
                unsafe {
                    Self::Output::new_unchecked(self.raw() * other.raw() as <Self::Output as Num>::Raw)
                }
            }
        }
        impl<const B0: u32, const B1: u32, const S0: i32, const S1: i32> Mul<$Iname<B1, S1>>
            for $Uname<B0, S0>
        where
            [(); (B0 + B1) as usize]:,
            [(); (S0 + S1) as usize]:,
        {
            type Output = $Iname<{ B0 + B1 }, { S0 + S1 }>;
            fn mul(self: $Uname<B0, S0>, other: $Iname<B1, S1>) -> Self::Output {
                unsafe {
                    Self::Output::new_unchecked(self.raw() as <Self::Output as Num>::Raw * other.raw())
                }
            }
        }
        impl<const B0: u32, const B1: u32, const S0: i32, const S1: i32> Div<$Uname<B1, S1>>
            for $Iname<B0, S0>
        where
            [(); (S0 - S1) as usize]:,
        {
            type Output = $Iname<B0, { S0 - S1 }>;
            fn div(self: $Iname<B0, S0>, other: $Uname<B1, S1>) -> Self::Output {
                unsafe {
                    Self::Output::new_unchecked(self.raw() / other.raw() as <Self::Output as Num>::Raw)
                }
            }
        }
        impl<const B0: u32, const B1: u32, const S0: i32, const S1: i32> Div<$Iname<B1, S1>>
            for $Uname<B0, S0>
        where
            [(); (B0 + 1) as usize]:,
            [(); (S0 - S1) as usize]:,
        {
            type Output = $Iname<{ B0 + 1 }, { S0 - S1 }>;
            fn div(self: $Uname<B0, S0>, other: $Iname<B1, S1>) -> Self::Output {
                unsafe {
                    Self::Output::new_unchecked(self.raw() as <Self::Output as Num>::Raw / other.raw())
                }
            }
        }
    };
}

fp_signed_unsigned_impl!(U8, I8);
fp_signed_unsigned_impl!(U16, I16);
fp_signed_unsigned_impl!(U32, I32);
fp_signed_unsigned_impl!(U64, I64);
fp_signed_unsigned_impl!(U128, I128);
fp_signed_unsigned_impl!(Usize, Isize);
