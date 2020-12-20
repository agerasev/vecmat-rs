use crate::Matrix;
use approx::{abs_diff_eq, AbsDiffEq};

impl<T, const M: usize, const N: usize> AbsDiffEq for Matrix<T, M, N>
where
    T: AbsDiffEq<Epsilon = T> + Copy,
{
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.zip(*other)
            .map(|(x, y)| abs_diff_eq!(x, y, epsilon = epsilon))
            .all()
    }
}
