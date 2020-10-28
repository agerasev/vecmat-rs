use std::mem::{MaybeUninit};
use std::ptr;
use std::ops::{
	Index, IndexMut, 
	Neg, Add, Sub, Mul, Div, Rem, 
	AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, 
	Not, BitAnd, BitOr, BitXor,
	BitAndAssign, BitOrAssign, BitXorAssign,
};
use std::iter::{IntoIterator};
use std::slice;
use std::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::{Num, Zero, Float, Signed};
use num_integer::{Integer};


macro_rules! vec_struct {
	($V:ident, $N:expr) => (
		#[derive(Clone, Copy, Debug, PartialEq)]
		pub struct $V<T: Copy> {
			pub data: [T; $N],
		}
	)
}

macro_rules! vec_new {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Default {
			pub fn new() -> Self {
				$V { data: [T::default(); $N] }
			}
		}
		
		impl<T> $V<T> where T: Copy {
			pub fn from_array(a: [T; $N]) -> Self {
				$V { data: a }
			}

			pub fn from_array_ref(a: &[T; $N]) -> Self {
				$V { data: a.clone() }
			}

			pub fn from_slice(s: &[T]) -> Option<Self> {
				match s.len() {
					$N => unsafe {
						let mut a: MaybeUninit<[T; $N]> = MaybeUninit::uninit();
						ptr::copy_nonoverlapping(s.as_ptr(), (*a.as_mut_ptr()).as_mut_ptr(), $N);
						Some($V::from_array(a.assume_init()))
					},
					_ => None,
				}
			}

			pub fn from_map<F>(f: F) -> Self where F: Fn(usize) -> T {
				unsafe {
					let mut a: MaybeUninit<[T; $N]> = MaybeUninit::uninit();
					for i in 0..$N {
						(*a.as_mut_ptr())[i] = f(i);
					}
					$V { data: a.assume_init() }
				}
			}

			pub fn from_scalar(v: T) -> Self {
				$V { data: [v; $N] }
			}
		}

		impl<T> Default for $V<T> where T: Copy + Default {
			fn default() -> Self {
				$V::new()
			}
		}
	)
}

macro_rules! vec_index {
	($V:ident, $N:expr) => (
		impl<T> Index<usize> for $V<T> where T: Copy {
			type Output = T;
			fn index(&self, i: usize) -> &Self::Output {
				assert!(i < $N);
				unsafe { self.data.get_unchecked(i) }
			}
		}

		impl<T> IndexMut<usize> for $V<T> where T: Copy {
			fn index_mut(&mut self, i: usize) -> &mut Self::Output {
				assert!(i < $N);
				unsafe { self.data.get_unchecked_mut(i) }
			}
		}
	)
}

macro_rules! vec_map {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy {
			pub fn map<F, S>(self, f: F) -> $V<S> where F: Fn(T) -> S, S: Copy {
				$V::from_map(|i| f(self[i]))
			}
		}
	)
}

macro_rules! vec_iter {
	($V:ident, $N:expr) => (
		impl <'a, T> $V<T> where T: Copy {
			pub fn iter(&'a self) -> slice::Iter<'a, T> {
				self.data.iter()
			}
		}
		impl <'a, T> $V<T> where T: Copy {
			pub fn iter_mut(&'a mut self) -> slice::IterMut<'a, T> {
				self.data.iter_mut()
			}
		}
		impl<'a, T> IntoIterator for &'a $V<T> where T: Copy {
			type Item = &'a T;
			type IntoIter = slice::Iter<'a, T>;
			fn into_iter(self) -> Self::IntoIter {
				self.data.iter()
			}
		}
		impl<'a, T> IntoIterator for &'a mut $V<T> where T: Copy {
			type Item = &'a mut T;
			type IntoIter = slice::IterMut<'a, T>;
			fn into_iter(self) -> Self::IntoIter {
				self.data.as_mut().into_iter()
			}
		}
	)
}

macro_rules! vec_fmt {
	($V:ident, $N:expr) => (
		impl<T> Display for $V<T> where T: Copy + Display {
			fn fmt(&self, f: &mut Formatter) -> FmtResult {
				write!(f, "{}(", stringify!($V))?;
				for i in 0..$N-1 {
					write!(f, "{}, ", self[i])?;
				}
				write!(f, "{})", self[$N-1])?;
				Ok(())
			}
		}
	)
}

macro_rules! vec_neg {
	($V:ident, $N:expr) => (
		impl<T> Neg for $V<T> where T: Copy + Num + Signed {
			type Output = $V<T>;
			fn neg(self) -> Self::Output {
				self.map(|v| -v)
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
		impl<T> $Trait for $V<T> where T: Copy + Num + $Trait<Output=T> {
			type Output = $V<T>;
			fn $method(self, vec: $V<T>) -> Self::Output {
				$V::from_map(|i| $op!(self[i], vec[i]))
			}
		}
	)
}

macro_rules! vec_op_scal {
	($V:ident, $N:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Num + $Trait<Output=T> {
			type Output = $V<T>;
			fn $method(self, a: T) -> Self::Output {
				self.map(|v| $op!(v, a))
			}
		}
	)
}

macro_rules! vec_op_vec_assign {
	($V:ident, $N:expr, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait for $V<T> where T: Copy + Num + $BaseTrait<Output=T> {
			fn $method(&mut self, vec: $V<T>) {
				*self = $op!(*self, vec);
			}
		}
	)
}

macro_rules! vec_op_scal_assign {
	($V:ident, $N:expr, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Num + $BaseTrait<Output=T> {
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
	($V:ident, $N:expr, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
		vec_op_vec_assign!($V, $N, $Trait, $BaseTrait, $method, $op);
		vec_op_scal_assign!($V, $N, $Trait, $BaseTrait, $method, $op);
	)
}

macro_rules! vec_div_mod_floor {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Integer {
			pub fn div_floor(&self, other: $V<T>) -> $V<T> {
				$V::from_map(|i| self[i].div_floor(&other[i]))
			}

			pub fn mod_floor(&self, other: $V<T>) -> $V<T> {
				$V::from_map(|i| self[i].mod_floor(&other[i]))
			}

			pub fn div_mod_floor(&self, other: $V<T>) -> ($V<T>, $V<T>) {
				let dm = $V::from_map(|i| self[i].div_mod_floor(&other[i]));
				(dm.map(|v| v.0), dm.map(|v| v.1))
			}
		}
	)
}

pub trait Dot<VT> {
	type Output;
	fn dot(self, other: VT) -> Self::Output;
}

macro_rules! vec_dot {
	($V:ident, $N:expr) => (
		impl<T> Dot<$V<T>> for $V<T> where T: Copy + Num {
			type Output = T;
			fn dot(self, vec: $V<T>) -> Self::Output {
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
		impl<T> $V<T> where T: Copy + Num {
			pub fn sqrlen(self) -> T {
				let mut out = T::zero();
				for i in 0..$N {
					out = out + self[i]*self[i];
				}
				out
			}
		}

		impl<T> $V<T> where T: Copy + Num + Float {
			pub fn length(self) -> T {
				self.sqrlen().sqrt()
			}

			pub fn normalize(self) -> $V<T> {
				self/self.length()
			}
		}
	)
}

macro_rules! vec_zero {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Zero {
			pub fn zero() -> Self {
				$V::from_scalar(T::zero())
			}

			pub fn is_zero(&self) -> bool {
				for i in 0..$N {
					if !self[i].is_zero() {
						return false;
					}
				}
				true
			}
		}

		impl<T> Zero for $V<T> where T: Copy + Num + Zero {
			fn zero() -> Self {
				$V::zero()
			}

			fn is_zero(&self) -> bool {
				$V::is_zero(self)
			}
		}
	)
}

macro_rules! vec_bool_not {
	($V:ident, $N:expr) => (
		impl Not for $V<bool> {
			type Output = $V<bool>;
			fn not(self) -> Self::Output {
				$V::from_map(|i| !self[i])
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
				$V::from_map(|i| $op!(self[i], other[i]))
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

macro_rules! vec_veq {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + PartialEq {
			pub fn veq(&self, vec: $V<T>) -> $V<bool> {
				$V::from_map(|i| self[i] == vec[i])
			}
			pub fn vne(&self, vec: $V<T>) -> $V<bool> {
				$V::from_map(|i| self[i] != vec[i])
			}
		}
	)
}

macro_rules! vec_vcmp {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + PartialOrd {
			pub fn vlt(&self, vec: $V<T>) -> $V<bool> {
				$V::from_map(|i| self[i] < vec[i])
			}
			pub fn vle(&self, vec: $V<T>) -> $V<bool> {
				$V::from_map(|i| self[i] <= vec[i])
			}
			pub fn vgt(&self, vec: $V<T>) -> $V<bool> {
				$V::from_map(|i| self[i] > vec[i])
			}
			pub fn vge(&self, vec: $V<T>) -> $V<bool> {
				$V::from_map(|i| self[i] >= vec[i])
			}
		}

		impl<T> $V<T> where T: Copy + PartialOrd {
			pub fn min(&self) -> T {
				let mut mv = self[0];
				for i in 1..$N {
					let v = self[i];
					if v < mv {
						mv = v;
					}
				}
				mv
			}
			
			pub fn max(&self) -> T {
				let mut mv = self[0];
				for i in 1..$N {
					let v = self[i];
					if v > mv {
						mv = v;
					}
				}
				mv
			}
		}
	)
}

macro_rules! vec_all {
	($V:ident, $N:expr) => (
		vec_struct!($V, $N);
		vec_index!($V, $N);
		vec_new!($V, $N);
		vec_iter!($V, $N);
		vec_fmt!($V, $N);
		vec_map!($V, $N);

		vec_neg!($V, $N);

		vec_op_vec!($V, $N, Add, add, op_add);
		vec_op_vec!($V, $N, Sub, sub, op_sub);
		vec_op_vec_assign!($V, $N, AddAssign, Add, add_assign, op_add);
		vec_op_vec_assign!($V, $N, SubAssign, Sub, sub_assign, op_sub);
		vec_ops_all!($V, $N, Mul, mul, op_mul);
		vec_ops_all!($V, $N, Div, div, op_div);
		vec_ops_all!($V, $N, Rem, rem, op_rem);
		vec_ops_all_assign!($V, $N, MulAssign, Mul, mul_assign, op_mul);
		vec_ops_all_assign!($V, $N, DivAssign, Div, div_assign, op_div);
		vec_ops_all_assign!($V, $N, RemAssign, Rem, rem_assign, op_rem);
		
		vec_div_mod_floor!($V, $N);
		
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

		vec_veq!($V, $N);
		vec_vcmp!($V, $N);
	)
}

vec_all!(Vec2, 2);
vec_all!(Vec3, 3);
vec_all!(Vec4, 4);


// from args

impl<T> Vec2<T> where T: Copy {
	pub fn from(v0: T, v1: T) -> Self {
		Self { data: [v0, v1] }
	}
}

impl<T> Vec3<T> where T: Copy {
	pub fn from(v0: T, v1: T, v2: T) -> Self {
		Self { data: [v0, v1, v2] }
	}
}

impl<T> Vec4<T> where T: Copy {
	pub fn from(v0: T, v1: T, v2: T, v3: T) -> Self {
		Self { data: [v0, v1, v2, v3] }
	}
}


// cross product

impl<T> Vec2<T> where T: Copy + Num {
	pub fn cross(self, vec: Vec2<T>) -> T {
		self[0]*vec[1] - self[1]*vec[0]
	}
}

impl<T> Vec3<T> where T: Copy + Num {
	pub fn cross(self, vec: Vec3<T>) -> Vec3<T> {
		let a = &self;
		let b = &vec;
		Vec3::<T> { data: [ a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0] ] }
	}
}

macro_rules! vec_mul_scal_rev {
	($V:ident, $T:ident) => (
		impl Mul<$V<$T>> for $T {
			type Output = $V<$T>;
			fn mul(self, a: $V<$T>) -> Self::Output {
				a*self
			}
		}
	)
}

// T * VecN<T> workaround
cartesian!(
	vec_mul_scal_rev,
	[Vec2, Vec3, Vec4],
	[i8, u8, i16, u16, i32, u32, i64, u64, f32, f64]
);

#[allow(unused_macros)]
macro_rules! vec_type {
	($Va:ident, $V:ident, $T:ident) => (
		pub type $Va = $V<$T>;
	)
}

#[cfg(test)]
#[test]
fn dummy_test() {
    Vec3::<f64>::new();
}
