use std::mem;
use std::ops::{
	Index, IndexMut, 
	Neg, Add, Sub, Mul, Div, Rem, 
	AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, 
	Not, BitAnd, BitOr, BitXor,
	BitAndAssign, BitOrAssign, BitXorAssign,
};
use std::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::{Num, Zero, Float, Signed};
use num_integer::{Integer};

macro_rules! vec_struct {
	($V:ident, $N:expr) => (
		#[derive(Clone, Copy, Debug, PartialEq)]
		pub struct $V<T: Copy> {
			pub d: [T; $N],
		}
	)
}

macro_rules! vec_new {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Default {
			pub fn new() -> Self {
				$V { d: [T::default(); $N] }
			}
		}
		
		impl<T> $V<T> where T: Copy {
			pub fn new_array(a: [T; $N]) -> Self {
				$V { d: a }
			}

			pub fn new_map<F>(f: F) -> Self where F: Fn(usize) -> T {
				let mut arr: [T; $N] = unsafe { mem::uninitialized() };
				for i in 0..$N {
					arr[i] = f(i);
				}
				$V { d: arr }
			}

			pub fn new_scal(v: T) -> Self {
				$V { d: [v; $N] }
			}
		}

		impl<T> Default for $V<T> where T: Copy + Default {
			fn default() -> Self {
				$V::new()
			}
		}
	)
}

macro_rules! vec_data {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy {
			pub fn data(&self) -> &[T; $N] {
				&self.d
			}

			pub fn data_mut(&mut self) -> &mut [T; $N] {
				&mut self.d
			}
		}
	)
}

macro_rules! vec_index {
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

macro_rules! vec_map {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy {
			pub fn map<F, S>(self, f: F) -> $V<S> where F: Fn(T) -> S, S: Copy {
				$V::new_map(|i| f(self[i]))
			}
		}
	)
}

macro_rules! vec_fmt {
	($V:ident, $N:expr) => (
		impl<T> Display for $V<T> where T: Copy + Display {
			fn fmt(&self, f: &mut Formatter) -> FmtResult {
				try!(write!(f, "{}[", stringify!($V)));
				for i in 0..$N-1 {
					try!(write!(f, "{}, ", self[i]));
				}
				try!(write!(f, "{}]", self[$N-1]));
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
				$V::new_map(|i| $op!(self[i], vec[i]))
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
				$V::new_map(|i| self[i].div_floor(&other[i]))
			}

			pub fn mod_floor(&self, other: $V<T>) -> $V<T> {
				$V::new_map(|i| self[i].mod_floor(&other[i]))
			}

			pub fn div_mod_floor(&self, other: $V<T>) -> ($V<T>, $V<T>) {
				let dm = $V::new_map(|i| self[i].div_mod_floor(&other[i]));
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
			pub fn sqr(self) -> T {
				let mut out = T::zero();
				for i in 0..$N {
					out = out + self[i]*self[i];
				}
				out
			}
		}

		impl<T> $V<T> where T: Copy + Num + Float {
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
		impl<T> $V<T> where T: Copy + Zero {
			pub fn zero() -> Self {
				$V::new_scal(T::zero())
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
				$V::new_map(|i| !self[i])
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
				$V::new_map(|i| $op!(self[i], other[i]))
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
				$V::new_map(|i| self[i] == vec[i])
			}
			pub fn vne(&self, vec: $V<T>) -> $V<bool> {
				$V::new_map(|i| self[i] != vec[i])
			}
		}
	)
}

macro_rules! vec_vcmp {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + PartialOrd {
			pub fn vlt(&self, vec: $V<T>) -> $V<bool> {
				$V::new_map(|i| self[i] < vec[i])
			}
			pub fn vle(&self, vec: $V<T>) -> $V<bool> {
				$V::new_map(|i| self[i] <= vec[i])
			}
			pub fn vgt(&self, vec: $V<T>) -> $V<bool> {
				$V::new_map(|i| self[i] > vec[i])
			}
			pub fn vge(&self, vec: $V<T>) -> $V<bool> {
				$V::new_map(|i| self[i] >= vec[i])
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
		vec_data!($V, $N);
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

impl<T> Vec3<T> where T: Copy + Num {
	pub fn cross(self, vec: Vec3<T>) -> Vec3<T> {
		let a = &self;
		let b = &vec;
		Vec3::<T> { d: [ a[1]*b[2] - a[2]*b[1], a[2]*b[0] - a[0]*b[2], a[0]*b[1] - a[1]*b[0] ] }
	}
}

macro_rules! vec_type {
	($Va:ident, $V:ident, $T:ident) => (
		pub type $Va = $V<$T>;
	)
}

vec_type!(Vec2bool, Vec2, bool);
vec_type!(Vec3bool, Vec3, bool);
vec_type!(Vec4bool, Vec4, bool);
vec_type!(Vec2i32, Vec2, i32);
vec_type!(Vec3i32, Vec3, i32);
vec_type!(Vec4i32, Vec4, i32);
vec_type!(Vec2u32, Vec2, u32);
vec_type!(Vec3u32, Vec3, u32);
vec_type!(Vec4u32, Vec4, u32);
vec_type!(Vec2f32, Vec2, f32);
vec_type!(Vec3f32, Vec3, f32);
vec_type!(Vec4f32, Vec4, f32);
vec_type!(Vec2f64, Vec2, f64);
vec_type!(Vec3f64, Vec3, f64);
vec_type!(Vec4f64, Vec4, f64);
