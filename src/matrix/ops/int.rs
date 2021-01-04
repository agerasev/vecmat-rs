use crate::Matrix;
use num_integer::{self as nint, Integer};

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Integer,
{
    pub fn div_floor(self, other: Matrix<T, M, N>) -> Matrix<T, M, N> {
        self.zip(other).map(|(x, y)| nint::div_floor(x, y))
    }
    pub fn mod_floor(self, other: Matrix<T, M, N>) -> Matrix<T, M, N> {
        self.zip(other).map(|(x, y)| nint::mod_floor(x, y))
    }
    pub fn div_mod_floor(self, other: Matrix<T, M, N>) -> (Matrix<T, M, N>, Matrix<T, M, N>) {
        self.zip(other)
            .map(|(x, y)| nint::div_mod_floor(x, y))
            .unzip()
    }
}
