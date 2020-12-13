use core::ops::{Neg};
use num_traits::{One, Num, Float, FromPrimitive};
use crate::*;


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
