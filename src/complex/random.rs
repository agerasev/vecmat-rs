use std::marker::PhantomData;
use num_traits::Float;
use rand::Rng;
use rand_distr::Distribution;
use super::*;


pub use rand_distr::StandardNormal;

/// Distribution that only guarantees to produce an element which norm is greater than epsilon.
pub struct NonZero;

/// Distribution that provides points uniformly distubuted on the N-dimensional sphere,
/// where N is the number of dimensions of a specified hypercomplex number.
pub struct Unit;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuaternionDistribution<T, D: Distribution<T>> {
    pub inner: D,
    phantom: PhantomData<T>,
}

impl<T, D: Distribution<T>> QuaternionDistribution<T, D> {
    fn new(inner: D) -> Self {
        Self { inner, phantom: PhantomData }
    }
}

impl<T, D: Distribution<T> + Clone> Distribution<Quaternion<T>> for QuaternionDistribution<T, D> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        Vector4::init(|| rng.sample(self.inner.clone())).into()
    }
}

impl<T> Distribution<Quaternion<T>> for StandardNormal where StandardNormal: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        QuaternionDistribution::<T, StandardNormal>::new(Self).sample(rng)
    }
}

impl<T: Float + Clone> Distribution<Quaternion<T>> for NonZero where StandardNormal: Distribution<Quaternion<T>> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        loop {
            let x = rng.sample(&StandardNormal);
            if x.clone().norm() > T::epsilon() {
                break x;
            }
        }
    }
}

impl<T: Float + Clone> Distribution<Quaternion<T>> for Unit where NonZero: Distribution<Quaternion<T>> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample(&NonZero).normalize()
    }
}
