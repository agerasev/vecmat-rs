use crate::{traits::*, vector::*};
use core::ops::{Add, Mul};
use num_traits::Float;

impl<T, const N: usize> Dot<Vector<T, N>> for Vector<T, N>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = T;
    fn dot(self, other: Vector<T, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x * y).sum()
    }
}
impl<T, const N: usize> Vector<T, N>
where
    T: Add<Output = T> + Mul<Output = T> + Clone,
{
    pub fn square_length(self) -> T {
        self.map(|x| x.clone() * x).sum()
    }
}
impl<T, const N: usize> Vector<T, N>
where
    T: Float + Clone,
{
    pub fn length(self) -> T {
        self.square_length().sqrt()
    }
    pub fn normalize(self) -> Vector<T, N> {
        self / self.length()
    }
}
