mod convert;
mod init;
mod iter;

pub use iter::*;

use crate::{traits::ImplicitClone, vector::Vector};

/// Matrix with fixed dimensions.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: Vector<Vector<T, N>, M>,
}

impl<T: ImplicitClone, const M: usize, const N: usize> ImplicitClone for Matrix<T, M, N> {}
