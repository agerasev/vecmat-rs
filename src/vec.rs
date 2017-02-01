use std::ops::{
	Index, IndexMut, 
	Neg, Add, Sub, Mul, Div, Rem, 
	AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, 
	Not, BitAnd, BitOr, BitXor,
	BitAndAssign, BitOrAssign, BitXorAssign,
};
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
pub use num::{Num, Zero, Signed, Float};

macro_rules! vec_struct {
	($V:ident, $N:expr) => (
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy, PartialEq)]
		pub struct $V<T: Copy> {
			pub d: [T; $N],
		}
	)
}

macro_rules! vec_data {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy {
			pub fn data(&self) -> &[T; $N] {
				&self.d
			}
		}
	)
}

macro_rules! vec_fmt {
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

macro_rules! vec_map {
	[$i:ident; $v:expr; $V:ident, $N:expr] => ({
		let mut out = $V::new();
		for $i in 0..$N {
			out[$i] = $v;
		}
		out
	})
}

macro_rules! vec_index {
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

macro_rules! vec_new {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Default {
			pub fn new() -> Self {
				$V::<T> { d: [T::default(); $N] }
			}
			
			pub fn map<F>(f: F) -> Self where F: Fn(usize) -> T {
				vec_map![i; f(i); $V, $N]
			}
		}
	)
}

pub trait From_<VU> {
	fn from_(v: VU) -> Self;
}

pub trait Into_<VT> {
	fn into_(self) -> VT;
}

macro_rules! vec_from {
	($V:ident, $N:expr) => (
		impl<'a, T, U> From<&'a [U; $N]> for $V<T> where T: Copy + Default + From<U>, U: Copy {
			fn from(a: &'a [U; $N]) -> Self {
				vec_map![i; T::from(a[i]); $V, $N]
			}
		}

		impl<T, U> From<[U; $N]> for $V<T> where T: Copy + Default + From<U>, U: Copy {
			fn from(a: [U; $N]) -> Self {
				Self::from(&a)
			}
		}

		impl<T, U> From_<$V<U>> for $V<T> where T: Copy + Default + From<U>, U: Copy + Default {
			fn from_(a: $V<U>) -> Self {
				Self::from(a.data())
			}
		}

		impl<T, U> Into_<$V<T>> for $V<U> where T: Copy + Default + From<U>, U: Copy + Default {
			fn into_(self) -> $V<T> {
				self.data().into()
			}
		}
	)
}

macro_rules! vec_neg {
	($V:ident, $N:expr) => (
		impl<T> Neg for $V<T> where T: Copy + Default + Num + Signed {
			type Output = $V<T>;
			fn neg(self) -> Self::Output {
				vec_map![i; -self[i]; $V, $N]
			}
		}
	)
}

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a*$b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a/$b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a%$b }) }

macro_rules! vec_op_vec {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<$V<T>> for $V<T> where T: Copy + Default + Num {
			type Output = $V<T>;
			fn $method(self, vec: $V<T>) -> Self::Output {
				vec_map![i; $op!(self[i], vec[i]); $V, $N]
			}
		}
	)
}

macro_rules! vec_op_scal {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Default + Num {
			type Output = $V<T>;
			fn $method(self, a: T) -> Self::Output {
				vec_map![i; $op!(self[i], a); $V, $N]
			}
		}
	)
}

macro_rules! vec_op_vec_assign {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<$V<T>> for $V<T> where T: Copy + Default + Num {
			fn $method(&mut self, vec: $V<T>) {
				*self = $op!(*self, vec);
			}
		}
	)
}

macro_rules! vec_op_scal_assign {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Default + Num {
			fn $method(&mut self, a: T) {
				*self = $op!(*self, a);
			}
		}
	)
}

macro_rules! vec_ops_all {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		vec_op_vec!($V, $N, $Trait, $method, $op);
		vec_op_scal!($V, $N, $Trait, $method, $op);
	)
}

macro_rules! vec_ops_all_assign {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		vec_op_vec_assign!($V, $N, $Trait, $method, $op);
		vec_op_scal_assign!($V, $N, $Trait, $method, $op);
	)
}

macro_rules! vec_dot {
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

macro_rules! vec_norm {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Default + Num {
			pub fn sqr(self) -> T {
				let mut out = T::zero();
				for i in 0..$N {
					out = out + self[i]*self[i];
				}
				out
			}
		}

		impl<T> $V<T> where T: Copy + Default + Num + Float {
			pub fn length(self) -> T {
				self.sqr().sqrt()
			}

			pub fn normalize(self) -> $V<T> {
				self/self.length()
			}
		}
	)
}

macro_rules! vec_zero {
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

macro_rules! vec_bool_not {
	($V:ident, $N:expr) => (
		impl Not for $V<bool> {
			type Output = $V<bool>;
			fn not(self) -> Self::Output {
				vec_map![i; !self[i]; $V, $N]
			}
		}
	)
}

macro_rules! op_bit_and { ($a:expr, $b:expr) => ({ $a & $b }) }
macro_rules! op_bit_or  { ($a:expr, $b:expr) => ({ $a | $b }) }
macro_rules! op_bit_xor { ($a:expr, $b:expr) => ({ $a ^ $b }) }

macro_rules! vec_bool_op {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl $Trait for $V<bool> {
			type Output = $V<bool>;
			fn $method(self, other: $V<bool>) -> Self::Output {
				vec_map![i; $op!(self[i], other[i]); $V, $N]
			}
		}
	)
}

macro_rules! vec_bool_op_assign {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl $Trait for $V<bool> {
			fn $method(&mut self, other: $V<bool>) {
				*self = $op!(*self, other);
			}
		}
	)
}

macro_rules! vec_bool_any {
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

macro_rules! vec_bool_all {
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

macro_rules! vec_vec_eq {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + PartialEq {
			pub fn eq_(&self, vec: $V<T>) -> $V<bool> {
				vec_map![i; self[i] == vec[i]; $V, $N]
			}
			pub fn ne_(&self, vec: $V<T>) -> $V<bool> {
				vec_map![i; self[i] != vec[i]; $V, $N]
			}
		}
	)
}

macro_rules! vec_vec_cmp {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + PartialOrd {
			pub fn lt_(&self, vec: $V<T>) -> $V<bool> {
				vec_map![i; self[i] < vec[i]; $V, $N]
			}
			pub fn le_(&self, vec: $V<T>) -> $V<bool> {
				vec_map![i; self[i] <= vec[i]; $V, $N]
			}
			pub fn gt_(&self, vec: $V<T>) -> $V<bool> {
				vec_map![i; self[i] > vec[i]; $V, $N]
			}
			pub fn ge_(&self, vec: $V<T>) -> $V<bool> {
				vec_map![i; self[i] >= vec[i]; $V, $N]
			}
		}
	)
}

macro_rules! vec_all {
	($V:ident, $N:expr) => (
		vec_struct!($V, $N);
		vec_data!($V, $N);
		vec_fmt!($V, $N);
		vec_index!($V, $N);
		vec_new!($V, $N);
		vec_from!($V, $N);

		vec_neg!($V, $N);

		vec_op_vec!($V, $N, Add, add, op_add);
		vec_op_vec!($V, $N, Sub, sub, op_sub);
		vec_op_vec_assign!($V, $N, AddAssign, add_assign, op_add);
		vec_op_vec_assign!($V, $N, SubAssign, sub_assign, op_sub);
		vec_ops_all!($V, $N, Mul, mul, op_mul);
		vec_ops_all!($V, $N, Div, div, op_div);
		vec_ops_all!($V, $N, Rem, rem, op_rem);
		vec_ops_all_assign!($V, $N, MulAssign, mul_assign, op_mul);
		vec_ops_all_assign!($V, $N, DivAssign, div_assign, op_div);
		vec_ops_all_assign!($V, $N, RemAssign, rem_assign, op_rem);

		vec_dot!($V, $N);
		vec_norm!($V, $N);

		vec_zero!($V, $N);
		vec_bool_not!($V, $N);

		vec_bool_op!($V, $N, BitAnd, bitand, op_bit_and);
		vec_bool_op!($V, $N, BitOr, bitor, op_bit_or);
		vec_bool_op!($V, $N, BitXor, bitxor, op_bit_xor);

		vec_bool_op_assign!($V, $N, BitAndAssign, bitand_assign, op_bit_and);
		vec_bool_op_assign!($V, $N, BitOrAssign, bitor_assign, op_bit_or);
		vec_bool_op_assign!($V, $N, BitXorAssign, bitxor_assign, op_bit_xor);

		vec_bool_any!($V, $N);
		vec_bool_all!($V, $N);

		vec_vec_eq!($V, $N);
		vec_vec_cmp!($V, $N);
	)
}

vec_all!(vec2, 2);
vec_all!(vec3, 3);
vec_all!(vec4, 4);

impl<T> vec3<T> where T: Copy + Num {
	pub fn cross(self, vec: vec3<T>) -> vec3<T> {
		let a = &self;
		let b = &vec;
		vec3::<T> { d: [ a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0] ] }
	}
}


macro_rules! vec_type {
	($Va:ident, $V:ident, $T:ident) => (
		#[allow(non_camel_case_types)]
		pub type $Va = $V<$T>;
	)
}

vec_type!(vec2b, vec2, bool);
vec_type!(vec3b, vec3, bool);
vec_type!(vec4b, vec4, bool);
vec_type!(vec2i, vec2, i32);
vec_type!(vec3i, vec3, i32);
vec_type!(vec4i, vec4, i32);
vec_type!(vec2u, vec2, u32);
vec_type!(vec3u, vec3, u32);
vec_type!(vec4u, vec4, u32);
vec_type!(vec2f, vec2, f32);
vec_type!(vec3f, vec3, f32);
vec_type!(vec4f, vec4, f32);
vec_type!(vec2d, vec2, f64);
vec_type!(vec3d, vec3, f64);
vec_type!(vec4d, vec4, f64);
