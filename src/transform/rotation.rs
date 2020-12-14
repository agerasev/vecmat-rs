use core::ops::{Neg};
use num_traits::{One, Num, Float, FromPrimitive, FloatConst};
use crate::*;
#[cfg(feature = "random")]
use rand::prelude::*;
#[cfg(feature = "random")]
use rand_distr::{Uniform as RangedUniform, uniform::{SampleUniform}};
#[cfg(feature = "random")]
use crate::distributions::*;
#[cfg(feature = "approx")]
use approx::{AbsDiffEq, abs_diff_eq};


/// Two-dimensional rotation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rotation2<T> {
	comp: Complex<T>,
}

impl<T> Rotation2<T> {
	pub fn from_complex(comp: Complex<T>) -> Self {
		Self { comp }
	}
	pub fn into_complex(self) -> Complex<T> {
		self.comp
	}
}

impl<T> From<Complex<T>> for Rotation2<T> {
	fn from(comp: Complex<T>) -> Self {
		Self::from_complex(comp)
	}
}
impl<T> Into<Complex<T>> for Rotation2<T> {
	fn into(self) -> Complex<T> {
		self.into_complex()
	}
}

impl<T> Rotation2<T> where T: Float + Clone {
	pub fn new(angle: T) -> Self {
		Self { comp: Complex::new(angle.clone().cos(), angle.sin()) }
	}
	pub fn angle(&self) -> T {
		self.comp.im.atan2(self.comp.re)
	}
}

impl<T> Transform2<T> for Rotation2<T> where T: Neg<Output=T> + Num + Clone {
	fn identity() -> Self {
		Self { comp: Complex::one() }
	}
	fn inverse(self) -> Self {
		Self { comp: self.comp.conj() }
	}
	fn apply(self, pos: Vector2<T>) -> Vector2<T> {
		(<Vector2<T> as Into<Complex<T>>>::into(pos) * self.into_complex()).into()
	}
	fn deriv(self, _pos: Vector2<T>, dir: Vector2<T>) -> Vector2<T> {
		self.apply(dir)
	}
	fn chain(self, other: Self) -> Self {
		Self { comp: self.comp * other.comp }
	}
}

impl<T> Rotation2<T> where T: Neg<Output=T> + Clone {
	pub fn to_linear(self) -> Linear2<T> {
		Linear2::from(Matrix2x2::from([
			[self.comp.re.clone(), -self.comp.im.clone()],
			[self.comp.im, self.comp.re],
		]))
	}
}

#[cfg(feature = "random")]
impl<T> Distribution<Rotation2<T>> for Uniform
where RangedUniform<T>: Distribution<T>, T: SampleUniform + Float + FloatConst + FromPrimitive + Clone {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation2<T> {
		Rotation2::new(rng.sample(&RangedUniform::new(T::zero(), T::from_f32(2.0).unwrap() * T::PI())))
	}
}
#[cfg(feature = "approx")]
impl<T> AbsDiffEq for Rotation2<T> where T: AbsDiffEq<Epsilon=T> + Clone {
	type Epsilon = T;
	fn default_epsilon() -> Self::Epsilon {
		T::default_epsilon()
	}
	fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
		abs_diff_eq!(self.comp.re, other.comp.re, epsilon=epsilon.clone()) &&
		abs_diff_eq!(self.comp.im, other.comp.im, epsilon=epsilon)
	}
}


/// Three-dimensional rotation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rotation3<T> {
	quat: Quaternion<T>,
}

impl<T> Rotation3<T> {
	pub fn from_quaternion(quat: Quaternion<T>) -> Self {
		Self { quat }
	}
	pub fn into_quaternion(self) -> Quaternion<T> {
		self.quat
	}
}

impl<T> From<Quaternion<T>> for Rotation3<T> {
	fn from(quat: Quaternion<T>) -> Self {
		Self::from_quaternion(quat)
	}
}
impl<T> Into<Quaternion<T>> for Rotation3<T> {
	fn into(self) -> Quaternion<T> {
		self.into_quaternion()
	}
}

impl<T> Rotation3<T> where T: Float + FromPrimitive + Clone {
	pub fn new(axis: Vector3<T>, angle: T) -> Self {
		let half = angle / T::from_f32(2.0).unwrap();
		Self { quat: Quaternion::from_scalar_and_vector3(half.cos(), axis * half.sin()) }
	}
	pub fn axis(&self) -> Vector3<T> {
		let (_, ax) = self.quat.into();
		ax.normalize()
	}
	pub fn angle(&self) -> T {
		let (w, ax) = self.quat.into();
		T::from_f32(2.0).unwrap() * ax.length().atan2(w)
	}
}

impl<T> Transform3<T> for Rotation3<T> where T: Neg<Output=T> + Num + Clone {
	fn identity() -> Self {
		Self { quat: Quaternion::one() }
	}
	fn inverse(self) -> Self {
		Self { quat: self.quat.conj() }
	}
	fn apply(self, pos: Vector3<T>) -> Vector3<T> {
		let qpos = Quaternion::from_scalar_and_vector3(T::zero(), pos);
		let qres = self.quat.clone() * qpos * self.quat.conj();
		let (_, res) = qres.into();
		res
	}
	fn deriv(self, _pos: Vector3<T>, dir: Vector3<T>) -> Vector3<T> {
		self.apply(dir)
	}
	fn chain(self, other: Self) -> Self {
		Self { quat: self.quat * other.quat }
	}
}

impl<T> Rotation3<T> where T: Float + FromPrimitive + Clone {
	pub fn to_linear(self) -> Linear3<T> {
		let t1 = T::one();
		let t2 = T::from_f32(2.0).unwrap();
		let q = self.quat;
		Linear3::from(Matrix3x3::from([[
			t1 - t2*q.y()*q.y() - t2*q.z()*q.z(),
			t2*q.x()*q.y() - t2*q.z()*q.w(),
			t2*q.x()*q.z() + t2*q.y()*q.w(),
		], [
			t2*q.x()*q.y() + t2*q.z()*q.w(),
			t1 - t2*q.x()*q.x() - t2*q.z()*q.z(),
			t2*q.y()*q.z() - t2*q.x()*q.w(),
		], [
			t2*q.x()*q.z() - t2*q.y()*q.w(),
			t2*q.y()*q.z() + t2*q.x()*q.w(),
			t1 - t2*q.x()*q.x() - t2*q.y()*q.y(),
		]]))
	}
}

#[cfg(feature = "random")]
impl<T> Distribution<Rotation3<T>> for Uniform where
	Unit: Distribution<Vector3<T>>,
	RangedUniform<T>: Distribution<T>,
	T: SampleUniform + Float + FloatConst + FromPrimitive + Clone
{
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation3<T> {
		Rotation3::new(
			rng.sample(&Unit),
			rng.sample(&RangedUniform::new(T::zero(), T::from_f32(2.0).unwrap() * T::PI())),
		)
	}
}
#[cfg(feature = "approx")]
impl<T> AbsDiffEq for Rotation3<T> where T: AbsDiffEq<Epsilon=T> + Clone {
	type Epsilon = T;
	fn default_epsilon() -> Self::Epsilon {
		T::default_epsilon()
	}
	fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
		abs_diff_eq!(self.quat, other.quat, epsilon=epsilon)
	}
}


#[cfg(all(test, feature = "approx", feature = "random"))]
mod tests {
	use rand_xorshift::XorShiftRng;
	use approx::{assert_abs_diff_eq};
	use super::*;

	const SAMPLE_ATTEMPTS: usize = 256;
	const EPS: f64 = 1e-14;

	mod r2d {
		use super::*;

		#[test]
		fn mapping() {
			let mut rng = XorShiftRng::seed_from_u64(0x2DA);
			for _ in 0..SAMPLE_ATTEMPTS {
				let r: Rotation2<f64> = rng.sample(&Uniform);
				let a: Vector2<f64> = rng.sample(&StandardNormal);
				let b = r.apply(a);
				assert_abs_diff_eq!(a.length(), b.length(), epsilon=EPS);
				assert_abs_diff_eq!(a.dot(b)/a.square_length(), r.angle().cos(), epsilon=EPS);
			}
		}
		
		#[test]
		fn chaining() {
			let mut rng = XorShiftRng::seed_from_u64(0x2DB);
			for _ in 0..SAMPLE_ATTEMPTS {
				let a: Rotation2<f64> = rng.sample(&Uniform);
				let b: Rotation2<f64> = rng.sample(&Uniform);
				let v: Vector2<f64> = rng.sample(&StandardNormal);
				assert_abs_diff_eq!(a.chain(b).apply(v), a.apply(b.apply(v)), epsilon=EPS);
			}
		}
		
		#[test]
		fn inversion() {
			let mut rng = XorShiftRng::seed_from_u64(0x2DC);
			for _ in 0..SAMPLE_ATTEMPTS {
				let r: Rotation2<f64> = rng.sample(&Uniform);
				assert_abs_diff_eq!(r.chain(r.inverse()), Rotation2::identity(), epsilon=EPS);
			}
		}
		
		#[test]
		fn to_linear() {
			let mut rng = XorShiftRng::seed_from_u64(0x2DD);
			for _ in 0..SAMPLE_ATTEMPTS {
				let a: Rotation2<f64> = rng.sample(&Uniform);
				let b: Rotation2<f64> = rng.sample(&Uniform);
				let x: Vector2<f64> = rng.sample(&StandardNormal);
				assert_abs_diff_eq!(a.to_linear().apply(x), a.apply(x), epsilon=EPS);
				assert_abs_diff_eq!(
					a.to_linear().chain(b.to_linear()),
					a.chain(b).to_linear(),
					epsilon=EPS,
				);
			}
		}
	}

	mod r3d {
		use super::*;

		#[test]
		fn mapping() {
			let mut rng = XorShiftRng::seed_from_u64(0x3DA);
			for _ in 0..SAMPLE_ATTEMPTS {
				let r: Rotation3<f64> = rng.sample(&Uniform);
				let mut a: Vector3<f64> = rng.sample(&StandardNormal);
				let mut b = r.apply(a);
				let (axis, angle) = (r.axis(), r.angle());
				assert_abs_diff_eq!(a.length(), b.length(), epsilon=EPS);
				a -= axis * a.dot(axis);
				b -= axis * b.dot(axis);
				let aa = a.square_length();
				assert_abs_diff_eq!(a.dot(b) / aa, angle.cos(), epsilon=EPS);
				assert_abs_diff_eq!(a.cross(b) / aa, axis * angle.sin(), epsilon=EPS);
			}
		}
		
		#[test]
		fn chaining() {
			let mut rng = XorShiftRng::seed_from_u64(0x2DB);
			for _ in 0..SAMPLE_ATTEMPTS {
				let a: Rotation3<f64> = rng.sample(&Uniform);
				let b: Rotation3<f64> = rng.sample(&Uniform);
				let v: Vector3<f64> = rng.sample(&StandardNormal);
				assert_abs_diff_eq!(a.chain(b).apply(v), a.apply(b.apply(v)), epsilon=EPS);
			}
		}
		
		#[test]
		fn inversion() {
			let mut rng = XorShiftRng::seed_from_u64(0x2DC);
			for _ in 0..SAMPLE_ATTEMPTS {
				let r: Rotation3<f64> = rng.sample(&Uniform);
				assert_abs_diff_eq!(r.chain(r.inverse()), Rotation3::identity(), epsilon=EPS);
			}
		}
		
		#[test]
		fn to_linear() {
			let mut rng = XorShiftRng::seed_from_u64(0x2DD);
			for _ in 0..SAMPLE_ATTEMPTS {
				let a: Rotation3<f64> = rng.sample(&Uniform);
				let b: Rotation3<f64> = rng.sample(&Uniform);
				let x: Vector3<f64> = rng.sample(&StandardNormal);
				assert_abs_diff_eq!(a.to_linear().apply(x), a.apply(x), epsilon=EPS);
				assert_abs_diff_eq!(
					a.to_linear().chain(b.to_linear()),
					a.chain(b).to_linear(),
					epsilon=EPS,
				);
			}
		}
	}
}