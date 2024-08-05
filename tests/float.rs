#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use fp::*;

fn check_f32<T: Num>(raw: T::Raw, float: f32) {
    assert_eq!(float, T::new(raw).unwrap().into_f32());
    assert_eq!(raw, T::from_f32(float).unwrap().raw());
    assert_eq!(float as f64, T::new(raw).unwrap().into_f64());
    assert_eq!(raw, T::from_f64(float as f64).unwrap().raw());
}

fn check_f64<T: Num>(raw: T::Raw, float: f64) {
    assert_eq!(float, T::new(raw).unwrap().into_f64());
    assert_eq!(raw, T::from_f64(float).unwrap().raw());
}

// Smallest shift value which can't overflow when converted to float
const MIN_F32_SHIFT: i32 = f32::MANTISSA_DIGITS as i32 - f32::MAX_EXP;
const MIN_F64_SHIFT: i32 = f64::MANTISSA_DIGITS as i32 - f64::MAX_EXP;

// Largest shift value which can't underflow when converted to float
const MAX_F32_SHIFT: i32 = f32::MANTISSA_DIGITS as i32 - f32::MIN_EXP;
const MAX_F64_SHIFT: i32 = f64::MANTISSA_DIGITS as i32 - f64::MIN_EXP;

fn check_f32_max<T: Num>() {
    // This check only works for certain BITS and SHIFT combinations.
    assert_eq!(f32::MAX, T::MAX.into_f32() - T::MIN.into_f32());
    assert_eq!(f32::MIN, T::MIN.into_f32() - T::MAX.into_f32());
    assert_eq!(T::from_f32(T::MIN.into_f32()).unwrap(), T::MIN);
    assert_eq!(T::from_f32(T::MAX.into_f32()).unwrap(), T::MAX);
}

fn check_f64_max<T: Num>() {
    // This check only works for certain BITS and SHIFT combinations.
    assert_eq!(f64::MAX, T::MAX.into_f64() - T::MIN.into_f64());
    assert_eq!(f64::MIN, T::MIN.into_f64() - T::MAX.into_f64());
    assert_eq!(T::from_f64(T::MIN.into_f64()).unwrap(), T::MIN);
    assert_eq!(T::from_f64(T::MAX.into_f64()).unwrap(), T::MAX);
}

#[test]
fn extreme_floats() {
    // Smallest positive subnormal
    check_f32::<I16<2, MAX_F32_SHIFT>>(1, f32::from_bits(1));
    check_f64::<Usize<{ f64::MANTISSA_DIGITS }, MAX_F64_SHIFT>>(1, f64::from_bits(1));

    // Largest negative subnormal
    check_f32::<I8<1, MAX_F32_SHIFT>>(-1, -f32::from_bits(1));
    check_f64::<I64<{ f64::MANTISSA_DIGITS }, MAX_F64_SHIFT>>(-1, -f64::from_bits(1));

    // Largest positive subnormal
    check_f32::<U64<{ f32::MANTISSA_DIGITS }, MAX_F32_SHIFT>>(
        (1 << f32::MANTISSA_DIGITS) - 1,
        f32::from_bits((1 << f32::MANTISSA_DIGITS) - 1),
    );
    check_f64::<U64<{ f64::MANTISSA_DIGITS }, MAX_F64_SHIFT>>(
        (1 << f64::MANTISSA_DIGITS) - 1,
        f64::from_bits((1 << f64::MANTISSA_DIGITS) - 1),
    );

    // Smallest positive normal
    check_f32::<I128<2, { 1 - f32::MIN_EXP }>>(1, f32::MIN_POSITIVE);
    check_f64::<U8<1, { 1 - f64::MIN_EXP }>>(1, f64::MIN_POSITIVE);

    // Min and max signed
    check_f32_max::<I64<{ f32::MANTISSA_DIGITS }, MIN_F32_SHIFT>>();
    check_f64_max::<Isize<{ f64::MANTISSA_DIGITS }, MIN_F64_SHIFT>>();

    // Min and max unsigned
    check_f32_max::<U32<{ f32::MANTISSA_DIGITS }, MIN_F32_SHIFT>>();
    check_f64_max::<U128<{ f64::MANTISSA_DIGITS }, MIN_F64_SHIFT>>();

    // Zero bits
    check_f32::<I16<0, { -f32::MAX_EXP }>>(0, 0.0);
    check_f64::<I16<0, MAX_F64_SHIFT>>(0, 0.0);

    // Maximum number of bits
    check_f32::<U128<{ f32::MANTISSA_DIGITS }, { f32::MANTISSA_DIGITS as i32 - 2 }>>(
        (core::f32::consts::PI * 2_f32.powi(f32::MANTISSA_DIGITS as i32 - 2)) as u128,
        core::f32::consts::PI,
    );
    check_f64::<U64<{ f64::MANTISSA_DIGITS }, { f64::MANTISSA_DIGITS as i32 - 2 }>>(
        (core::f64::consts::PI * 2_f64.powi(f64::MANTISSA_DIGITS as i32 - 2)) as u64,
        core::f64::consts::PI,
    );
}

#[test]
#[should_panic(expected = "number could overflow f32")]
fn f32_overflow() {
    let num: I32<{ f32::MANTISSA_DIGITS }, { MIN_F32_SHIFT - 1 }> =
        unsafe { I32::new_unchecked(0) };
    let _ = num.into_f32();
}

#[test]
#[should_panic(expected = "number could overflow f64")]
fn f64_overflow() {
    let num: Usize<0, { -f64::MAX_EXP - 1 }> = unsafe { Usize::new_unchecked(0) };
    let _ = num.into_f64();
}

#[test]
#[should_panic(expected = "number could underflow f32")]
fn f32_underflow() {
    let num: I8<0, { MAX_F32_SHIFT + 1 }> = unsafe { I8::new_unchecked(0) };
    let _ = num.into_f32();
}

#[test]
#[should_panic(expected = "number could underflow f64")]
fn f64_underflow() {
    let num: I128<{ f64::MANTISSA_DIGITS }, { MAX_F64_SHIFT + 1 }> =
        unsafe { I128::new_unchecked(0) };
    let _ = num.into_f64();
}

#[test]
#[should_panic(expected = "number could be truncated in f32")]
fn f32_truncation() {
    let num: U32<{ f32::MANTISSA_DIGITS + 1 }, 0> = unsafe { U32::new_unchecked(0) };
    let _ = num.into_f32();
}

#[test]
#[should_panic(expected = "number could be truncated in f64")]
fn f64_truncation() {
    let num: I64<{ f64::MANTISSA_DIGITS + 1 }, 0> = unsafe { I64::new_unchecked(0) };
    let _ = num.into_f64();
}
