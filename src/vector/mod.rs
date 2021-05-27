#[cfg(feature = "approx")]
mod approx;
mod base;
#[cfg(feature = "rand")]
mod distr;
mod dot;
mod ops;
mod spec;
#[cfg(test)]
mod tests;
mod math;

pub use base::*;
#[cfg(feature = "rand")]
pub use distr::*;

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;
pub type Vector8<T> = Vector<T, 8>;
pub type Vector16<T> = Vector<T, 16>;
