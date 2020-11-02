mod base;
pub use base::*;

mod ops;
pub use ops::*;

mod dot;
pub use dot::*;

mod spec;
pub use spec::*;

#[cfg(test)]
mod tests;

pub use crate::{Dot};

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;
pub type Vector8<T> = Vector<T, 8>;
pub type Vector16<T> = Vector<T, 16>;
