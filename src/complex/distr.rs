use crate::{
    distr::*,
    vector::{Vector},
    matrix::{Matrix},
    complex::{Complex, Quaternion, Moebius},
};
use core::ops::{Neg};
use num_traits::{Float, Num};
use rand_::{distributions::Distribution, Rng};

impl<T> Distribution<Complex<T>> for Normal
where
    Normal: Distribution<Vector<T, 2>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Complex<T> {
        rng.sample(self).into()
    }
}

impl<T: Float> Distribution<Complex<T>> for NonZero
where
    NonZero: Distribution<Vector<T, 2>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Complex<T> {
        rng.sample(Self).into()
    }
}

impl<T: Float> Distribution<Complex<T>> for Unit
where
    Unit: Distribution<Vector<T, 2>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Complex<T> {
        rng.sample(Self).into()
    }
}

impl<T> Distribution<Quaternion<T>> for Normal
where
    Normal: Distribution<Vector<T, 4>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample(self).into()
    }
}

impl<T: Float> Distribution<Quaternion<T>> for NonZero
where
    NonZero: Distribution<Vector<T, 4>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample(Self).into()
    }
}

impl<T: Float> Distribution<Quaternion<T>> for Unit
where
    Unit: Distribution<Vector<T, 4>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample(Self).into()
    }
}

impl<T: Neg<Output=T> + Num + Copy> Distribution<Moebius<T>> for SomeDistr
where
    Invertible: Distribution<Matrix<T, 2, 2>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Moebius<T> {
        let mat = rng.sample(Invertible);
        (mat / mat.det()).into()
    }
}
