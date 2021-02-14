use rand_::{prelude::*};
use rand_xorshift::XorShiftRng;
use approx::*;
use crate::{
    distr::{Normal, SomeDistr},
    complex::{Complex, Quaternion, Moebius},
};

const TRANSFORM_ATTEMPTS: usize = 64;
const POINT_ATTEMPTS: usize = 16;

#[test]
#[allow(clippy::many_single_char_names)]
fn moebius2() {
    let mut rng = XorShiftRng::seed_from_u64(0xDEAD0);
    for _ in 0..TRANSFORM_ATTEMPTS {
        let a: Moebius<Complex<f64>> = rng.sample(SomeDistr);
        let b: Moebius<Complex<f64>> = rng.sample(SomeDistr);
        let c = a.chain(b);
        for _ in 0..POINT_ATTEMPTS {
            let x: Complex<f64> = rng.sample(Normal);
            let y = a.apply(b.apply(x));
            let z = c.apply(x);
            assert_abs_diff_eq!(y, z, epsilon=1e-12);
        }
    }
}

#[test]
#[allow(clippy::many_single_char_names)]
fn moebius4() {
    let mut rng = XorShiftRng::seed_from_u64(0xDEAD1);
    for _ in 0..TRANSFORM_ATTEMPTS {
        let a: Moebius<Quaternion<f64>> = rng.sample(SomeDistr);
        let b: Moebius<Quaternion<f64>> = rng.sample(SomeDistr);
        let c = a.chain(b);
        for _ in 0..POINT_ATTEMPTS {
            let x: Quaternion<f64> = rng.sample(Normal);
            let y = a.apply(b.apply(x));
            let z = c.apply(x);
            assert_abs_diff_eq!(y, z, epsilon=1e-12);
        }
    }
}
