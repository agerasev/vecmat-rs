use core::ops::{Neg};
use num_traits::{One, Num};
use crate::*;


macro_rules! linear { ($Z:ident, $X:ident, $W:ident, $V:ident) => (
	/// Linear transformation.
	#[repr(transparent)]
	#[derive(Clone, Copy, PartialEq, Debug)]
	pub struct $Z<T> {
		lin: $W<T>,
	}

	impl<T> From<$W<T>> for $Z<T> {
		fn from(lin: $W<T>) -> Self {
			Self { lin }
		}
	}
	impl<T> Into<$W<T>> for $Z<T> {
		fn into(self) -> $W<T> {
			self.lin
		}
	}

	impl<T> $X<T> for $Z<T> where T: Neg<Output=T> + Num + Clone {
		fn identity() -> Self {
			Self { lin: $W::one() }
		}
		fn inverse(self) -> Self {
			Self { lin: self.lin.inverse() }
		}
		fn apply(self, pos: $V<T>) -> $V<T> {
			self.lin.dot(pos)
		}
		fn deriv(self, _pos: $V<T>, dir: $V<T>) -> $V<T> {
			self.apply(dir)
		}
		fn chain(self, other: Self) -> Self {
			Self { lin: self.lin.dot(other.lin) }
		}
	}
) }

linear!(Linear2, Transform2, Matrix2x2, Vector2);
linear!(Linear3, Transform3, Matrix3x3, Vector3);
linear!(Linear4, Transform4, Matrix4x4, Vector4);
