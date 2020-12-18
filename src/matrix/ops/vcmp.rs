use crate::matrix::*;
use core::cmp::PartialOrd;

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: PartialEq,
{
    pub fn veq(self, other: Matrix<T, M, N>) -> Matrix<bool, M, N> {
        self.zip(other).map(|(x, y)| x == y)
    }
    pub fn vne(self, other: Matrix<T, M, N>) -> Matrix<bool, M, N> {
        self.zip(other).map(|(x, y)| x != y)
    }
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: PartialOrd,
{
    pub fn vlt(self, other: Matrix<T, M, N>) -> Matrix<bool, M, N> {
        self.zip(other).map(|(x, y)| x < y)
    }
    pub fn vle(self, other: Matrix<T, M, N>) -> Matrix<bool, M, N> {
        self.zip(other).map(|(x, y)| x <= y)
    }
    pub fn vgt(self, other: Matrix<T, M, N>) -> Matrix<bool, M, N> {
        self.zip(other).map(|(x, y)| x > y)
    }
    pub fn vge(self, other: Matrix<T, M, N>) -> Matrix<bool, M, N> {
        self.zip(other).map(|(x, y)| x >= y)
    }
}
