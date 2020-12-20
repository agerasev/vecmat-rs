use core::ops::{Neg};
use num_traits::{Zero, One, Num};
use crate::*;
#[cfg(feature = "rand")]
use rand::{prelude::*};
#[cfg(feature = "rand")]
use crate::distributions::*;
#[cfg(feature = "approx")]
use approx::{AbsDiffEq, abs_diff_eq};


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

	impl<T> $X<T> for $Z<T> where T: Neg<Output=T> + Num + Clone {
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

	#[cfg(feature = "rand")]
	impl<T> Distribution<$Z<T>> for StandardNormal where StandardNormal: Distribution<$L<T>> + Distribution<$S<T>> {
		fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $Z<T> {
			$Z::new(self.sample(rng), self.sample(rng))
		}
	}
	#[cfg(feature = "rand")]
	impl<T> Distribution<$Z<T>> for Invertible where Invertible: Distribution<$L<T>>, StandardNormal: Distribution<$S<T>> {
		fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $Z<T> {
			$Z::new(rng.sample(&Self), rng.sample(&StandardNormal))
		}
	}

	#[cfg(feature = "approx")]
	impl<T> AbsDiffEq for $Z<T> where T: AbsDiffEq<Epsilon=T> + Clone {
		type Epsilon = T;
		fn default_epsilon() -> Self::Epsilon {
			T::default_epsilon()
		}
		fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
			abs_diff_eq!(self.lin, other.lin, epsilon=epsilon.clone()) &&
			abs_diff_eq!(self.pos, other.pos, epsilon=epsilon)
		}
	}
) }

linear!(Affine2, Transform2, Linear2, Shift2, Matrix2x2, Vector2);
linear!(Affine3, Transform3, Linear3, Shift3, Matrix3x3, Vector3);
linear!(Affine4, Transform4, Linear4, Shift4, Matrix4x4, Vector4);


#[cfg(all(test, feature = "approx"))]
mod tests {
	mod base {
		use num_traits::{Zero, One};
		use approx::*;
		use crate::*;

		macro_rules! identity_test { ($X:ident, $W:ident, $V:ident) => (
			let m = $X::<f64>::identity();
			assert_abs_diff_eq!(Into::<$W<_>>::into(m.linear()), $W::one());
			assert_abs_diff_eq!(Into::<$V<_>>::into(m.shift()), $V::zero());
			let v = $V::fill(1.0);
			assert_abs_diff_eq!(v, m.apply(v));
		) }
		#[test]
		fn identity() {
			identity_test!(Affine2, Matrix2x2, Vector2);
			identity_test!(Affine3, Matrix3x3, Vector3);
			identity_test!(Affine4, Matrix4x4, Vector4);
		}

		macro_rules! inverse_test { ($X:ident, $W:ident, $V:ident) => (
			let m = $X::new(($W::fill(1.0) + $W::one()).into(), $V::fill(1.0).into());
			let v = $V::fill(1.0);
			assert_abs_diff_eq!(v, m.inverse().apply(m.apply(v)));
			assert_abs_diff_eq!(v, m.apply(m.inverse().apply(v)));
		) }
		#[test]
		fn inverse() {
			inverse_test!(Affine2, Matrix2x2, Vector2);
			inverse_test!(Affine3, Matrix3x3, Vector3);
			inverse_test!(Affine4, Matrix4x4, Vector4);
		}

		macro_rules! chain_test {
			($X:ident, $W:ident, $V:ident) => (
				let m0 = $X::new(($W::fill(1.0) + $W::one()).into(), $V::fill(1.0).into());
				let m1 = $X::new(($W::fill(1.0) - $W::one()).into(), $V::indices().map(|i| i as f64).into());
				let v = $V::fill(1.0);
				assert_abs_diff_eq!(m0.apply(m1.apply(v)), m0.chain(m1).apply(v));
				assert_abs_diff_eq!(m1.apply(m0.apply(v)), m1.chain(m0).apply(v));
			)
		}
		#[test]
		fn chain() {
			chain_test!(Affine2, Matrix2x2, Vector2);
			chain_test!(Affine3, Matrix3x3, Vector3);
			chain_test!(Affine4, Matrix4x4, Vector4);
		}
	}

	#[cfg(feature = "rand")]
	mod random {
		use num_traits::{Zero};
		use rand::{prelude::*};
		use rand_xorshift::XorShiftRng;
		use approx::{assert_abs_diff_eq};
		use crate::{*, distributions::*};
		
		const SAMPLE_ATTEMPTS: usize = 256;

		#[test]
		fn chaining() {
			const EPS: f64 = 1e-14;
			let mut rng = XorShiftRng::seed_from_u64(0xCEE);

			for _ in 0..SAMPLE_ATTEMPTS {
				let a: Affine3<f64> = rng.sample(&StandardNormal);
				let b: Affine3<f64> = rng.sample(&StandardNormal);
				let c: Vector3<f64> = rng.sample(&StandardNormal);
				let z = Vector3::<f64>::zero();

				assert_abs_diff_eq!(a.chain(b).apply(c), a.apply(b.apply(c)), epsilon=EPS);
				assert_abs_diff_eq!(a.chain(b).deriv(z, c), a.deriv(z, b.deriv(z, c)), epsilon=EPS);
			}
		}
		
		#[test]
		fn inversion() {
			const EPS: f64 = 1e-12;
			let mut rng = XorShiftRng::seed_from_u64(0xDEE);

			for _ in 0..SAMPLE_ATTEMPTS {
				let a: Affine3<f64> = rng.sample(&Invertible);
				let x: Vector3<f64> = rng.sample(&StandardNormal);
				let z = Vector3::<f64>::zero();
				
				assert_abs_diff_eq!(a.inverse().apply(a.apply(x)), x, epsilon=EPS);
				assert_abs_diff_eq!(a.apply(a.inverse().apply(x)), x, epsilon=EPS);
				assert_abs_diff_eq!(a.inverse().deriv(z, a.deriv(z, x)), x, epsilon=EPS);
				assert_abs_diff_eq!(a.deriv(z, a.inverse().deriv(z, x)), x, epsilon=EPS);
			}
		}
	}
}
