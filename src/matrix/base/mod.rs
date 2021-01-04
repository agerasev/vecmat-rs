mod convert;
mod init;
mod iter;

pub use iter::*;

use crate::vector::Vector;

/// Matrix with fixed dimensions.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: Vector<Vector<T, N>, M>,
}
