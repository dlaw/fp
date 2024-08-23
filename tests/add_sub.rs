use fp;

fn validate<A: fp::Num, B: fp::Num, C: fp::Num>()
where
    A::Raw: fp::raw::Add<A::Raw, B::Raw>
        + core::ops::Add<A::Raw, Output = B::Raw>
        + fp::raw::Sub<A::Raw, C::Raw>
        + fp::raw::Neg<C::Raw>
        + TryInto<C::Raw>,
    C::Raw: core::ops::Neg<Output = C::Raw> + core::ops::Sub<C::Raw, Output = C::Raw>,
{
    for a0 in [A::MIN, A::ZERO, A::MAX] {
        for a1 in [A::MIN, A::ZERO, A::MAX] {
            assert!(a0.add::<A, B>(a1) >= B::MIN);
            assert!(a0.add::<A, B>(a1) <= B::MAX);
            assert!(a0.raw() + a1.raw() == a0.add::<A, B>(a1).raw());
            assert!(a0.sub::<A, C>(a1) >= C::MIN);
            assert!(a0.sub::<A, C>(a1) <= C::MAX);
            assert!(
                a0.raw().try_into().ok().unwrap() - a1.raw().try_into().ok().unwrap()
                    == a0.sub::<A, C>(a1).raw()
            )
        }
        assert!(a0.neg::<C>() >= C::MIN);
        assert!(a0.neg::<C>() <= C::MAX);
        let a0_as_c: C::Raw = a0.raw().try_into().ok().unwrap();
        assert!(a0.neg::<C>().raw() == -a0_as_c);
    }
}

#[test]
fn add_sub_limits() {
    validate::<fp::I8<7, -3>, fp::I8<8, -3>, fp::I8<8, -3>>();
    validate::<fp::I32<4, 0>, fp::I32<5, 0>, fp::I32<5, 0>>();
    validate::<fp::Usize<12, 0>, fp::Usize<13, 0>, fp::Isize<13, 0>>();
    validate::<fp::U128<127, 41>, fp::U128<128, 41>, fp::I128<128, 41>>();
}
