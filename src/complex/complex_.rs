use crate::{traits::Dot, vector::Vector2};
use core::ops::Neg;
use num_traits::Num;

pub use num_complex::{Complex, ParseComplexError};

impl<T> From<Vector2<T>> for Complex<T> {
    fn from(vec: Vector2<T>) -> Self {
        let (x, y) = vec.into();
        Self::new(x, y)
    }
}
impl<T> From<Complex<T>> for Vector2<T> {
    fn from(c: Complex<T>) -> Self {
        Vector2::from([c.re, c.im])
    }
}

impl<T> Dot for Complex<T>
where
    T: Neg<Output = T> + Num,
{
    type Output = T;
    fn dot(self, other: Self) -> T {
        <Self as Into<Vector2<_>>>::into(self).dot(other.into())
    }
}
