use num_integer::{self as nint, Integer};
use crate::vector::*;


impl<T, const N: usize> Vector<T, N> where T: Integer {
	pub fn div_floor(self, other: Vector<T, N>) -> Vector<T, N> {
		self.zip(other).map(|(x, y)| nint::div_floor(x, y))
	}
	pub fn mod_floor(self, other: Vector<T, N>) -> Vector<T, N> {
		self.zip(other).map(|(x, y)| nint::mod_floor(x, y))
	}
	pub fn div_mod_floor(self, other: Vector<T, N>) -> (Vector<T, N>, Vector<T, N>) {
		self.zip(other).map(|(x, y)| nint::div_mod_floor(x, y)).unzip()
	}
}
