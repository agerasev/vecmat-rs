use crate::{traits::{Dot, Normalize}, Vector};
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
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    pub fn square_length(self) -> T {
        self.map(|x| x * x).sum()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Float,
{
    pub fn length(self) -> T {
        self.square_length().sqrt()
    }
    pub fn normalize(self) -> Vector<T, N> {
        self / self.length()
    }
}

impl<T, const N: usize> Normalize for Vector<T, N>
where
    T: Float
{
    fn normalize(self) -> Self {
        Vector::normalize(self)
    }
}
