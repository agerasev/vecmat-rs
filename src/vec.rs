use std::ops::{Index, IndexMut, Neg, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use num::{Num, Zero, Signed};


macro_rules! vn_struct {
	($V:ident, $N:expr) => (
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy)]
		pub struct $V<T: Copy> {
			pub d: [T; $N],
		}
	)
}

macro_rules! vn_index {
	($V:ident, $N:expr) => (
		impl<T> Index<usize> for $V<T> where T: Copy {
			type Output = T;
			fn index(&self, i: usize) -> &Self::Output {
				&self.d[i]
			}
		}

		impl<T> IndexMut<usize> for $V<T> where T: Copy {
			fn index_mut(&mut self, i: usize) -> &mut Self::Output {
				&mut self.d[i]
			}
		}
	)
}

macro_rules! vn_zero {
	($V:ident, $N:expr) => (
		impl<T> Zero for $V<T> where T: Copy + Num + Zero {
			fn zero() -> Self {
				$V::<T> { d: [T::zero(); $N] }
			}

			fn is_zero(&self) -> bool {
				for i in 0..$N {
					if !self.d[i].is_zero() {
						return false;
					}
				}
				true
			}
		}
	)
}

macro_rules! vn_new {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Zero {
			pub fn new() -> Self {
				$V::<T>::zero()
			}
		}
	)
}

macro_rules! vn_from {
	($V:ident, $N:expr) => (
		impl<T> From<[T; $N]> for $V<T> where T: Copy {
			fn from(a: [T; $N]) -> Self {
				$V::<T> { d: a }
			}
		}
	)
}

macro_rules! vn_neg {
	($V:ident, $N:expr) => (
		impl<T> Neg for $V<T> where T: Copy + Num + Zero + Signed {
			type Output = $V<T>;
			fn neg(self) -> Self::Output {
				let mut out = $V::<T>::new();
				for i in 0..$N {
					out.d[i] = -self.d[i];
				}
				out
			}
		}
	)
}

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a*$b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a/$b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a%$b }) }

macro_rules! vn_vec_op {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<$V<T>> for $V<T> where T: Copy + Num + Zero {
			type Output = $V<T>;
			fn $method(self, vec: $V<T>) -> Self::Output {
				let mut out = $V::<T>::new();
				for i in 0..$N {
					out.d[i] = $op!(self.d[i], vec.d[i]);
				}
				out
			}
		}
	)
}

macro_rules! vn_scal_op {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Num {
			type Output = $V<T>;
			fn $method(self, a: T) -> Self::Output {
				let mut out = self;
				for i in 0..$N {
					out.d[i] = $op!(self.d[i], a);
				}
				out
			}
		}
	)
}

macro_rules! vn_vec_op_assign {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<$V<T>> for $V<T> where T: Copy + Num {
			fn $method(&mut self, vec: $V<T>) {
				for i in 0..$N {
					self.d[i] = $op!(self.d[i], vec.d[i]);
				}
			}
		}
	)
}

macro_rules! vn_scal_op_assign {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Num {
			fn $method(&mut self, a: T) {
				for i in 0..$N {
					self.d[i] = $op!(self.d[i], a);
				}
			}
		}
	)
}

macro_rules! vn_dot {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Zero {
			pub fn dot(self, vec: $V<T>) -> T {
				let mut out = T::zero();
				for i in 0..$N {
					out = out + self.d[i]*vec.d[i];
				}
				out
			}
		}
	)
}

macro_rules! vn_all {
	($V:ident, $N:expr) => (
		vn_struct!($V, $N);
		vn_index!($V, $N);
		vn_zero!($V, $N);
		vn_new!($V, $N);
		vn_from!($V, $N);
		vn_neg!($V, $N);
		vn_vec_op!($V, $N, Add, add, op_add);
		vn_vec_op!($V, $N, Sub, sub, op_sub);
		vn_vec_op!($V, $N, Mul, mul, op_mul);
		vn_vec_op!($V, $N, Div, div, op_div);
		vn_vec_op!($V, $N, Rem, rem, op_rem);
		vn_scal_op!($V, $N, Mul, mul, op_mul);
		vn_scal_op!($V, $N, Div, div, op_div);
		vn_scal_op!($V, $N, Rem, rem, op_rem);
		vn_vec_op_assign!($V, $N, AddAssign, add_assign, op_add);
		vn_vec_op_assign!($V, $N, SubAssign, sub_assign, op_sub);
		vn_vec_op_assign!($V, $N, MulAssign, mul_assign, op_mul);
		vn_vec_op_assign!($V, $N, DivAssign, div_assign, op_div);
		vn_vec_op_assign!($V, $N, RemAssign, rem_assign, op_rem);
		vn_scal_op_assign!($V, $N, MulAssign, mul_assign, op_mul);
		vn_scal_op_assign!($V, $N, DivAssign, div_assign, op_div);
		vn_scal_op_assign!($V, $N, RemAssign, rem_assign, op_rem);
		vn_dot!($V, $N);
	)
}

vn_all!(vec2, 2);
vn_all!(vec3, 3);
vn_all!(vec4, 4);

impl<T> vec3<T> where T: Copy + Num {
	pub fn cross(self, vec: vec3<T>) -> vec3<T> {
		let a = &self.d;
		let b = &vec.d;
		vec3::<T> { d: [ a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0] ] }
	}
}