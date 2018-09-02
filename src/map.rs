use vec::*;
use mat::*;
use num::{Num, Float, Signed};

macro_rules! affine {
	($Map:ident, $Mat:ident, $Vec:ident) => (
		#[derive(Clone, Debug, PartialEq)]
		pub struct $Map<T> where T: Copy + Num + Float + Signed {
			pub linear: $Mat<T>,
			pub shift: $Vec<T>,
		}

		impl<T> $Map<T> where T: Copy + Num + Float + Signed {
			pub fn new() -> Self {
				Self { linear: $Mat::one(), shift: $Vec::zero() }
			}
			pub fn from(m: $Mat<T>, v: $Vec<T>) -> Self {
				Self { linear: m, shift: v }
			}
			pub fn inverse(&self) -> Self {
				let inv = self.linear.inverse();
				Self {
					linear: inv,
					shift: -inv.dot(self.shift),
				}
			}
			pub fn map(&self, v: $Vec<T>) -> $Vec<T> {
				self.linear.dot(v) + self.shift
			}
		}
	)
}

affine!(Affine2, Mat2, Vec2);
affine!(Affine3, Mat3, Vec3);
affine!(Affine4, Mat4, Vec4);
