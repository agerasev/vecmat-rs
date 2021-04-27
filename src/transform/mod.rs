mod affine;
mod chain;
mod linear;
mod rotation;
mod shift;
mod scale;
mod moebius;

pub use affine::*;
pub use chain::*;
pub use linear::*;
pub use rotation::*;
pub use shift::*;
pub use scale::*;
pub use moebius::*;

/// General tansformation trait.
///
/// It's assumed that transfomation is a group.
pub trait Transform<T> {
    /// Identity transformation.
    fn identity() -> Self;
    /// Inverse transformation.
    fn inv(self) -> Self;

    /// Perform the transformation itself.
    fn apply(&self, pos: T) -> T;
    /// Find transformation directional derivative at specified point.
    fn deriv(&self, pos: T, dir: T) -> T;

    /// Chain two transformations into a new one.
    ///
    /// `C = A.chain(B)` means that `C(x) = A(B(x))`.
    fn chain(self, other: Self) -> Self;
}

pub trait Directional<T>: Transform<T> {
    /// Returns the result of the direction transformation at the specified position.
    fn apply_dir(&self, pos: T, dir: T) -> T;

    /// Returns the result of the normal transformation at the specified position.
    /// 
    /// If the transformation is conformal then the result is equal to `apply_dir`.
    fn apply_normal(&self, pos: T, normal: T) -> T;
}
