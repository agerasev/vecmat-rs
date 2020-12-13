use core::ops::{Neg};
use num_traits::{Zero, Num, Float, FromPrimitive};
use crate::*;


/// 3-dimensional rotation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rotation3<T> {
	quat: Quaternion<T>,
}

impl<T> From<Quaternion<T>> for Rotation3<T> {
	fn from(quat: Quaternion<T>) -> Self {
		Self { quat }
	}
}
impl<T> Into<Quaternion<T>> for Rotation3<T> {
	fn into(self) -> Quaternion<T> {
		self.quat
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
		Self { quat: Quaternion::zero() }
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
