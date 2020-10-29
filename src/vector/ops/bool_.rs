use core::{
	ops::{
		Not, BitAnd, BitOr, BitXor,
		BitAndAssign, BitOrAssign, BitXorAssign,
	},
};
use crate::vector::*;


impl<const N: usize> Not for Vector<bool, N> {
    type Output = Vector<bool, N>;
    fn not(self) -> Self::Output {
        self.map(|x| !x)
    }
}

impl<const N: usize> BitAnd for Vector<bool, N> {
    type Output = Vector<bool, N>;
    fn bitand(self, other: Vector<bool, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x & y)
    }
}
impl<const N: usize> BitOr for Vector<bool, N> {
    type Output = Vector<bool, N>;
    fn bitor(self, other: Vector<bool, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x | y)
    }
}
impl<const N: usize> BitXor for Vector<bool, N> {
    type Output = Vector<bool, N>;
    fn bitxor(self, other: Vector<bool, N>) -> Self::Output {
        self.zip(other).map(|(x, y)| x ^ y)
    }
}

impl<const N: usize> BitAndAssign for Vector<bool, N> {
    fn bitand_assign(&mut self, other: Vector<bool, N>) {
        self.iter_mut().zip(other.into_iter()).for_each(|(s, y)| *s &= y)
    }
}
impl<const N: usize> BitOrAssign for Vector<bool, N> {
    fn bitor_assign(&mut self, other: Vector<bool, N>) {
        self.iter_mut().zip(other.into_iter()).for_each(|(s, y)| *s |= y)
    }
}
impl<const N: usize> BitXorAssign for Vector<bool, N> {
    fn bitxor_assign(&mut self, other: Vector<bool, N>) {
        self.iter_mut().zip(other.into_iter()).for_each(|(s, y)| *s ^= y)
    }
}


impl<const N: usize> Vector<bool, N> {
    pub fn any(self) -> bool {
        for i in 0..N {
            if self[i] {
                return true;
            }
        }
        false
    }
}
impl<const N: usize> Vector<bool, N> {
    pub fn all(self) -> bool {
        for i in 0..N {
            if !self[i] {
                return false;
            }
        }
        true
    }
}
