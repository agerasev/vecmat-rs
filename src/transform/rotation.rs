use core::ops::{Neg};
use num_traits::{Zero, One, Num, Float, FromPrimitive};
use hcomplex::{Quaternion};
use crate::*;


/// 3-dimensional rotation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rotation3<T> {
	quat: Quaternion<T>,
}

impl<T> Rotation3<T> {
	fn from_quat(quat: Quaternion<T>) -> Self {
		Self { quat }
	}
}

fn w_vec3_to_quat<T>(w: T, xyz: Vector3<T>) -> Quaternion<T> where T: Clone {
	Quaternion::new2(w, xyz.x(), xyz.y(), xyz.z())
}
fn quat_to_w_vec3<T>(quat: Quaternion<T>) -> (T, Vector3<T>) where T: Clone {
	(quat.w(), Vector3::from([quat.x(), quat.y(), quat.z()]))
}
fn vec4_to_quat<T>(vec: Vector4<T>) -> Quaternion<T> where T: Clone {
	Quaternion::new2(vec.x(), vec.y(), vec.z(), vec.w())
}
fn quat_to_vec4<T>(quat: Quaternion<T>) -> Vector4<T> where T: Clone {
	Vector4::from([quat.w(), quat.x(), quat.y(), quat.z()])
}

impl<T> Rotation3<T> where T: Float + FromPrimitive + Clone {
	pub fn new(axis: Vector3<T>, angle: T) -> Self {
		let half = angle / T::from_f32(2.0).unwrap();
		Self::from_quat(w_vec3_to_quat(half.cos(), axis * half.sin()))
	}
	pub fn axis(&self) -> Vector3<T> {
		quat_to_w_vec3(self.quat).1.normalize()
	}
	pub fn angle(&self) -> T {
		let ax = quat_to_w_vec3(self.quat).1;
		T::from_f32(2.0).unwrap() * ax.length().atan2(q.w())
	}
}

impl<T: Clone + Float> Rotation3<T> {
	fn quaternion(&self) -> Vector4<T> {
		quat_to_vec4(self.quat)
	}
	fn from_quaternion(quat: Vector4<T>) -> Self {
		Self::from_quat(vec4_to_quat(quat))
	}
}

impl<T> Transform3<T> for Rotation3<T> where T: Zero + Neg<Output=T> + Num + Clone {
	fn identity() -> Self {
		Self::from_quat(Quaternion::zero())
	}
	fn inverse(self) -> Self {
		let (w, xyz) = quat_to_w_vec3(self.quat);
		Self::from_quat(w_vec3_to_quat(w, -xyz))
	}
	fn apply(self, pos: Vector3<T>) -> Vector3<T> {
		let qpos = w_vec3_to_quat(T::zero(), pos);
		let new_qpos = self.quat.clone() * qpos * self.quat.conj();
		quat_to_w_vec3(new_qpos).1
	}
	fn deriv(self, _pos: $V<T>, dir: $V<T>) -> $V<T> {
		self.apply(dir)
	}
	fn chain(self, other: Self) -> Self {
		Self { lin: self.lin.dot(other.lin) }
	}
}
