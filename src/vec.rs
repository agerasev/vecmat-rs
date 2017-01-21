use std::ops::{Index, IndexMut, Neg, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, Not};
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use num::{Num, Zero, Signed};

macro_rules! vn_struct {
	($V:ident, $N:expr) => (
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy, PartialEq)]
		pub struct $V<T: Copy> {
			pub d: [T; $N],
		}
	)
}

macro_rules! vn_fmt {
	($V:ident, $N:expr) => (
		impl<T> Display for $V<T> where T: Copy + Display {
			fn fmt(&self, f: &mut Formatter) -> FmtResult {
				try!(write!(f, "{} [ ", stringify!($V)));
				for i in 0..$N {
					try!(write!(f, "\t{}, ", self[i]));
				}
				try!(write!(f, "]"));
				Ok(())
			}
		}

		impl<T> Debug for $V<T> where T: Copy + Display {
			#[inline]
			fn fmt(&self, f: &mut Formatter) -> FmtResult {
				Display::fmt(self, f)
			}
		}
	)
}

macro_rules! vn_map {
	[$i:ident; $v:expr; $V:ident, $N:expr] => ({
		let mut out = $V::new();
		for $i in 0..$N {
			out[$i] = $v;
		}
		out
	})
}

macro_rules! vn_index {
	($V:ident, $N:expr) => (
		impl<T> Index<usize> for $V<T> where T: Copy {
			type Output = T;
			#[inline]
			fn index(&self, i: usize) -> &Self::Output {
				&self.d[i]
			}
		}

		impl<T> IndexMut<usize> for $V<T> where T: Copy {
			#[inline]
			fn index_mut(&mut self, i: usize) -> &mut Self::Output {
				&mut self.d[i]
			}
		}
	)
}

macro_rules! vn_new {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Default {
			pub fn new() -> Self {
				$V::<T> { d: [T::default(); $N] }
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
		impl<T> Neg for $V<T> where T: Copy + Default + Num + Signed {
			type Output = $V<T>;
			fn neg(self) -> Self::Output {
				vn_map![i; -self[i]; $V, $N]
			}
		}
	)
}

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a*$b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a/$b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a%$b }) }

macro_rules! vn_op_vec {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<$V<T>> for $V<T> where T: Copy + Default + Num {
			type Output = $V<T>;
			fn $method(self, vec: $V<T>) -> Self::Output {
				vn_map![i; $op!(self[i], vec[i]); $V, $N]
			}
		}
	)
}

macro_rules! vn_op_scal {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Default + Num {
			type Output = $V<T>;
			fn $method(self, a: T) -> Self::Output {
				vn_map![i; $op!(self[i], a); $V, $N]
			}
		}
	)
}

macro_rules! vn_op_vec_assign {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<$V<T>> for $V<T> where T: Copy + Default + Num {
			fn $method(&mut self, vec: $V<T>) {
				*self = $op!(*self, vec);
			}
		}
	)
}

macro_rules! vn_op_scal_assign {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Default + Num {
			fn $method(&mut self, a: T) {
				*self = $op!(*self, a);
			}
		}
	)
}

macro_rules! vn_ops_all {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		vn_op_vec!($V, $N, $Trait, $method, $op);
		vn_op_scal!($V, $N, $Trait, $method, $op);
	)
}

macro_rules! vn_ops_all_assign {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		vn_op_vec_assign!($V, $N, $Trait, $method, $op);
		vn_op_scal_assign!($V, $N, $Trait, $method, $op);
	)
}

macro_rules! vn_dot {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Default + Num {
			pub fn dot(self, vec: $V<T>) -> T {
				let mut out = T::zero();
				for i in 0..$N {
					out = out + self[i]*vec[i];
				}
				out
			}
		}
	)
}

macro_rules! vn_zero {
	($V:ident, $N:expr) => (
		impl<T> Zero for $V<T> where T: Copy + Default + Num {
			fn zero() -> Self {
				$V::<T> { d: [T::zero(); $N] }
			}

			fn is_zero(&self) -> bool {
				for i in 0..$N {
					if !self[i].is_zero() {
						return false;
					}
				}
				true
			}
		}
	)
}

macro_rules! vn_bool_not {
	($V:ident, $N:expr) => (
		impl Not for $V<bool> {
			type Output = $V<bool>;
			fn not(self) -> Self::Output {
				vn_map![i; !self[i]; $V, $N]
			}
		}
	)
}

macro_rules! vn_bool_any {
	($V:ident, $N:expr) => (
		impl $V<bool> {
			pub fn any(self) -> bool {
				for i in 0..$N {
					if self[i] {
						return true;
					}
				}
				false
			}
		}
	)
}

macro_rules! vn_bool_all {
	($V:ident, $N:expr) => (
		impl $V<bool> {
			pub fn all(self) -> bool {
				for i in 0..$N {
					if !self[i] {
						return false;
					}
				}
				true
			}
		}
	)
}

macro_rules! vn_vec_eq {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + PartialEq {
			pub fn eq_(&self, vec: $V<T>) -> $V<bool> {
				vn_map![i; self[i] == vec[i]; $V, $N]
			}
			pub fn ne_(&self, vec: $V<T>) -> $V<bool> {
				vn_map![i; self[i] != vec[i]; $V, $N]
			}
		}
	)
}

macro_rules! vn_vec_cmp {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + PartialOrd {
			pub fn lt_(&self, vec: $V<T>) -> $V<bool> {
				vn_map![i; self[i] < vec[i]; $V, $N]
			}
			pub fn le_(&self, vec: $V<T>) -> $V<bool> {
				vn_map![i; self[i] <= vec[i]; $V, $N]
			}
			pub fn gt_(&self, vec: $V<T>) -> $V<bool> {
				vn_map![i; self[i] > vec[i]; $V, $N]
			}
			pub fn ge_(&self, vec: $V<T>) -> $V<bool> {
				vn_map![i; self[i] >= vec[i]; $V, $N]
			}
		}
	)
}

macro_rules! vn_all {
	($V:ident, $N:expr) => (
		vn_struct!($V, $N);
		vn_fmt!($V, $N);
		vn_index!($V, $N);
		vn_new!($V, $N);
		vn_from!($V, $N);

		vn_neg!($V, $N);

		vn_op_vec!($V, $N, Add, add, op_add);
		vn_op_vec!($V, $N, Sub, sub, op_sub);
		vn_op_vec_assign!($V, $N, AddAssign, add_assign, op_add);
		vn_op_vec_assign!($V, $N, SubAssign, sub_assign, op_sub);
		vn_ops_all!($V, $N, Mul, mul, op_mul);
		vn_ops_all!($V, $N, Div, div, op_div);
		vn_ops_all!($V, $N, Rem, rem, op_rem);
		vn_ops_all_assign!($V, $N, MulAssign, mul_assign, op_mul);
		vn_ops_all_assign!($V, $N, DivAssign, div_assign, op_div);
		vn_ops_all_assign!($V, $N, RemAssign, rem_assign, op_rem);

		vn_dot!($V, $N);

		vn_zero!($V, $N);
		vn_bool_not!($V, $N);
		vn_bool_any!($V, $N);
		vn_bool_all!($V, $N);
		vn_vec_eq!($V, $N);
		vn_vec_cmp!($V, $N);
	)
}

vn_all!(vec2, 2);
vn_all!(vec3, 3);
vn_all!(vec4, 4);

impl<T> vec3<T> where T: Copy + Num {
	pub fn cross(self, vec: vec3<T>) -> vec3<T> {
		let a = &self;
		let b = &vec;
		vec3::<T> { d: [ a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0] ] }
	}
}