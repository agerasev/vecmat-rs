use crate::Vector;
use core::{
    iter::{Product, Sum},
    ops::{Add, Mul},
};
use num_traits::{One, Zero};

impl<T: Zero + Add, const N: usize> Sum for Vector<T, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, x| a + x)
    }
}

impl<T: One + Mul, const N: usize> Product for Vector<T, N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::init(T::one), |a, x| a * x)
    }
}

#[cfg(test)]
mod test {
    use crate::Vector;

    #[test]
    fn sum() {
        let arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].map(Vector::<i32, 3>::from);
        assert_eq!(arr.into_iter().sum::<Vector<i32, 3>>(), [12, 15, 18].into());
    }

    #[test]
    fn product() {
        let arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].map(Vector::<i32, 3>::from);
        assert_eq!(
            arr.into_iter().product::<Vector<i32, 3>>(),
            [28, 80, 162].into(),
        );
    }
}
