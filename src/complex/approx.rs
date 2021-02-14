use crate::complex::{Complex, Quaternion};
use approx::{abs_diff_eq, AbsDiffEq};

impl<T> AbsDiffEq for Complex<T>
where
    T: AbsDiffEq<Epsilon = T> + Copy,
{
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.into_vector(), other.into_vector(), epsilon = epsilon)
    }
}

impl<T> AbsDiffEq for Quaternion<T>
where
    T: AbsDiffEq<Epsilon = T> + Copy,
{
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.into_vector(), other.into_vector(), epsilon = epsilon)
    }
}