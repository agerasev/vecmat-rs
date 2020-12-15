use approx::{AbsDiffEq, abs_diff_eq};
use crate::{Vector4, Quaternion};

impl<T> AbsDiffEq for Quaternion<T> where T: AbsDiffEq<Epsilon=T> + Clone {
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(<Self as Into<Vector4<T>>>::into(self.clone()), other.clone().into(), epsilon=epsilon)
    }
}
