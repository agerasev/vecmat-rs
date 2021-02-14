use crate::{
    distr::*,
    vector::{Vector},
    complex::{Complex, Quaternion},
};
use num_traits::Float;
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
