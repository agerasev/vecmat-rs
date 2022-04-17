use crate::Matrix;
use core::{
    iter::{Product, Sum},
    ops::{Add, Mul},
};
use num_traits::{One, Zero};

impl<T: Zero + Add, const M: usize, const N: usize> Sum for Matrix<T, M, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, x| a + x)
    }
}

impl<T: One + Mul, const M: usize, const N: usize> Product for Matrix<T, M, N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::init(T::one), |a, x| a * x)
    }
}

#[cfg(test)]
mod test {
    use crate::Matrix;

    #[test]
    fn sum() {
        let arr = [[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]]]
            .map(Matrix::<i32, 2, 2>::from);
        assert_eq!(
            arr.into_iter().sum::<Matrix<i32, 2, 2>>(),
            [[15, 18], [21, 24]].into()
        );
    }

    #[test]
    fn product() {
        let arr = [[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]]]
            .map(Matrix::<i32, 2, 2>::from);
        assert_eq!(
            arr.into_iter().product::<Matrix<i32, 2, 2>>(),
            [[45, 120], [231, 384]].into()
        );
    }
}
