use num_traits::Float;
use rand_::{Rng, {distributions::Distribution}};
use crate::{vector::{Vector, VectorDistribution}, Quaternion, distr::*};

pub struct QuaternionDistribution<D: Distribution<T>, T> {
    pub inner: VectorDistribution<D, T, 4>,
}

impl<D: Distribution<T>, T> QuaternionDistribution<D, T> {
    pub fn new(inner: D) -> Self {
        Self { inner: VectorDistribution::new(inner) }
    }
}

impl<D: Distribution<T>, T> Distribution<Quaternion<T>> for QuaternionDistribution<D, T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample(&self.inner).into()
    }
}

impl<T> Distribution<Quaternion<T>> for Normal where Normal: Distribution<Vector<T, 4>> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample(Self).into()
    }
}

impl<T: Float> Distribution<Quaternion<T>> for NonZero where NonZero: Distribution<Vector<T, 4>> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample(Self).into()
    }
}

impl<T: Float> Distribution<Quaternion<T>> for Unit where Unit: Distribution<Vector<T, 4>> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample(Self).into()
    }
}
