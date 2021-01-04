use crate::Vector;
use num_integer::{self as int, Integer};

impl<T, const N: usize> Vector<T, N>
where
    T: Integer,
{
    pub fn div_floor(self, other: Vector<T, N>) -> Vector<T, N> {
        self.zip(other).map(|(x, y)| int::div_floor(x, y))
    }
    pub fn mod_floor(self, other: Vector<T, N>) -> Vector<T, N> {
        self.zip(other).map(|(x, y)| int::mod_floor(x, y))
    }
    pub fn div_mod_floor(self, other: Vector<T, N>) -> (Vector<T, N>, Vector<T, N>) {
        self.zip(other)
            .map(|(x, y)| int::div_mod_floor(x, y))
            .unzip()
    }
}
