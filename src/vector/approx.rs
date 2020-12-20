use crate::Vector;
use approx::{abs_diff_eq, AbsDiffEq};

impl<T, const N: usize> AbsDiffEq for Vector<T, N>
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
