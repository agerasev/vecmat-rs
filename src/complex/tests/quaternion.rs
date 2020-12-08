use rand::{prelude::*};
use rand_xorshift::XorShiftRng;
use ::approx::*;
use crate::{prelude::*, *, random::*};


#[cfg(feature = "std")]
use std::{boxed::Box};


const SAMPLE_ATTEMPTS: usize = 256;
type Qf = Quaternion<f64>;


#[test]
fn imaginary_units() {
    type Qi = Quaternion<i32>;
    assert_eq!(Qi::i()*Qi::i(), -Qi::one());
    assert_eq!(Qi::j()*Qi::j(), -Qi::one());
    assert_eq!(Qi::k()*Qi::k(), -Qi::one());
    assert_eq!(Qi::i()*Qi::j()*Qi::k(), -Qi::one());

    assert_eq!(Qi::i()*Qi::j(), Qi::k());
    assert_eq!(Qi::j()*Qi::k(), Qi::i());
    assert_eq!(Qi::k()*Qi::i(), Qi::j());

    assert_eq!(Qi::j()*Qi::i(), -Qi::k());
    assert_eq!(Qi::k()*Qi::j(), -Qi::i());
    assert_eq!(Qi::i()*Qi::k(), -Qi::j());
}

#[test]
fn inversion() {
    let mut rng = XorShiftRng::seed_from_u64(0xFEED0);
    for _ in 0..SAMPLE_ATTEMPTS {
        let a: Qf = rng.sample(&NonZero);
        assert_abs_diff_eq!(a/a, Quaternion::one(), epsilon=1e-14);
    }
}

#[test]
fn law_of_cosines() {
    for _ in 0..SAMPLE_ATTEMPTS {
        let mut rng = XorShiftRng::seed_from_u64(0xFEED1);
        let a: Qf = rng.sample(&StandardNormal);
        let b: Qf = rng.sample(&StandardNormal);
        assert_abs_diff_eq!(a.norm_sqr() + b.norm_sqr() + 2.0*a.dot(b), (a + b).norm_sqr());
    }
}

#[test]
fn conjugation() {
    for _ in 0..SAMPLE_ATTEMPTS {
        let mut rng = XorShiftRng::seed_from_u64(0xFEED2);
        let a: Qf = rng.sample(&StandardNormal);
        assert_abs_diff_eq!(a*a.conj(), Quaternion::<f64>::one()*a.norm_sqr());
        assert_abs_diff_eq!(a.conj()*a, Quaternion::<f64>::one()*a.norm_sqr());
    }
}

#[cfg(feature = "std")]
#[test]
fn derivation() {
    let cases = [
        (
            Box::new(|p: Qf| p) as Box<dyn Fn(_) -> _>,
            Box::new(|_: Qf, v: Qf| v) as Box<dyn Fn(_, _) -> _>,
        ),
        (
            Box::new(|p: Qf| p*p),
            Box::new(|p: Qf, v: Qf| p*v + v*p),
        ),
        (
            Box::new(|p: Qf| p.inv()),
            Box::new(|p: Qf, v: Qf| {
                let p2 = p.norm_sqr();
                (v.conj() - (2.0*p.dot(v)/p2)*p.conj())/p2
            })
        ),
    ];

    const EPS: f64 = 1e-8;
    let mut rng = XorShiftRng::seed_from_u64(0xFEED3);

    for (f, dfdv) in cases.iter() {
        for _ in 0..SAMPLE_ATTEMPTS {
            let p = rng.sample(&StandardNormal);
            let v = rng.sample(&Unit);
            let deriv = dfdv(p, v);
            let dabs = deriv.norm();
            assert_abs_diff_eq!(
                deriv,
                (f(p + EPS*v) - f(p))/EPS,
                epsilon=1e-6*dabs,
            );
        }
    }
}
