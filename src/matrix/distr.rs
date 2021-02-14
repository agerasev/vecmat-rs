use crate::{
    traits::{NormL1, Epsilon},
    distr::{Invertible, Normal},
    Matrix,
};
use core::{ops::Neg, marker::PhantomData};
use num_traits::{Num};
use rand_::{distributions::Distribution, Rng};

/// Per-component matrix distribution.
pub struct MatrixDistribution<D: Distribution<T>, T, const M: usize, const N: usize> {
    pub inner: D,
    phantom: PhantomData<Matrix<T, M, N>>,
}

impl<D: Distribution<T>, T, const M: usize, const N: usize> MatrixDistribution<D, T, M, N> {
    pub fn new(inner: D) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<D: Distribution<T>, T, const M: usize, const N: usize> Distribution<Matrix<T, M, N>>
    for MatrixDistribution<D, T, M, N>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix<T, M, N> {
        Matrix::init(|| rng.sample(&self.inner))
    }
}

impl<T, const M: usize, const N: usize> Distribution<Matrix<T, M, N>> for Normal
where
    Normal: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix<T, M, N> {
        rng.sample(&MatrixDistribution::new(self))
    }
}

impl<T, const N: usize> Distribution<Matrix<T, N, N>> for Invertible
where
    Normal: Distribution<Matrix<T, N, N>>,
    T: Neg<Output=T> + Num + NormL1 + Copy,
    <T as NormL1>::Output: Epsilon,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Matrix<T, N, N> {
        loop {
            let x = rng.sample(&Normal);
            if !x.det().norm_l1().is_epsilon() {
                break x;
            }
        }
    }
}
