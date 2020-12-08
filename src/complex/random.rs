use core::ops::{Div};
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

pub struct QuaternionDistribution<T, D: Distribution<T>> {
    distribution: D,
}

impl<T, D: Distribution<T>> QuaternionDistribution<T, D> {
    fn new(distribution: D) -> Self {
        Self { distribution }
    }
}

impl<T, D: Distribution<T>> Distribution<Quaternion<T>> for QuaternionDistribution<T, D> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        Quaternion::new(rng.sample(self), rng.sample(self), rng.sample(self), rng.sample(self))
    }
}

impl<T> Distribution<Quaternion<T>> for StandardNormal where StandardNormal: Distribution<U> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        Quaternion::new(rng.sample(Self), rng.sample(Self))
    }
}

impl<T> Distribution<Quaternion<T>> for StandardNormal where StandardNormal: Distribution<U> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        Quaternion::new(rng.sample(Self), rng.sample(Self))
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
