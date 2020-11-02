use core::{
	ops::{
		Not, BitAnd, BitOr, BitXor,
		BitAndAssign, BitOrAssign, BitXorAssign,
	},
	iter::{IntoIterator},
};
use crate::matrix::*;


impl<const M: usize, const N: usize> Not for Matrix<bool, M, N> {
    type Output = Matrix<bool, M, N>;
    fn not(self) -> Self::Output {
        self.map(|x| !x)
    }
}

impl<const M: usize, const N: usize> BitAnd for Matrix<bool, M, N> {
    type Output = Matrix<bool, M, N>;
    fn bitand(self, other: Matrix<bool, M, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x & y)
    }
}
impl<const M: usize, const N: usize> BitOr for Matrix<bool, M, N> {
    type Output = Matrix<bool, M, N>;
    fn bitor(self, other: Matrix<bool, M, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x | y)
    }
}
impl<const M: usize, const N: usize> BitXor for Matrix<bool, M, N> {
    type Output = Matrix<bool, M, N>;
    fn bitxor(self, other: Matrix<bool, M, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x ^ y)
    }
}

impl<const M: usize, const N: usize> BitAndAssign for Matrix<bool, M, N> {
    fn bitand_assign(&mut self, other: Matrix<bool, M, N>) {
        self.iter_mut().zip(other.into_iter()).for_each(|(s, y)| *s &= y)
    }
}
impl<const M: usize, const N: usize> BitOrAssign for Matrix<bool, M, N> {
    fn bitor_assign(&mut self, other: Matrix<bool, M, N>) {
        self.iter_mut().zip(other.into_iter()).for_each(|(s, y)| *s |= y)
    }
}
impl<const M: usize, const N: usize> BitXorAssign for Matrix<bool, M, N> {
    fn bitxor_assign(&mut self, other: Matrix<bool, M, N>) {
        self.iter_mut().zip(other.into_iter()).for_each(|(s, y)| *s ^= y)
    }
}

impl<const M: usize, const N: usize> Matrix<bool, M, N> {
    pub fn any(self) -> bool {
        self.into_iter().any(|x| x)
    }
}
impl<const M: usize, const N: usize> Matrix<bool, M, N> {
    pub fn all(self) -> bool {
        self.into_iter().all(|x| x)
    }
}
