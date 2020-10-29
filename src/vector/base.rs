use core::{
	convert::{TryFrom, TryInto},
	mem::{MaybeUninit},
	ops::{
		Deref,
		Index, IndexMut,
		Neg, Add, Sub, Mul, Div, Rem,
		AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
		Not, BitAnd, BitOr, BitXor,
		BitAndAssign, BitOrAssign, BitXorAssign,
	},
	cmp::{PartialOrd},
	iter::{IntoIterator},
	array::{TryFromSliceError},
	slice,
	fmt::{Display, Formatter, Result as FmtResult},
};
use num_traits::{Num, Zero, One, Float, Signed};
use num_integer::{self as nint, Integer};
use crate::{traits::*, array::*};


macro_rules! vector_base { ($N:expr, $V:ident) => (
	/// Vector of fixed-size.
	#[repr(transparent)]
	#[derive(Clone, Copy, Debug, PartialEq)]
	pub struct $V<T> {
		data: [T; $N],
	}

	impl<T> $V<T> {
		/// Initialize vector by closure.
		pub fn init<F: FnMut() -> T>(f: F) -> Self {
			Self { data: <[T; $N]>::init_ext(f) }
		}
	}
	impl<T> Default for $V<T> where T: Default {
		/// Create vector filled with default values.
		fn default() -> Self {
			Self::init(|| T::default())
		}
	}
	impl<T> Zero for $V<T> where T: Zero {
		fn zero() -> Self {
			Self::init(|| T::zero())
		}
		fn is_zero(&self) -> bool {
			self.iter().all(|x| x.is_zero())
		}
	}
	impl<T> $V<T> where T: Default {
		/// Create default vector.
		pub fn new() -> Self {
			Self::default()
		}
	}

	impl<T> From<[T; $N]> for $V<T> {
		fn from(a: [T; $N]) -> Self {
			Self { data: a }
		}
	}
	impl<T> From<&[T; $N]> for $V<T> where T: Clone {
		fn from(ar: &[T; $N]) -> Self {
			Self { data: ar.clone() }
		}
	}

	impl<T> Into<[T; $N]> for $V<T> {
		fn into(self) -> [T; $N] {
			self.data
		}
	}
	impl<'a, T> Into<&'a [T; $N]> for &'a $V<T> {
		fn into(self) -> &'a [T; $N] {
			&self.data
		}
	}
	impl<'a, T> Into<&'a mut [T; $N]> for &'a mut $V<T> {
		fn into(self) -> &'a mut [T; $N] {
			&mut self.data
		}
	}

	impl<T> AsRef<[T; $N]> for $V<T> {
		fn as_ref(&self) -> &[T; $N] {
			&self.data
		}
	}
	impl<T> AsMut<[T; $N]> for $V<T> {
		fn as_mut(&mut self) -> &mut [T; $N] {
			&mut self.data
		}
	}

	impl<'a, T> TryFrom<&'a [T]> for $V<T> where T: Copy {
		type Error = TryFromSliceError;
		fn try_from(s: &'a [T]) -> Result<Self, Self::Error> {
			s.try_into().map(|a| Self { data: a })
		}
	}

	impl<T> Index<usize> for $V<T> {
		type Output = T;
		fn index(&self, i: usize) -> &T {
			&self.data[i]
		}
	}
	impl<T> IndexMut<usize> for $V<T> {
		fn index_mut(&mut self, i: usize) -> &mut T {
			&mut self.data[i]
		}
	}

	impl<T> Display for $V<T> where T: Display {
		fn fmt(&self, f: &mut Formatter) -> FmtResult {
			write!(f, "{}(", stringify!($V))?;
			for i in 0..$N-1 {
				write!(f, "{}, ", self[i])?;
			}
			write!(f, "{})", self[$N-1])?;
			Ok(())
		}
	}
) }

macro_rules! vector_iter { ($N:expr, $V:ident, $A:ident) => (
	impl <T> $V<T> {
		/// Returns iterator over vector element refrences.
		pub fn iter(&self) -> impl Iterator<Item=&T> {
			self.data.iter()
		}
		/// Returns iterator over vector element mutable refrences.
		pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
			self.data.iter_mut()
		}
	}

	impl<T> IntoIterator for $V<T> {
		type Item = T;
		type IntoIter = <[T; $N] as $A<T>>::IntoIter_;
		fn into_iter(self) -> Self::IntoIter {
			self.data.into_iter_ext()
		}
	}
	impl<'a, T> IntoIterator for &'a $V<T> {
		type Item = &'a T;
		type IntoIter = slice::Iter<'a, T>;
		fn into_iter(self) -> Self::IntoIter {
			self.data.iter()
		}
	}
	impl<'a, T> IntoIterator for &'a mut $V<T> {
		type Item = &'a mut T;
		type IntoIter = slice::IterMut<'a, T>;
		fn into_iter(self) -> Self::IntoIter {
			self.data.iter_mut()
		}
	}

	impl<T> $V<T> where T: Clone + Zero + One + AddAssign {
		/// Create vector which element value equals to its position in vector.
		pub fn range() -> Self {
			let mut i = T::zero();
			Self::init(|| {
				let j = i.clone();
				i += T::one();
				j
			})
		}
	}

	impl<T> $V<T> {
		/// Call closure for each element of the vector passing it by value.
		pub fn for_each<F: FnMut(T)>(self, f: F) {
			self.data.for_each_ext(f)
		}
		/// Map vector elements.
		pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> $V<U> {
			self.data.map_ext(f).into()
		}
		/// Zip two vectors into one.
		pub fn zip<U>(self, other: $V<U>) -> $V<(T, U)> {
			self.data.zip_ext(other.data).into()
		}
	}
	impl<T, U> $V<(T, U)> {
		/// Unzip vector of tuples into two vectors.
		pub fn unzip(self) -> ($V<T>, $V<U>) {
			let z = self.data.unzip_ext();
			(z.0.into(), z.1.into())
		}
	}
	impl<T> $V<T> {
		fn fold<S, F: FnMut(S, T) -> S>(self, s: S, f: F) -> S {
			self.data.fold_ext(s, f)
		}
        fn fold_first<F: FnMut(T, T) -> T>(self, f: F) -> T {
			self.data.fold_first_ext(f)
		}
        fn scan<S, U, F: FnMut(&mut S, T) -> U>(self, s: S, f: F) -> [U; $N] {
			self.data.scan_ext(s, f).into()
		}
		fn sum(self) -> T where T: Add<Output=T> {
			self.data.fold_first_ext(|x, y| x + y)
		}
	}
) }

macro_rules! vector_neg { ($N:expr, $V:ident) => (
	impl<T> Neg for $V<T> where T: Neg<Output=T> {
		type Output = $V<T>;
		fn neg(self) -> Self::Output {
			self.map(|v| -v)
		}
	}
) }

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a * $b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a / $b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a % $b }) }

macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! vector_op_vec {
	($N:expr, $V:ident, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait for $V<T> where T: $Trait<Output=T> {
			type Output = $V<T>;
			fn $method(self, vec: $V<T>) -> Self::Output {
				self.zip(vec).map(|(x, y)| $op!(x, y))
			}
		}
	)
}
macro_rules! vector_op_scal {
	($N:expr, $V:ident, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: $Trait<Output=T> + Clone {
			type Output = $V<T>;
			fn $method(self, a: T) -> Self::Output {
				self.map(|v| $op!(v, a.clone()))
			}
		}
	)
}
macro_rules! vector_op_vec_assign {
	($N:expr, $V:ident, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait for $V<T> where T: $Trait {
			fn $method(&mut self, vec: $V<T>) {
				self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| { $op!(*s, x); })
			}
		}
	)
}
macro_rules! vector_op_scal_assign {
	($N:expr, $V:ident, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: $Trait + Clone {
			fn $method(&mut self, a: T) {
				self.iter_mut().for_each(|s| { $op!(*s, a.clone()); })
			}
		}
	)
}

macro_rules! vector_ops_all {
	($N:expr, $V:ident, $Trait:ident, $method:ident, $op:ident) => (
		vector_op_vec!($N, $V, $Trait, $method, $op);
		vector_op_scal!($N, $V, $Trait, $method, $op);
	)
}

macro_rules! vector_ops_all_assign {
	($N:expr, $V:ident, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
		vector_op_vec_assign!($N, $V, $Trait, $BaseTrait, $method, $op);
		vector_op_scal_assign!($N, $V, $Trait, $BaseTrait, $method, $op);
	)
}

macro_rules! vector_ops_base { ($N:expr, $V:ident) => (
	vector_neg!($N, $V);

	vector_op_vec!($N, $V, Add, add, op_add);
	vector_op_vec!($N, $V, Sub, sub, op_sub);
	vector_ops_all!($N, $V, Mul, mul, op_mul);
	vector_ops_all!($N, $V, Div, div, op_div);
	vector_ops_all!($N, $V, Rem, rem, op_rem);

	vector_op_vec_assign!($N, $V, AddAssign, Add, add_assign, op_add_assign);
	vector_op_vec_assign!($N, $V, SubAssign, Sub, sub_assign, op_sub_assign);
	vector_ops_all_assign!($N, $V, MulAssign, Mul, mul_assign, op_mul_assign);
	vector_ops_all_assign!($N, $V, DivAssign, Div, div_assign, op_div_assign);
	vector_ops_all_assign!($N, $V, RemAssign, Rem, rem_assign, op_rem_assign);
) }

pub trait Abs {
	fn abs(self) -> Self;
}
impl<T> Abs for T where T: Zero + PartialOrd + Neg<Output=Self> {
	fn abs(self) -> Self {
		if self < Self::zero() { -self } else { self }
	}
}

macro_rules! vector_metrics { ($V:ident, $N:expr) => (
	impl<T> Metric<metrics::L0> for $V<T> where T: PartialEq {
		type Output = bool;
		fn distance(&self, other: &Self) -> bool {
			self != other
		}
	}
	impl<T> Metric<metrics::L1> for $V<T> where T: Sub<Output=T> + Abs + Add<Output=T> + Clone {
		type Output = T;
		fn distance(&self, other: &Self) -> T {
			(self.clone() - other.clone()).map(|x| x.abs()).sum()
		}
	}
	impl<T> Metric<metrics::L2> for $V<T> where T: Float + Mul<Output=T> + Clone {
		type Output = T;
		fn distance(&self, other: &Self) -> T {
			(self.clone() * other.clone()).map(|x| x.clone()*x).sum().sqrt()
		}
	}
	/*
	impl<T> DotProduct<$V<T>> for $V<T> where T: Copy + Num {
		type Output = T;
		fn dot(self, vec: $V<T>) -> Self::Output {
			let mut out = T::zero();
			for i in 0..$N {
				out = out + self[i]*vec[i];
			}
			out
		}
	}
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
	*/
) }

/*
macro_rules! vector_div_mod_floor {
	($N:expr, $V:ident) => (
		impl<T> $V<T> where T: Integer {
			pub fn div_floor(self, other: $V<T>) -> $V<T> {
				self.zip(other).map(|(x, y)| nint::div_floor(x, y))
			}
			pub fn mod_floor(self, other: $V<T>) -> $V<T> {
				self.zip(other).map(|(x, y)| nint::mod_floor(x, y))
			}
			pub fn div_mod_floor(self, other: $V<T>) -> ($V<T>, $V<T>) {
				self.zip(other).map(|(x, y)| nint::div_mod_floor(x, y)).unzip()
			}
		}
	)
}

//vector_div_rem!($N, $V);
//vector_div_mod_floor!($N, $V);

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
*/

macro_rules! vector_all { ($N:expr, $V:ident, $A:ident) => (
		vector_base!($N, $V);
		vector_iter!($N, $V, $A);

		vector_ops_base!($N, $V);

		vector_metrics!($V, $N);

		//vec_bool_not!($V, $N);

		//vec_bool_op!($V, $N, BitAnd, bitand, op_bit_and);
		//vec_bool_op!($V, $N, BitOr, bitor, op_bit_or);
		//vec_bool_op!($V, $N, BitXor, bitxor, op_bit_xor);

		//vec_bool_op_assign!($V, $N, BitAndAssign, bitand_assign, op_bit_and);
		//vec_bool_op_assign!($V, $N, BitOrAssign, bitor_assign, op_bit_or);
		//vec_bool_op_assign!($V, $N, BitXorAssign, bitxor_assign, op_bit_xor);

		//vec_bool_any!($V, $N);
		//vec_bool_all!($V, $N);

		//vec_veq!($V, $N);
		//vec_vcmp!($V, $N);
) }

vector_all!(2, Vector2, Array2Ext);
vector_all!(3, Vector3, Array3Ext);
vector_all!(4, Vector4, Array4Ext);

/*
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
*/

#[cfg(test)]
#[test]
fn dummy_test() {
    Vector3::<f64>::new();
}
