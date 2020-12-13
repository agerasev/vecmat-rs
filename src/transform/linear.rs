use core::ops::{Neg};
use num_traits::{One, Num, Float, FromPrimitive};
use crate::*;
#[cfg(feature = "approx")]
use approx::{AbsDiffEq, abs_diff_eq};


macro_rules! linear { ($Z:ident, $X:ident, $W:ident, $V:ident) => (
	/// Linear transformation.
	#[repr(transparent)]
	#[derive(Clone, Copy, PartialEq, Debug)]
	pub struct $Z<T> {
		lin: $W<T>,
	}
	impl<T> $Z<T> {
		pub fn from_matrix(lin: $W<T>) -> Self {
			Self { lin }
		}
		pub fn into_matrix(self) -> $W<T> {
			self.lin
		}
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

	#[cfg(feature = "approx")]
	impl<T> AbsDiffEq for $Z<T> where T: AbsDiffEq<Epsilon=T> + Clone {
		type Epsilon = T;
		fn default_epsilon() -> Self::Epsilon {
			T::default_epsilon()
		}
		fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
			abs_diff_eq!(self.lin, other.lin, epsilon=epsilon)
		}
	}
) }

linear!(Linear2, Transform2, Matrix2x2, Vector2);
linear!(Linear3, Transform3, Matrix3x3, Vector3);
linear!(Linear4, Transform4, Matrix4x4, Vector4);


impl<T> Linear3<T> where T: Float + Clone {
	pub fn look_at(dir: Vector3<T>, up: Vector3<T>) -> Self {
		let right = dir.cross(up).normalize();
		let down = dir.cross(right);
		Self::from(Matrix3x3::from([
			right.into(),
			down.into(),
			dir.into(),
		]))
	}
}
impl<T> Linear3<T> where T: Float + FromPrimitive + Clone {
	pub fn look_at_any(dir: Vector3<T>) -> Self {
		if dir.z().abs() < T::from_f32(0.5).unwrap() {
			Self::look_at(dir, Vector3::from([T::zero(), T::zero(), T::one()]))
		} else {
			Self::look_at(dir, Vector3::from([T::zero(), T::one(), T::zero()]))
		}
	}
}

#[cfg(all(test, feature = "random", feature = "approx"))]
mod tests {
	use rand::{prelude::*};
	use rand_xorshift::XorShiftRng;
	use approx::{assert_abs_diff_eq};
	use crate::{distributions::*};
	use super::*;

	const SAMPLE_ATTEMPTS: usize = 256;

	#[test]
	fn linearity() {
		let mut rng = XorShiftRng::seed_from_u64(0xBEE);

        for _ in 0..SAMPLE_ATTEMPTS {
			
            let m: Matrix3x3<f64> = rng.sample(&StandardNormal);
            let x: Vector3<f64> = rng.sample(&StandardNormal);
            let a: f64 = rng.sample(&StandardNormal);

            assert_abs_diff_eq!(Linear3::from(m * a).apply(x), Linear3::from(m).apply(x * a), epsilon=1e-14);
            assert_abs_diff_eq!(Linear3::from(m * a).apply(x), Linear3::from(m).apply(x) * a, epsilon=1e-14);
        }
	}
	
	#[test]
	fn chaining() {
		let mut rng = XorShiftRng::seed_from_u64(0xBEA);

        for _ in 0..SAMPLE_ATTEMPTS {
            let a = Linear3::<f64>::from_matrix(rng.sample(&StandardNormal));
            let b = Linear3::<f64>::from_matrix(rng.sample(&StandardNormal));
            let c: Vector3<f64> = rng.sample(&StandardNormal);

            assert_abs_diff_eq!(a.chain(Linear3::identity()), a, epsilon=1e-14);
            assert_abs_diff_eq!(Linear3::identity().chain(b), b, epsilon=1e-14);
            assert_abs_diff_eq!(a.chain(b).apply(c), a.apply(b.apply(c)), epsilon=1e-14);
        }
	}
	
	#[test]
	fn inversion() {
		let mut rng = XorShiftRng::seed_from_u64(0xBEB);

        for _ in 0..SAMPLE_ATTEMPTS {
			let a = Linear3::<f64>::from_matrix(rng.sample(&Invertible));
            let x: Vector3<f64> = rng.sample(&StandardNormal);

            assert_abs_diff_eq!(a.chain(a.inverse()), Linear3::identity(), epsilon=1e-12);
            assert_abs_diff_eq!(a.inverse().chain(a), Linear3::identity(), epsilon=1e-12);
            assert_abs_diff_eq!(a.inverse().apply(a.apply(x)), x, epsilon=1e-12);
            assert_abs_diff_eq!(a.apply(a.inverse().apply(x)), x, epsilon=1e-12);
        }
	}
	
	#[test]
	fn look_to_the_direction() {
		let mut rng = XorShiftRng::seed_from_u64(0xBEC);

        for _ in 0..SAMPLE_ATTEMPTS {
            let d: Vector3<f64> = rng.sample(&Unit);
            let m = Linear3::look_at_any(d);

            assert_abs_diff_eq!(m.apply(d), Vector3::from([0.0, 0.0, 1.0]), epsilon=1e-14);
        }
    }
}
