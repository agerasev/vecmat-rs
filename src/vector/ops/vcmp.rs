use crate::Vector;
use core::cmp::PartialOrd;

impl<T, const N: usize> Vector<T, N>
where
    T: PartialEq,
{
    pub fn veq(self, other: Vector<T, N>) -> Vector<bool, N> {
        self.zip(other).map(|(x, y)| x == y)
    }
    pub fn vne(self, other: Vector<T, N>) -> Vector<bool, N> {
        self.zip(other).map(|(x, y)| x != y)
    }
}
impl<T, const N: usize> Vector<T, N>
where
    T: PartialOrd,
{
    pub fn vlt(self, other: Vector<T, N>) -> Vector<bool, N> {
        self.zip(other).map(|(x, y)| x < y)
    }
    pub fn vle(self, other: Vector<T, N>) -> Vector<bool, N> {
        self.zip(other).map(|(x, y)| x <= y)
    }
    pub fn vgt(self, other: Vector<T, N>) -> Vector<bool, N> {
        self.zip(other).map(|(x, y)| x > y)
    }
    pub fn vge(self, other: Vector<T, N>) -> Vector<bool, N> {
        self.zip(other).map(|(x, y)| x >= y)
    }
}
