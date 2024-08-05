use core::ops::{Add, Neg, Sub};

use crate::Num;

/// Needed for const-generic support, because the built-in
/// methods to compute maximum of two values are not const.
pub const fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

macro_rules! fp_impl {
    ($Name:ident, $Iname:ident) => {
        use crate::$Name;
        /// Two fixed-point integers with the same raw type and the same shift may be
        /// added together.  The result has the same raw type and the same shift.  The result
        /// has 1 more bit than the number of bits in the wider of the two inputs.
        impl<const B0: u32, const B1: u32, const S: i32> Add<$Name<B1, S>> for $Name<B0, S>
        where
            [(); (max(B0, B1) + 1) as usize]:,
        {
            type Output = $Name<{ max(B0, B1) + 1 }, S>;
            fn add(self: $Name<B0, S>, other: $Name<B1, S>) -> Self::Output {
                unsafe {
                    Self::Output::new_unchecked(
                        self.raw().unchecked_add(other.raw()),
                    )
                }
            }
        }
        /// Two fixed-point integers with the same raw type and the same shift may be
        /// subtracted.  The result is always signed, even if the inputs were unsigned.
        /// The result has the same shift as the inputs, and 1 more bit than the number
        /// of bits in the wider of the two inputs.
        impl<const B0: u32, const B1: u32, const S: i32> Sub<$Name<B1, S>> for $Name<B0, S>
        where
            [(); (max(B0, B1) + 1) as usize]:,
        {
            // Subtraction output is always signed, even for unsigned inputs.
            type Output = $Iname<{ max(B0, B1) + 1 }, S>;
            fn sub(self: $Name<B0, S>, other: $Name<B1, S>) -> Self::Output {
                // Convert raw values to signed prior to subtracting.
                let self_raw = self.raw() as <Self::Output as Num>::Raw;
                let other_raw = other.raw() as <Self::Output as Num>::Raw;
                unsafe {
                    Self::Output::new_unchecked(
                        self_raw.unchecked_sub(other_raw),
                    )
                }
            }
        }
        /// Any fixed-point integer may be negated.  The result of negation is always
        /// signed.  Negation adds a bit: unsigned values gain a sign bit; signed values
        /// can overflow from `MIN` to `-MIN = MAX + 1`.  The shift is unchanged.
        impl<const B: u32, const S: i32> Neg for $Name<B, S>
        where
            [(); (B + 1) as usize]:,
        {
            type Output = $Iname<{ B + 1 }, S>;
            fn neg(self: $Name<B, S>) -> Self::Output {
                unsafe { Self::Output::new_unchecked(-(self.raw() as <Self::Output as Num>::Raw)) }
            }
        }
    };
}

fp_impl!(U8, I8);
fp_impl!(I8, I8);
fp_impl!(U16, I16);
fp_impl!(I16, I16);
fp_impl!(U32, I32);
fp_impl!(I32, I32);
fp_impl!(U64, I64);
fp_impl!(I64, I64);
fp_impl!(U128, I128);
fp_impl!(I128, I128);
fp_impl!(Usize, Isize);
fp_impl!(Isize, Isize);
