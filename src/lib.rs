//! This crate provides fixed-point arithmetic with _statically verified_
//! overflow safety and bit shift correctness.
//!
//! Fixed-point arithmetic represents fractional values as integers with
//! an implicit bit shift.  For example, the decimal number 2.375 (in base 2: 10.011) could
//! be represented in fixed-point as the integer `0b10011` (decimal 19) with an implicit bit shift of 3.  It is
//! the programmer's responsibility to keep track of all the bit shifts used in a program,
//! ensure they are consistent with each other, and avoid any overflows during
//! arithmetic operations.
//!
//! In contrast, floating-point numbers automatically adjust the "bit shift" (i.e. the exponent)
//! to provide the largest possible resolution which will not overflow.
//! They are easy to use, and they do the right thing most of
//! the time.  However, they can cause subtle rounding bugs which are famously difficult
//! to identify and prevent.  In the immortal words of Professor Gerald Sussman,
//! "Nothing brings fear to my heart more than a floating-point number."
//!
//! This crate uses the Rust type system to provide fixed-point numbers with compile-time
//! bit shift checking and overflow protection.  Each fixed-point type has two const generic
//! parameters, one describing the bit shift and one describing the maximum
//! number of bits which could be nonzero.  Each arithmetic operation is implemented with
//! an output type which correctly reflects the bits and shift of the result.  For example,
//! the result of multiplying a 10-bit number (shifted by 2) and a 12-bit number (shifted by 3)
//! is a 22-bit number (shifted by 5).
//!
//! The trait `Num` represents any fixed-point number stored as an
//! integer, and the structs `NumXxx<const BITS: u32, const SHIFT: i32>` implement the
//! `Num` trait for each integer type `Xxx`.  Arithmetic operations on the fixed-point
//! types are guaranteed to provide correctness and overflow safety with zero runtime
//! overhead.
//!
//! It is necessary to use nightly Rust in order to enable the unstable
//! `generic_const_exprs` feature.  Otherwise it would not be possible to specify
//! the correct return type from most operations.

#![feature(generic_const_exprs)]

use core::fmt::Debug;
use core::ops::{Shl, Shr};

#[derive(Debug)]
pub enum RangeError {
    TooSmall,
    TooLarge,
}

/// A fixed-point number, stored as type `Raw`,
/// where only the `BITS` least-significant bits may be nonzero.
/// The raw value is divided by `2.pow(SHIFT)` to obtain the logical value.
pub trait Num: Clone + Copy + Debug + Eq + Ord + PartialEq + PartialOrd + Sized {
    /// The underlying ("raw") representation of this fixed-point number.
    /// Typically this is a primitive integer type, e.g. `i64`.
    type Raw: Num<Raw = Self::Raw> + Shl<u32, Output = Self::Raw> + Shr<u32, Output = Self::Raw>;
    /// The type that this fixed point number will become after `BITS` and/or `SHIFT`
    /// are changed by an operation.
    type Output<const B: u32, const S: i32>: Num<Raw = Self::Raw>;
    /// `BITS` is the number of least-significant bits which are permitted to vary.
    /// The `Raw::BITS - BITS` high-order bits must be zero (for unsigned `Raw`) or the
    /// same as the high bit of the lower `BITS` bits (for signed `Raw`).
    const BITS: u32;
    /// `SHIFT` sets the scaling factor between the stored raw value (of type `Raw`)
    /// and the "logical" value with it represents.  The logical value of this
    /// fixed-point number is equal to the raw value divided by `2.pow(SHIFT)`.
    ///
    /// In other words, positive `SHIFT` means that the logical value consists of
    /// `BITS - SHIFT` integer bits followed by `SHIFT` fractional bits, and negative
    /// shift means that the logical value consists of `BITS - SHIFT` integer bits
    /// (of which the last `-SHIFT` bits are zero).
    const SHIFT: i32;
    /// Minimum possible value of this type.
    const MIN: Self;
    /// Maximum possible value of this type.
    const MAX: Self;
    /// Whether this type is signed. (If false, it's unsigned.)
    const SIGNED: bool;
    /// Interpret the provided raw value as a fixed-point number of type `Self`.
    /// Unsafe: no bounds checking is performed; the caller must ensure that the
    /// result lies between `Self::MIN` and `Self::MAX`. It is almost always better
    /// to use `.new().unwrap()` instead of this function, so that an out-of-bounds
    /// value panics with a reasonable message instead of propagating undefined
    /// behavior.
    unsafe fn new_unchecked(val: Self::Raw) -> Self;
    /// Interpret the provided raw value as a fixed-point number of type `Self`,
    /// or return a `RangeError` if it is too small or too large to represent
    /// a valid instance of `Self`.
    fn new(val: Self::Raw) -> Result<Self, RangeError> {
        if val < Self::MIN.raw() {
            Err(RangeError::TooSmall)
        } else if val > Self::MAX.raw() {
            Err(RangeError::TooLarge)
        } else {
            Ok(unsafe { Self::new_unchecked(val) })
        }
    }
    /// Return the raw value which internally represents this fixed-point number.
    fn raw(self) -> Self::Raw;
    /// Return the fixed-point number of type `Self` which has a logical value of `val`,
    /// or return a RangeError if `val` is too small or too large to be represented
    /// by `Self`.
    fn from_f32(val: f32) -> Result<Self, RangeError> {
        assert!(val.is_finite());
        // TODO: handle fp types where MIN/MAX don't fit in f32
        if val < Self::MIN.into_f32() {
            Err(RangeError::TooSmall)
        } else if val > Self::MAX.into_f32() {
            Err(RangeError::TooLarge)
        } else {
            Ok(unsafe { Self::from_f32_unchecked(val) })
        }
    }
    unsafe fn from_f32_unchecked(val: f32) -> Self;
    /// Return the fixed-point number of type `Self` which has a logical value of `val`,
    /// or return a RangeError if `val` is too small or too large to be represented
    /// by `Self`.
    fn from_f64(val: f64) -> Result<Self, RangeError> {
        assert!(val.is_finite());
        // TODO: handle fp types where MIN/MAX don't fit in f64
        if val < Self::MIN.into_f64() {
            Err(RangeError::TooSmall)
        } else if val > Self::MAX.into_f64() {
            Err(RangeError::TooLarge)
        } else {
            Ok(unsafe { Self::from_f64_unchecked(val) })
        }
    }
    unsafe fn from_f64_unchecked(val: f64) -> Self;
    /// Return the logical value of `Self` as `f32`. Return value is guaranteed to be exact.
    fn into_f32(self) -> f32;
    /// Return the logical value of `Self` as `f64`. Return value is guaranteed to be exact.
    fn into_f64(self) -> f64;
    /// Return the fixed-point number of type `Self` which has the same logical value as `val`.
    /// `F` and `Self` must have the same shift and signedness. `Self` must have at least as
    /// many bits as `F`.
    fn from_fp<T: Num, F: Num<Raw = T>>(val: F) -> Self
    where
        Self::Raw: TryFrom<T>,
    {
        assert!(Self::SHIFT == F::SHIFT);
        assert!(Self::BITS >= F::BITS);
        assert!(Self::SIGNED == F::SIGNED);
        unsafe { Self::new_unchecked(val.raw().try_into().ok().unwrap()) }
    }
    /// Return the fixed-point number of type `F` which has the same logical value as `self`.
    /// `F` and `Self` must have the same shift and signedness. `F` must have at least as
    /// many bits as `Self`.
    fn into_fp<T: Num, F: Num<Raw = T>>(self) -> F
    where
        T: TryFrom<Self::Raw>,
    {
        F::from_fp(self)
    }
    /// Increase the number of bits used to represent this value. Both the raw and logical
    /// values are unchanged.  This is a type system operation only.
    /// Compilation will fail if the new number of bits is too large for the raw type.
    fn add_bits<const N: u32>(self) -> Self::Output<{ Self::BITS + N }, { Self::SHIFT }>
    where
        [(); (Self::BITS + N) as usize]:,
    {
        unsafe { Self::Output::new_unchecked(self.raw()) }
    }
    /// Set the number of bits used to represent this value. The value is checked
    /// at runtime to ensure it is in range for the new number of bits. If succesful,
    /// both the raw and logical values are unchanged.
    fn set_bits<const N: u32>(self) -> Result<Self::Output<N, { Self::SHIFT }>, RangeError> {
        Self::Output::new(self.raw())
    }
    /// Set the number of bits used to represent this value.  Unsafe: no bounds checking
    /// is performed; the caller must ensure that the value fits within
    /// the new number of bits.  It is almost always better to call `.set_bits().unwrap()`
    /// instead, so that an out-of-bounds
    /// value panics with a reasonable message instead of propagating undefined
    /// behavior.
    unsafe fn set_bits_unchecked<const N: u32>(self) -> Self::Output<N, { Self::SHIFT }> {
        unsafe { Self::Output::new_unchecked(self.raw()) }
    }
    /// Set the number of bits used to represent this value, saturating in case of
    /// overflow.
    fn saturate<const N: u32>(self) -> Self::Output<N, { Self::SHIFT }> {
        match Self::Output::new(self.raw()) {
            Err(RangeError::TooSmall) => Self::Output::MIN,
            Err(RangeError::TooLarge) => Self::Output::MAX,
            Ok(val) => val,
        }
    }
    /// Shift the logical value of this number left by N bits. (N may be negative
    /// for a right shift).  This is a type system operation only; the raw value
    /// is unchanged.  The logical value is multiplied by 2^N.
    fn logical_shl<const N: i32>(self) -> Self::Output<{ Self::BITS }, { Self::SHIFT - N }>
    where
        [(); (Self::SHIFT - N) as usize]:,
    {
        unsafe { Self::Output::new_unchecked(self.raw()) }
    }
    /// Shift the logical value of this number right by N bits. (N may be negative
    /// for a left shift).  This is a type system operation only; the raw value
    /// is unchanged.  The logical value is divided by 2^N.
    fn logical_shr<const N: i32>(self) -> Self::Output<{ Self::BITS }, { Self::SHIFT + N }>
    where
        [(); (Self::SHIFT + N) as usize]:,
    {
        unsafe { Self::Output::new_unchecked(self.raw()) }
    }
    /// Shift the raw value of this number left by N bits. Compiles to a left shift.
    /// The logical value is unchanged.
    fn raw_shl<const N: u32>(self) -> Self::Output<{ Self::BITS + N }, { Self::SHIFT + N as i32 }>
    where
        [(); (Self::BITS + N) as usize]:,
        [(); (Self::SHIFT + N as i32) as usize]:,
    {
        unsafe { Self::Output::new_unchecked(self.raw() << N) }
    }
    /// Shift the raw value of this number right by N bits. Compiles to a right shift.
    /// The logical value is unchanged, except for truncation of the N least-significant bits.
    fn raw_shr<const N: u32>(self) -> Self::Output<{ Self::BITS - N }, { Self::SHIFT - N as i32 }>
    where
        [(); (Self::BITS - N) as usize]:,
        [(); (Self::SHIFT - N as i32) as usize]:,
    {
        unsafe { Self::Output::new_unchecked(self.raw() >> N) }
    }
}

mod num_impl;
pub use num_impl::*;
mod add_sub;
mod mul_div;
