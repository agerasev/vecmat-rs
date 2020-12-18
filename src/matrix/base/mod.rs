mod convert;
mod init;
mod iter;

pub use convert::*;
pub use init::*;
pub use iter::*;

use crate::vector::*;

/// Matrix with fixed dimensions.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: Vector<Vector<T, N>, M>,
}
