use core::marker::PhantomData;
use num_traits::Float;
use rand::Rng;
use rand_distr::Distribution;
use crate::{vector::*, complex::*, distributions::*};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuaternionDistribution<T, D: Distribution<T>> {
    pub inner: D,
    phantom: PhantomData<T>,
}

impl<T, D: Distribution<T>> QuaternionDistribution<T, D> {
    pub fn new(inner: D) -> Self {
        Self { inner, phantom: PhantomData }
    }
}

impl<T, D: Distribution<T> + Clone> Distribution<Quaternion<T>> for QuaternionDistribution<T, D> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample(VectorDistribution4::new(self.inner.clone())).into()
    }
}

impl<T> Distribution<Quaternion<T>> for StandardNormal where StandardNormal: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample::<Vector4<T>, Self>(Self).into()
    }
}

impl<T: Float + Clone> Distribution<Quaternion<T>> for NonZero where StandardNormal: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample::<Vector4<T>, Self>(Self).into()
    }
}

impl<T: Float + Clone> Distribution<Quaternion<T>> for Unit where StandardNormal: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<T> {
        rng.sample::<Vector4<T>, Self>(Self).into()
    }
}
