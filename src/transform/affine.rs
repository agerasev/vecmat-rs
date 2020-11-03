use num_traits::{Zero, One, Signed};
use crate::*;


macro_rules! linear { ($Z:ident, $X:ident, $L:ident, $S:ident, $W:ident, $V:ident) => (
	/// Affine transformation.
	#[derive(Clone, Copy, PartialEq, Debug)]
	pub struct $Z<T> {
		lin: $W<T>,
		pos: $V<T>,
	}

	impl<T> $Z<T> {
		/// Construct affine transformation from linear one and shift.
		pub fn new(linear: $L<T>, shift: $S<T>) -> Self {
			Self { lin: linear.into(), pos: shift.into() }
		}
		/// Split into linear and shift components.
		pub fn split(self) -> ($L<T>, $S<T>) {
			(self.lin.into(), self.pos.into())
		}
	}
	impl<T> $Z<T> where T: Clone {
		/// Linear component of the transformation.
		pub fn linear(&self) -> $L<T> {
			self.lin.clone().into()
		}
		/// Shift component of the transformation.
		pub fn shift(&self) -> $S<T> {
			self.pos.clone().into()
		}
	}

	impl<T> $X<T> for $Z<T> where T: Signed + Zero + One + Clone {
		fn identity() -> Self {
			Self{ lin: $W::one(), pos: $V::zero() }
		}
		fn inverse(self) -> Self {
			let ilin = self.lin.inverse();
			Self {
				lin: ilin.clone(),
				pos: ilin.dot(-self.pos),
			}
		}
		fn apply(self, pos: $V<T>) -> $V<T> {
			self.lin.dot(pos) + self.pos
		}
		fn deriv(self, _pos: $V<T>, dir: $V<T>) -> $V<T> {
			self.lin.dot(dir)
		}
		fn chain(self, other: Self) -> Self {
			Self {
				lin: self.lin.clone().dot(other.lin),
				pos: self.lin.dot(other.pos) + self.pos,
			}
		}
	}
) }

linear!(Affine2, Transform2, Linear2, Shift2, Matrix2x2, Vector2);
linear!(Affine3, Transform3, Linear3, Shift3, Matrix3x3, Vector3);
linear!(Affine4, Transform4, Linear4, Shift4, Matrix4x4, Vector4);
