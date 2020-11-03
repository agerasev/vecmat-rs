use crate::{vector::*, transform::*};
use num_traits::{Zero, Signed};


macro_rules! shift { ($Z:ident, $X:ident, $V:ident) => (
	/// Shift transformation.
	#[repr(transparent)]
	#[derive(Clone, Copy, PartialEq, Debug)]
	pub struct $Z<T> {
		pos: $V<T>,
	}

	impl<T> From<$V<T>> for $Z<T> {
		fn from(pos: $V<T>) -> Self {
			Self { pos }
		}
	}
	impl<T> Into<$V<T>> for $Z<T> {
		fn into(self) -> $V<T> {
			self.pos
		}
	}

	impl<T> $X<T> for $Z<T> where T: Signed + Zero {
		fn identity() -> Self {
			Self { pos: $V::zero() }
		}
		fn inverse(self) -> Self {
			Self { pos: -self.pos }
		}
		fn apply(self, pos: $V<T>) -> $V<T> {
			pos - self.pos
		}
		fn deriv(self, _pos: $V<T>, dir: $V<T>) -> $V<T> {
			dir
		}
		fn chain(self, other: Self) -> Self {
			Self { pos: self.pos + other.pos }
		}
	}
) }

shift!(Shift2, Transform2, Vector2);
shift!(Shift3, Transform3, Vector3);
shift!(Shift4, Transform4, Vector4);
