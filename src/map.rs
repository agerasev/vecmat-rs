use crate::vec::*;
use crate::mat::*;
use num_traits::{Num, Float, Signed};

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
			pub fn map_vec(&self, v: $Vec<T>) -> $Vec<T> {
				self.linear.dot(v) + self.shift
			}
			pub fn map_mat(&self, m: $Mat<T>) -> $Mat<T> {
				self.linear.dot(m)
			}
			pub fn chain(&self, a: &$Map<T>) -> Self {
				Self::from(self.map_mat(a.linear), self.map_vec(a.shift))
			}
		}
	)
}

affine!(Affine2, Mat2, Vec2);
affine!(Affine3, Mat3, Vec3);
affine!(Affine4, Mat4, Vec4);


#[cfg(test)]
#[test]
fn dummy_test() {
    Affine3::<f64>::new();
}
