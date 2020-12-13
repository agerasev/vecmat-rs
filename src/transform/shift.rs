use core::ops::{Neg};
use num_traits::{Zero, Num};
use crate::{vector::*, transform::*};
#[cfg(feature = "random")]
use rand::{prelude::*};
#[cfg(feature = "random")]
use crate::distributions::*;
#[cfg(feature = "approx")]
use approx::{AbsDiffEq, abs_diff_eq};


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

	impl<T> $X<T> for $Z<T> where T: Neg<Output=T> + Num + Clone {
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

	#[cfg(feature = "random")]
	impl<T> Distribution<$Z<T>> for StandardNormal where StandardNormal: Distribution<$V<T>> {
		fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $Z<T> {
			$Z::from(self.sample(rng))
		}
	}

	#[cfg(feature = "approx")]
	impl<T> AbsDiffEq for $Z<T> where T: AbsDiffEq<Epsilon=T> + Clone {
		type Epsilon = T;
		fn default_epsilon() -> Self::Epsilon {
			T::default_epsilon()
		}
		fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
			abs_diff_eq!(self.pos, other.pos, epsilon=epsilon)
		}
	}
) }

shift!(Shift2, Transform2, Vector2);
shift!(Shift3, Transform3, Vector3);
shift!(Shift4, Transform4, Vector4);
