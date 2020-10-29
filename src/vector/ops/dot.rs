use core::ops::{Add, Mul};
use num_traits::{Float};
use crate::{traits::*, vector::*};


impl<T, const N: usize> NormL1 for Vector<T, N> where T: Float {
	type Output = T;
	fn norm_l1(self) -> T {
		self.map(|x| x.abs()).sum()
	}
}
impl<T, const N: usize> NormL2 for Vector<T, N> where T: Float {
	type Output = T;
	fn norm_l2(self) -> T {
		self.map(|x| x*x).sum().sqrt()
	}
}
impl<T, const N: usize> NormLInf for Vector<T, N> where T: Float {
	type Output = T;
	fn norm_l_inf(self) -> T {
		self.map(|x| x.abs()).fold_first(|x, y| x.max(y))
	}
}

impl<T, const N: usize> Dot<Vector<T, N>> for Vector<T, N> where T: Mul<Output=T> + Add<Output=T> {
	type Output = T;
	fn dot(self, other: Vector<T, N>) -> Self::Output {
		self.zip(other).map(|(x, y)| x * y).sum()
	}
}
impl<T, const N: usize> Vector<T, N> where T: Add<Output=T> + Mul<Output=T> + Clone {
	pub fn square_length(self) -> T {
		self.map(|x| x.clone()*x).sum()
	}
}
impl<T, const N: usize> Vector<T, N> where T: Float + Clone {
	pub fn length(self) -> T {
		self.square_length().sqrt()
	}
	pub fn normalize(self) -> Vector<T, N> {
		self / self.length()
	}
}
