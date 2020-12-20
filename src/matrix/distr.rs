use crate::{distr::{Normal, Invertible}, Matrix};
use core::marker::PhantomData;
use rand_::{distributions::Distribution, Rng};
use num_traits::Float;

/// Per-component matrix distribution.
pub struct MatrixDistribution<D: Distribution<T>, T, const N: usize, const M: usize> {
    pub inner: D,
    phantom: PhantomData<Matrix<T, M, N>>,
}

impl<D: Distribution<T>, T, const N: usize, const M: usize> MatrixDistribution<D, T, M, N> {
    pub fn new(inner: D) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<D: Distribution<T>, T, const N: usize, const M: usize> Distribution<Matrix<T, M, N>>
    for MatrixDistribution<D, T, M, N>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix<T, M, N> {
        Matrix::init(|| rng.sample(&self.inner))
    }
}

impl<T, const N: usize, const M: usize> Distribution<Matrix<T, M, N>> for Normal
where
    Normal: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix<T, M, N> {
        rng.sample(&MatrixDistribution::new(Normal))
    }
}

impl<T, const N: usize> Distribution<Matrix<T, N, N>> for Invertible
where
    Normal: Distribution<Matrix<T, N, N>>,
    T: Float
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix<T, N, N> {
        loop {
            let x = rng.sample(&Normal);
            if x.det().abs() > T::epsilon() {
                break x;
            }
        }
    }
}
