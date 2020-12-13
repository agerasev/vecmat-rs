use core::ops::Neg;
use num_traits::Num;
use crate::{Vector2, Dot};

pub use num_complex::*;


impl<T> From<Vector2<T>> for Complex<T> {
    fn from(vec: Vector2<T>) -> Self {
        let (x, y) = vec.into();
        Self::new(x, y)
    }
}
impl<T> Into<Vector2<T>> for Complex<T> {
    fn into(self) -> Vector2<T> {
        Vector2::from([self.re, self.im])
    }
}

impl<T> Dot for Complex<T> where T: Neg<Output=T> + Num {
    type Output = T;
    fn dot(self, other: Self) -> T {
        <Self as Into<Vector2<_>>>::into(self).dot(other.into()).into()
    }
}
