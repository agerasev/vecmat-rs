#[cfg(feature = "rand")]
use crate::distr::{Invertible, Normal};
use crate::{Vector, transform::{Chain, Linear, Shift}};
#[cfg(feature = "rand")]
use rand_::{distributions::Distribution, Rng};

/// Affine transformation.
pub type Affine<T, const N: usize> = Chain<Shift<T, N>, Linear<T, N>, Vector<T, N>>;

pub type Affine2<T> = Affine<T, 2>;
pub type Affine3<T> = Affine<T, 3>;
pub type Affine4<T> = Affine<T, 4>;

impl<T, const N: usize> Affine<T, N>
where
    T: Copy,
{
    /// Linear component of the transformation.
    pub fn linear(&self) -> Linear<T, N> {
        *self.inner()
    }

    /// Shift component of the transformation.
    pub fn shift(&self) -> Shift<T, N> {
        *self.outer()
    }
}

#[cfg(feature = "rand")]
impl<T, const N: usize> Distribution<Affine<T, N>> for Normal
where
    Normal: Distribution<Linear<T, N>> + Distribution<Shift<T, N>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Affine<T, N> {
        Affine::new(self.sample(rng), self.sample(rng))
    }
}
#[cfg(feature = "rand")]
impl<T, const N: usize> Distribution<Affine<T, N>> for Invertible
where
    Invertible: Distribution<Linear<T, N>>,
    Normal: Distribution<Shift<T, N>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Affine<T, N> {
        Affine::new(rng.sample(&Normal), rng.sample(&Self))
    }
}

#[cfg(all(test, feature = "approx"))]
mod tests {
    mod base {
        use super::super::*;
        use crate::{matrix::*, vector::*, Transform};
        use approx::*;
        use num_traits::{One, Zero};

        macro_rules! identity_test {
            ($X:ident, $M:ident, $V:ident) => {
                let m = $X::<f64>::identity();
                assert_abs_diff_eq!(Into::<$M<_>>::into(m.linear()), $M::one());
                assert_abs_diff_eq!(Into::<$V<_>>::into(m.shift()), $V::zero());
                let v = $V::fill(1.0);
                assert_abs_diff_eq!(v, m.apply(v));
            };
        }
        #[test]
        fn identity() {
            identity_test!(Affine2, Matrix2x2, Vector2);
            identity_test!(Affine3, Matrix3x3, Vector3);
            identity_test!(Affine4, Matrix4x4, Vector4);
        }

        macro_rules! inverse_test {
            ($X:ident, $M:ident, $V:ident) => {
                let m = $X::new($V::fill(1.0).into(), ($M::fill(1.0) + $M::one()).into());
                let v = $V::fill(1.0);
                let eps = 1e-12;
                assert_abs_diff_eq!(v, m.inv().apply(m.apply(v)), epsilon = eps);
                assert_abs_diff_eq!(v, m.apply(m.inv().apply(v)), epsilon = eps);
            };
        }
        #[test]
        fn inverse() {
            inverse_test!(Affine2, Matrix2x2, Vector2);
            inverse_test!(Affine3, Matrix3x3, Vector3);
            inverse_test!(Affine4, Matrix4x4, Vector4);
        }

        macro_rules! chain_test {
            ($X:ident, $M:ident, $V:ident) => {
                let m0 = $X::new($V::fill(1.0).into(), ($M::fill(1.0) + $M::one()).into());
                let m1 = $X::new(
                    $V::indices().map(|i| i as f64).into(),
                    ($M::fill(1.0) - $M::one()).into(),
                );
                let v = $V::fill(1.0);
                assert_abs_diff_eq!(m0.apply(m1.apply(v)), m0.chain(m1).apply(v));
                assert_abs_diff_eq!(m1.apply(m0.apply(v)), m1.chain(m0).apply(v));
            };
        }
        #[test]
        fn chain() {
            chain_test!(Affine2, Matrix2x2, Vector2);
            chain_test!(Affine3, Matrix3x3, Vector3);
            chain_test!(Affine4, Matrix4x4, Vector4);
        }
    }

    #[cfg(feature = "rand")]
    mod random {
        use super::super::*;
        use crate::{vector::*, Transform};
        use approx::assert_abs_diff_eq;
        use num_traits::Zero;
        use rand_::prelude::*;
        use rand_xorshift::XorShiftRng;

        const SAMPLE_ATTEMPTS: usize = 256;

        #[test]
        fn chaining() {
            const EPS: f64 = 1e-14;
            let mut rng = XorShiftRng::seed_from_u64(0xCEE);

            for _ in 0..SAMPLE_ATTEMPTS {
                let a: Affine3<f64> = rng.sample(&Normal);
                let b: Affine3<f64> = rng.sample(&Normal);
                let c: Vector3<f64> = rng.sample(&Normal);
                let z = Vector3::<f64>::zero();

                assert_abs_diff_eq!(a.chain(b).apply(c), a.apply(b.apply(c)), epsilon = EPS);
                assert_abs_diff_eq!(
                    a.chain(b).deriv(z, c),
                    a.deriv(z, b.deriv(z, c)),
                    epsilon = EPS
                );
            }
        }

        #[test]
        fn inversion() {
            const EPS: f64 = 1e-12;
            let mut rng = XorShiftRng::seed_from_u64(0xDEE);

            for _ in 0..SAMPLE_ATTEMPTS {
                let a: Affine3<f64> = rng.sample(&Invertible);
                let x: Vector3<f64> = rng.sample(&Normal);
                let z = Vector3::<f64>::zero();

                assert_abs_diff_eq!(a.inv().apply(a.apply(x)), x, epsilon = EPS);
                assert_abs_diff_eq!(a.apply(a.inv().apply(x)), x, epsilon = EPS);
                assert_abs_diff_eq!(a.inv().deriv(z, a.deriv(z, x)), x, epsilon = EPS);
                assert_abs_diff_eq!(a.deriv(z, a.inv().deriv(z, x)), x, epsilon = EPS);
            }
        }
    }
}
