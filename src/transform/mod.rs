mod shift;
pub use shift::*;

mod linear;
pub use linear::*;

mod affine;
pub use affine::*;

mod rotation;
pub use rotation::*;


use crate::vector::*;

macro_rules! transform { ($X:ident, $V:ident) => (
    /// General N-dimensional tansformation trait.
    ///
    /// It's assumed that transfomation is a group.
    pub trait $X<T> {
        /// Identity transformation.
        fn identity() -> Self;
        /// Inverse transformation.
        fn inverse(self) -> Self;

        /// Perform the transformation itself.
        fn apply(self, pos: $V<T>) -> $V<T>;
        /// Find transformation directional derivative at specified point.
        fn deriv(self, pos: $V<T>, dir: $V<T>) -> $V<T>;

        /// Chain two transformations into a new one.
        fn chain(self, other: Self) -> Self;
    }
) }

transform!(Transform2, Vector2);
transform!(Transform3, Vector3);
transform!(Transform4, Vector4);
