#[cfg(feature = "approx")]
mod approx;
mod base;
#[cfg(feature = "rand")]
mod distr;
mod format;
mod ops;
mod product;
mod square;
#[cfg(test)]
mod tests;
mod transpose;
mod math;

pub use base::*;
#[cfg(feature = "rand")]
pub use distr::*;

pub type Matrix2x2<T> = Matrix<T, 2, 2>;
pub type Matrix2x3<T> = Matrix<T, 2, 3>;
pub type Matrix2x4<T> = Matrix<T, 2, 4>;
pub type Matrix3x2<T> = Matrix<T, 3, 2>;
pub type Matrix3x3<T> = Matrix<T, 3, 3>;
pub type Matrix3x4<T> = Matrix<T, 3, 4>;
pub type Matrix4x2<T> = Matrix<T, 4, 2>;
pub type Matrix4x3<T> = Matrix<T, 4, 3>;
pub type Matrix4x4<T> = Matrix<T, 4, 4>;
