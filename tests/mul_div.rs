#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use fp::*;

use core::ops::{Mul, Div};

fn validate_mul<A: Num, B: Num, C: Num>() where A: Mul<B, Output=C> {
    for a in [A::MIN, A::MAX] {
        for b in [B::MIN, B::MAX] {
            assert!(a * b >= C::MIN);
            assert!(a * b <= C::MAX);
        }
    }
}

#[test]
fn mul_limits() {
    validate_mul::<I32<4, 0>, I32<5, 0>, I32<9, 0>>();
    validate_mul::<I32<4, 0>, U32<5, 0>, I32<9, 0>>();
    validate_mul::<U32<4, 0>, I32<5, 0>, I32<9, 0>>();
    validate_mul::<U32<4, 0>, U32<5, 0>, U32<9, 0>>();
}
