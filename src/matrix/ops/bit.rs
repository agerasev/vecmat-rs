use crate::matrix::*;
use core::{
    iter::IntoIterator,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

impl<T, const M: usize, const N: usize> Not for Matrix<T, M, N>
where
    T: Not<Output = T>,
{
    type Output = Matrix<T, M, N>;
    fn not(self) -> Self::Output {
        self.map(|x| !x)
    }
}

impl<T, const M: usize, const N: usize> BitAnd for Matrix<T, M, N>
where
    T: BitAnd<Output = T>,
{
    type Output = Matrix<T, M, N>;
    fn bitand(self, other: Matrix<T, M, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x & y)
    }
}
impl<T, const M: usize, const N: usize> BitOr for Matrix<T, M, N>
where
    T: BitOr<Output = T>,
{
    type Output = Matrix<T, M, N>;
    fn bitor(self, other: Matrix<T, M, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x | y)
    }
}
impl<T, const M: usize, const N: usize> BitXor for Matrix<T, M, N>
where
    T: BitXor<Output = T>,
{
    type Output = Matrix<T, M, N>;
    fn bitxor(self, other: Matrix<T, M, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x ^ y)
    }
}

impl<T, const M: usize, const N: usize> BitAndAssign for Matrix<T, M, N>
where
    T: BitAndAssign,
{
    fn bitand_assign(&mut self, other: Matrix<T, M, N>) {
        self.iter_mut()
            .zip(other.into_iter())
            .for_each(|(s, y)| *s &= y)
    }
}
impl<T, const M: usize, const N: usize> BitOrAssign for Matrix<T, M, N>
where
    T: BitOrAssign,
{
    fn bitor_assign(&mut self, other: Matrix<T, M, N>) {
        self.iter_mut()
            .zip(other.into_iter())
            .for_each(|(s, y)| *s |= y)
    }
}
impl<T, const M: usize, const N: usize> BitXorAssign for Matrix<T, M, N>
where
    T: BitXorAssign,
{
    fn bitxor_assign(&mut self, other: Matrix<T, M, N>) {
        self.iter_mut()
            .zip(other.into_iter())
            .for_each(|(s, y)| *s ^= y)
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
