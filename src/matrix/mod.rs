mod base;
pub use base::*;

mod ops;
pub use ops::*;

mod transpose;
pub use transpose::*;

mod product;
pub use product::*;

mod square;
pub use square::*;

#[cfg(test)]
mod tests;

pub use crate::traits::{Dot, Outer};

pub type Matrix2x2<T> = Matrix<T, 2, 2>;
pub type Matrix2x3<T> = Matrix<T, 2, 3>;
pub type Matrix2x4<T> = Matrix<T, 2, 4>;
pub type Matrix3x2<T> = Matrix<T, 3, 2>;
pub type Matrix3x3<T> = Matrix<T, 3, 3>;
pub type Matrix3x4<T> = Matrix<T, 3, 4>;
pub type Matrix4x2<T> = Matrix<T, 4, 2>;
pub type Matrix4x3<T> = Matrix<T, 4, 3>;
pub type Matrix4x4<T> = Matrix<T, 4, 4>;
