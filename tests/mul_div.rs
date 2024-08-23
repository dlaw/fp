use fp;

fn validate_mul<A: fp::Num, B: fp::Num, C: fp::Num>()
where
    A::Raw: fp::raw::Mul<B::Raw, C::Raw>,
{
    for a in [A::MIN, A::MAX] {
        for b in [B::MIN, B::MAX] {
            assert!(a.mul::<B, C>(b) >= C::MIN);
            assert!(a.mul::<B, C>(b) <= C::MAX);
        }
    }
}

#[test]
fn mul_limits() {
    validate_mul::<fp::I32<4, 0>, fp::I32<5, 0>, fp::I32<9, 0>>();
    validate_mul::<fp::I32<4, 0>, fp::U32<5, 0>, fp::I32<9, 0>>();
    validate_mul::<fp::U32<4, 0>, fp::I32<5, 0>, fp::I32<9, 0>>();
    validate_mul::<fp::U32<4, 0>, fp::U32<5, 0>, fp::U32<9, 0>>();
}
