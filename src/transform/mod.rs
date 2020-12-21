mod affine;
mod linear;
mod rotation;
mod shift;

pub use affine::*;
pub use linear::*;
pub use rotation::*;
pub use shift::*;

use crate::Vector;

/// General N-dimensional tansformation trait.
///
/// It's assumed that transfomation is a group.
pub trait Transform<T, const N: usize> {
    /// Identity transformation.
    fn identity() -> Self;
    /// Inverse transformation.
    fn inv(self) -> Self;

    /// Perform the transformation itself.
    fn apply(self, pos: Vector<T, N>) -> Vector<T, N>;
    /// Find transformation directional derivative at specified point.
    fn deriv(self, pos: Vector<T, N>, dir: Vector<T, N>) -> Vector<T, N>;

    /// Chain two transformations into a new one.
    fn chain(self, other: Self) -> Self;
}

// TODO: Use trait aliases when it will be possible.
//trait Transform2<T> = Transform<T, 2>;
//trait Transform3<T> = Transform<T, 3>;
//trait Transform4<T> = Transform<T, 4>;
