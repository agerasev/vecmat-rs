use crate::{
    distr::{NonZero, Normal, StatefulNormal, Unit},
    Vector,
};
use core::marker::PhantomData;
use num_traits::Float;
use rand_::{distributions::Distribution, Rng};

/// Per-component vector distribution.
pub struct VectorDistribution<D: Distribution<T>, T, const N: usize> {
    pub inner: D,
    phantom: PhantomData<Vector<T, N>>,
}

impl<D: Distribution<T>, T, const N: usize> VectorDistribution<D, T, N> {
    pub fn new(inner: D) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<D: Distribution<T>, T, const N: usize> Distribution<Vector<T, N>>
    for VectorDistribution<D, T, N>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector<T, N> {
        Vector::init(|| rng.sample(&self.inner))
    }
}

impl<T, const N: usize> Distribution<Vector<T, N>> for Normal
where
    StatefulNormal<T>: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector<T, N> {
        rng.sample(&VectorDistribution::new(StatefulNormal::new()))
    }
}

impl<T: Float, const N: usize> Distribution<Vector<T, N>> for NonZero
where
    Normal: Distribution<Vector<T, N>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector<T, N> {
        loop {
            let x = rng.sample(&Normal);
            if x.clone().length() > T::epsilon() {
                break x;
            }
        }
    }
}

impl<T: Float, const N: usize> Distribution<Vector<T, N>> for Unit
where
    NonZero: Distribution<Vector<T, N>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector<T, N> {
        rng.sample(&NonZero).normalize()
    }
}
