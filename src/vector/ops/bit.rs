use crate::Vector;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

impl<T, const N: usize> Not for Vector<T, N>
where
    T: Not<Output = T>,
{
    type Output = Vector<T, N>;
    fn not(self) -> Self::Output {
        self.map(|x| !x)
    }
}

impl<T, const N: usize> BitAnd for Vector<T, N>
where
    T: BitAnd<Output = T>,
{
    type Output = Vector<T, N>;
    fn bitand(self, other: Vector<T, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x & y)
    }
}
impl<T, const N: usize> BitOr for Vector<T, N>
where
    T: BitOr<Output = T>,
{
    type Output = Vector<T, N>;
    fn bitor(self, other: Vector<T, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x | y)
    }
}
impl<T, const N: usize> BitXor for Vector<T, N>
where
    T: BitXor<Output = T>,
{
    type Output = Vector<T, N>;
    fn bitxor(self, other: Vector<T, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x ^ y)
    }
}

impl<T, const N: usize> BitAndAssign for Vector<T, N>
where
    T: BitAndAssign,
{
    fn bitand_assign(&mut self, other: Vector<T, N>) {
        self.iter_mut()
            .zip(other.into_iter())
            .for_each(|(s, y)| *s &= y)
    }
}
impl<T, const N: usize> BitOrAssign for Vector<T, N>
where
    T: BitOrAssign,
{
    fn bitor_assign(&mut self, other: Vector<T, N>) {
        self.iter_mut()
            .zip(other.into_iter())
            .for_each(|(s, y)| *s |= y)
    }
}
impl<T, const N: usize> BitXorAssign for Vector<T, N>
where
    T: BitXorAssign,
{
    fn bitxor_assign(&mut self, other: Vector<T, N>) {
        self.iter_mut()
            .zip(other.into_iter())
            .for_each(|(s, y)| *s ^= y)
    }
}

impl<const N: usize> Vector<bool, N> {
    pub fn any(self) -> bool {
        self.into_iter().any(|x| x)
    }
}
impl<const N: usize> Vector<bool, N> {
    pub fn all(self) -> bool {
        self.into_iter().all(|x| x)
    }
}
