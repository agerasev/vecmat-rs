mod linear;
mod shift;
//mod affine;
//mod rotation;

pub use linear::*;
pub use shift::*;
//pub use affine::*;
//pub use rotation::*;

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
