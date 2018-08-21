use std::mem;
use std::ops::{Index, IndexMut, Neg, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::iter::{IntoIterator};
use std::slice;
use std::fmt::{Display, Formatter, Result as FmtResult};
use num_traits::{Num, Zero, One, Signed};
use vec::*;


macro_rules! mat_struct {
	($V:ident, $N:expr, $M:expr) => (
		#[derive(Clone, Copy, Debug, PartialEq)]
		pub struct $V<T: Copy> {
			pub d: [T; $N*$M],
		}
	)
}

macro_rules! mat_new {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> $V<T> where T: Copy + Default {
			pub fn new() -> Self {
				$V { d: [T::default(); $N*$M] }
			}
		}

		impl<T> $V<T> where T: Copy {
			pub fn from_arr(a: [T; $N*$M]) -> Self {
				$V { d: a }
			}

			pub fn from_map<F>(f: F) -> Self where F: Fn(usize, usize) -> T {
				let mut arr: [T; $N*$M] = unsafe { mem::uninitialized() };
				for j in 0..$M {
					for i in 0..$N {
						arr[i + j*$N] = f(i, j);
					}
				}
				$V { d: arr }
			}

			pub fn from_scal(v: T) -> Self {
				$V { d: [v; $N*$M] }
			}
		}

		impl<T> Default for $V<T> where T: Copy + Default {
			fn default() -> Self {
				$V::new()
			}
		}
	)
}

macro_rules! mat_data {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> $V<T> where T: Copy {
			pub fn data(&self) -> &[T; $N*$M] {
				&self.d
			}

			pub fn data_mut(&mut self) -> &mut [T; $N*$M] {
				&mut self.d
			}
		}
	)
}

macro_rules! mat_index {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> Index<(usize, usize)> for $V<T> where T: Copy {
			type Output = T;
			fn index(&self, ij: (usize, usize)) -> &Self::Output {
				&self.d[ij.0 + ij.1*$N]
			}
		}

		impl<T> IndexMut<(usize, usize)> for $V<T> where T: Copy {
			fn index_mut(&mut self, ij: (usize, usize)) -> &mut Self::Output {
				&mut self.d[ij.0 + ij.1*$N]
			}
		}
	)
}

macro_rules! mat_iter {
	($V:ident, $N:expr, $M:expr) => (
		impl <'a, T> $V<T> where T: Copy {
			pub fn iter(&'a self) -> slice::Iter<'a, T> {
				self.d.iter()
			}
		}
		impl <'a, T> $V<T> where T: Copy {
			pub fn iter_mut(&'a mut self) -> slice::IterMut<'a, T> {
				self.d.iter_mut()
			}
		}
		impl<'a, T> IntoIterator for &'a $V<T> where T: Copy {
			type Item = &'a T;
			type IntoIter = slice::Iter<'a, T>;
			fn into_iter(self) -> Self::IntoIter {
				self.d.into_iter()
			}
		}
		impl<'a, T> IntoIterator for &'a mut $V<T> where T: Copy {
			type Item = &'a mut T;
			type IntoIter = slice::IterMut<'a, T>;
			fn into_iter(self) -> Self::IntoIter {
				self.d.as_mut().into_iter()
			}
		}
	)
}

macro_rules! mat_fmt {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> Display for $V<T> where T: Copy + Display {
			fn fmt(&self, f: &mut Formatter) -> FmtResult {
				try!(write!(f, "{} [\n", stringify!($V)));
				for j in 0..$M {
					for i in 0..$N {
						try!(write!(f, "\t{}, ", self[(i, j)]));
					}
					try!(write!(f, "\n"));
				}
				try!(write!(f, "]"));
				Ok(())
			}
		}
	)
}

macro_rules! mat_map {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> $V<T> where T: Copy {
			pub fn map<F, S>(self, f: F) -> $V<S> where F: Fn(T) -> S, S: Copy {
				$V::from_map(|i, j| f(self[(i, j)]))
			}
		}
	)
}

macro_rules! mat_neg {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> Neg for $V<T> where T: Copy + Num + Signed {
			type Output = Self;
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

macro_rules! mat_op_mat {
	($V:ident, $N:expr, $M:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait for $V<T> where T: Copy + Num + $Trait<Output=T> {
			type Output = $V<T>;
			fn $method(self, mat: $V<T>) -> Self::Output {
				$V::from_map(|i, j| $op!(self[(i, j)], mat[(i, j)]))
			}
		}
	)
}

macro_rules! mat_op_scal {
	($V:ident, $N:expr, $M:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Num + $Trait<Output=T> {
			type Output = $V<T>;
			fn $method(self, a: T) -> Self::Output {
				$V::from_map(|i, j| $op!(self[(i, j)], a))
			}
		}
	)
}

macro_rules! mat_op_mat_assign {
	($V:ident, $N:expr, $M:expr, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<$V<T>> for $V<T> where T: Copy + Num + $BaseTrait<Output=T> {
			fn $method(&mut self, mat: $V<T>) {
				*self = $op!(*self, mat);
			}
		}
	)
}

macro_rules! mat_op_scal_assign {
	($V:ident, $N:expr, $M:expr, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Num + $BaseTrait<Output=T> {
			fn $method(&mut self, a: T) {
				*self = $op!(*self, a);
			}
		}
	)
}

macro_rules! mat_zero {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> $V<T> where T: Copy + Zero {
			pub fn zero() -> Self {
				$V::<T> { d: [T::zero(); ($N*$M)] }
			}

			pub fn is_zero(&self) -> bool {
				for i in 0..($N*$M) {
					if !self.d[i].is_zero() {
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

macro_rules! mat_all {
	($V:ident, $N:expr, $M:expr) => (
		mat_struct!($V, $N, $M);

		mat_new!($V, $N, $M);
		mat_data!($V, $N, $M);
		mat_iter!($V, $N, $M);
		mat_fmt!($V, $N, $M);
		mat_index!($V, $N, $M);
		mat_map!($V, $N, $M);
		
		mat_neg!($V, $N, $M);
		
		mat_op_mat!($V, $N, $M, Add, add, op_add);
		mat_op_mat!($V, $N, $M, Sub, sub, op_sub);
		mat_op_mat!($V, $N, $M, Mul, mul, op_mul);
		mat_op_mat!($V, $N, $M, Div, div, op_div);
		mat_op_mat!($V, $N, $M, Rem, rem, op_rem);
		mat_op_scal!($V, $N, $M, Mul, mul, op_mul);
		mat_op_scal!($V, $N, $M, Div, div, op_div);
		mat_op_scal!($V, $N, $M, Rem, rem, op_rem);
		
		mat_op_mat_assign!($V, $N, $M, AddAssign, Add, add_assign, op_add);
		mat_op_mat_assign!($V, $N, $M, SubAssign, Sub, sub_assign, op_sub);
		mat_op_mat_assign!($V, $N, $M, MulAssign, Mul, mul_assign, op_mul);
		mat_op_mat_assign!($V, $N, $M, DivAssign, Div, div_assign, op_div);
		mat_op_mat_assign!($V, $N, $M, RemAssign, Rem, rem_assign, op_rem);
		mat_op_scal_assign!($V, $N, $M, MulAssign, Mul, mul_assign, op_mul);
		mat_op_scal_assign!($V, $N, $M, DivAssign, Div, div_assign, op_div);
		mat_op_scal_assign!($V, $N, $M, RemAssign, Rem, rem_assign, op_rem);
		
		mat_zero!($V, $N, $M);
	)
}

mat_all!(Mat2x2, 2, 2);
mat_all!(Mat2x3, 2, 3);
mat_all!(Mat2x4, 2, 4);
mat_all!(Mat3x2, 3, 2);
mat_all!(Mat3x3, 3, 3);
mat_all!(Mat3x4, 3, 4);
mat_all!(Mat4x2, 4, 2);
mat_all!(Mat4x3, 4, 3);
mat_all!(Mat4x4, 4, 4);

macro_rules! mat_transpose {
	($Vnm:ident, $Vmn:ident, $N:expr, $M:expr) => (
		impl<T> $Vnm<T> where T: Copy {
			pub fn transpose(self) -> $Vmn<T> {
				$Vmn::from_map(|i, j| self[(j, i)])
			}
		}
	)
}

mat_transpose!(Mat2x2, Mat2x2, 2, 2);
mat_transpose!(Mat2x3, Mat3x2, 2, 3);
mat_transpose!(Mat2x4, Mat4x2, 2, 4);
mat_transpose!(Mat3x2, Mat2x3, 3, 2);
mat_transpose!(Mat3x3, Mat3x3, 3, 3);
mat_transpose!(Mat3x4, Mat4x3, 3, 4);
mat_transpose!(Mat4x2, Mat2x4, 4, 2);
mat_transpose!(Mat4x3, Mat3x4, 4, 3);
mat_transpose!(Mat4x4, Mat4x4, 4, 4);

pub trait Outer<VT> {
	type Output;
	fn outer(self, other: VT) -> Self::Output;
}

macro_rules! mat_outer {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		impl<T> Outer<$Vn<T>> for $Vm<T> where T: Copy + Num {
			type Output = $Vnm<T>;
			fn outer(self, vec: $Vn<T>) -> Self::Output {
				$Vnm::from_map(|i, j| self[j]*vec[i])
			}
		}
	)
}

macro_rules! mat_row_col {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		impl<T> $Vnm<T> where T: Copy {
			pub fn row(self, j: usize) -> $Vn<T> {
				$Vn::from_map(|i| self[(i, j)])
			}

			pub fn col(self, i: usize) -> $Vm<T> {
				$Vm::from_map(|j| self[(i, j)])
			}
		}
	)
}

macro_rules! mat_mul_vec {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		impl<T> Dot<$Vn<T>> for $Vnm<T> where T: Copy + Num {
			type Output = $Vm<T>;
			fn dot(self, vec: $Vn<T>) -> Self::Output {
				$Vm::from_map(|j| { self.row(j).dot(vec) })
			}
		}
	)
}

macro_rules! mat_mul_vec_mat {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		impl<T> Dot<$Vnm<T>> for $Vm<T> where T: Copy + Num {
			type Output = $Vn<T>;
			fn dot(self, mat: $Vnm<T>) -> Self::Output {
				$Vn::from_map(|i| { self.dot(mat.col(i)) })
			}
		}
	)
}

macro_rules! mat_mul_vec_all {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		mat_outer!($Vnm, $Vn, $Vm, $N, $M);
		mat_row_col!($Vnm, $Vn, $Vm, $N, $M);
		mat_mul_vec!($Vnm, $Vn, $Vm, $N, $M);
		mat_mul_vec_mat!($Vnm, $Vn, $Vm, $N, $M);
	)
}

mat_mul_vec_all!(Mat2x2, Vec2, Vec2, 2, 2);
mat_mul_vec_all!(Mat2x3, Vec2, Vec3, 2, 3);
mat_mul_vec_all!(Mat2x4, Vec2, Vec4, 2, 4);
mat_mul_vec_all!(Mat3x2, Vec3, Vec2, 3, 2);
mat_mul_vec_all!(Mat3x3, Vec3, Vec3, 3, 3);
mat_mul_vec_all!(Mat3x4, Vec3, Vec4, 3, 4);
mat_mul_vec_all!(Mat4x2, Vec4, Vec2, 4, 2);
mat_mul_vec_all!(Mat4x3, Vec4, Vec3, 4, 3);
mat_mul_vec_all!(Mat4x4, Vec4, Vec4, 4, 4);

macro_rules! mat_mul_mat {
	($Vnm:ident, $Vln:ident, $Vlm:ident, $N:expr, $M:expr, $L:expr) => (
		impl<T> Dot<$Vln<T>> for $Vnm<T> where T: Copy + Num {
			type Output = $Vlm<T>;
			fn dot(self, mat: $Vln<T>) -> Self::Output {
				$Vlm::from_map(|i, j| self.row(j).dot(mat.col(i)))
			}
		}
	)
}

mat_mul_mat!(Mat2x2, Mat2x2, Mat2x2, 2, 2, 2);
mat_mul_mat!(Mat2x2, Mat3x2, Mat3x2, 2, 2, 3);
mat_mul_mat!(Mat2x2, Mat4x2, Mat4x2, 2, 2, 4);
mat_mul_mat!(Mat2x3, Mat2x2, Mat2x3, 2, 3, 2);
mat_mul_mat!(Mat2x3, Mat3x2, Mat3x3, 2, 3, 3);
mat_mul_mat!(Mat2x3, Mat4x2, Mat4x3, 2, 3, 4);
mat_mul_mat!(Mat2x4, Mat2x2, Mat2x4, 2, 4, 2);
mat_mul_mat!(Mat2x4, Mat3x2, Mat3x4, 2, 4, 3);
mat_mul_mat!(Mat2x4, Mat4x2, Mat4x4, 2, 4, 4);
mat_mul_mat!(Mat3x2, Mat2x3, Mat2x2, 3, 2, 2);
mat_mul_mat!(Mat3x2, Mat3x3, Mat3x2, 3, 2, 3);
mat_mul_mat!(Mat3x2, Mat4x3, Mat4x2, 3, 2, 4);
mat_mul_mat!(Mat3x3, Mat2x3, Mat2x3, 3, 3, 2);
mat_mul_mat!(Mat3x3, Mat3x3, Mat3x3, 3, 3, 3);
mat_mul_mat!(Mat3x3, Mat4x3, Mat4x3, 3, 3, 4);
mat_mul_mat!(Mat3x4, Mat2x3, Mat2x4, 3, 4, 2);
mat_mul_mat!(Mat3x4, Mat3x3, Mat3x4, 3, 4, 3);
mat_mul_mat!(Mat3x4, Mat4x3, Mat4x4, 3, 4, 4);
mat_mul_mat!(Mat4x2, Mat2x4, Mat2x2, 4, 2, 2);
mat_mul_mat!(Mat4x2, Mat3x4, Mat3x2, 4, 2, 3);
mat_mul_mat!(Mat4x2, Mat4x4, Mat4x2, 4, 2, 4);
mat_mul_mat!(Mat4x3, Mat2x4, Mat2x3, 4, 3, 2);
mat_mul_mat!(Mat4x3, Mat3x4, Mat3x3, 4, 3, 3);
mat_mul_mat!(Mat4x3, Mat4x4, Mat4x3, 4, 3, 4);
mat_mul_mat!(Mat4x4, Mat2x4, Mat2x4, 4, 4, 2);
mat_mul_mat!(Mat4x4, Mat3x4, Mat3x4, 4, 4, 3);
mat_mul_mat!(Mat4x4, Mat4x4, Mat4x4, 4, 4, 4);

macro_rules! mat_one {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + One + Zero {
			pub fn one() -> Self {
				$V::from_map(|i, j| if i == j { T::one() } else { T::zero() })
			}
		}

		impl<T> One for $V<T> where T: Copy + Num + One + Zero {
			fn one() -> Self {
				$V::one()
			}
		}
	)
}

mat_one!(Mat2x2, 2);
mat_one!(Mat3x3, 3);
mat_one!(Mat4x4, 4);

macro_rules! mat_submatrix {
	($Vs:ident, $Vr:ident, $N:expr) => (
		impl<T> $Vs<T> where T: Copy {
			pub fn submatrix(self, x: usize, y: usize) -> $Vr<T> {
				$Vr::from_map(|i, j| self[(i + (i >= x) as usize, j + (j >= y) as usize)])
			}
		}
	)
}

impl<T> Mat2x2<T> where T: Copy {
	pub fn submatrix(self, x:usize, y:usize) -> T {
		self[(1 - x, 1 - y)]
	}
}

mat_submatrix!(Mat4x4, Mat3x3, 4);
mat_submatrix!(Mat3x3, Mat2x2, 3);

macro_rules! mat_cofactor {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Signed {
			pub fn cofactor(self, x: usize, y: usize) -> T {
				(if (x + y) % 2 == 0 { T::one() } else { -T::one() })*self.submatrix(x,y).det()
			}
		}
	)
}

impl<T> Mat2x2<T> where T: Copy + Num + Signed {
	pub fn cofactor(self, x: usize, y: usize) -> T {
		(if (x + y) % 2 == 0 { T::one() } else { -T::one() })*self.submatrix(x,y)
	}
}

mat_cofactor!(Mat4x4, 4);
mat_cofactor!(Mat3x3, 3);

/// Determinant
macro_rules! mat_det {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Signed {
			pub fn det(self) -> T {
				let mut tmp = T::zero();
				let j = 0;
				for i in 0..$N {
					tmp = tmp + self[(i, j)]*self.cofactor(i, j);
				}
				tmp
			}
		}
	)
}

impl<T> Mat2x2<T> where T: Copy + Num + Signed {
	pub fn det(self) -> T {
		self[(0, 0)]*self[(1, 1)] - self[(1, 0)]*self[(0, 1)]
	}
}

mat_det!(Mat4x4, 4);
mat_det!(Mat3x3, 3);

/// Adjugate matrix
macro_rules! mat_adj {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Signed {
			pub fn adj(self) -> $V<T> {
				$V::from_map(|i, j| self.cofactor(j, i))
			}
		}
	)
}

mat_adj!(Mat4x4, 4);
mat_adj!(Mat3x3, 3);
mat_adj!(Mat2x2, 2);

/// Inverse matrix
macro_rules! mat_inverse {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Signed {
			pub fn inverse(self) -> $V<T> {
				self.adj()/self.det()
			}
		}
	)
}

mat_inverse!(Mat4x4, 4);
mat_inverse!(Mat3x3, 3);
mat_inverse!(Mat2x2, 2);

pub type Mat2<T> = Mat2x2<T>;
pub type Mat3<T> = Mat3x3<T>;
pub type Mat4<T> = Mat4x4<T>;

macro_rules! mat_type {
	($Va:ident, $V:ident, $T:ident) => (
		pub type $Va = $V<$T>;
	)
}

mat_type!(Mat2i32, Mat2, i32);
mat_type!(Mat3i32, Mat3, i32);
mat_type!(Mat4i32, Mat4, i32);
mat_type!(Mat2u32, Mat2, u32);
mat_type!(Mat3u32, Mat3, u32);
mat_type!(Mat4u32, Mat4, u32);
mat_type!(Mat2f32, Mat2, f32);
mat_type!(Mat3f32, Mat3, f32);
mat_type!(Mat4f32, Mat4, f32);
mat_type!(Mat2f64, Mat2, f64);
mat_type!(Mat3f64, Mat3, f64);
mat_type!(Mat4f64, Mat4, f64);
