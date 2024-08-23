use fp::*;

#[test]
fn good_conversions() {
    let _: I16<7, 0> = I32::<6, 0>::new(5).unwrap().into_fp();
    let _: I32<8, 0> = 125i8.into_fp();
    let _: u16 = U32::<16, 0>::new(5).unwrap().into_fp();
}
