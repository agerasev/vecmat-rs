use std::ops::{Index, IndexMut, Neg, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use num::{Num, Zero, One, Signed};
use vec::*;

macro_rules! vnm_struct {
	($V:ident, $N:expr, $M:expr) => (
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy, PartialEq)]
		pub struct $V<T: Copy> {
			pub d: [T; $N*$M],
		}
	)
}

macro_rules! vnm_data {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> $V<T> where T: Copy {
			pub fn data(&self) -> &[T; $N*$M] {
				&self.d
			}
		}
	)
}

macro_rules! vnm_fmt {
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

		impl<T> Debug for $V<T> where T: Copy + Display {
			#[inline]
			fn fmt(&self, f: &mut Formatter) -> FmtResult {
				Display::fmt(self, f)
			}
		}
	)
}

macro_rules! vnm_map {
	[$i:ident, $j:ident; $v:expr; $V:ident, $N:expr, $M:expr] => ({
		let mut out = $V::new();
		for $j in 0..$M {
			for $i in 0..$N {
				out[($i, $j)] = $v;
			}
		}
		out
	})
}

macro_rules! vnm_index {
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

macro_rules! vnm_new {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> $V<T> where T: Copy + Default {
			pub fn new() -> Self {
				$V::<T> { d: [T::default(); $N*$M] }
			}
		}
	)
}

macro_rules! vnm_from {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> From<[T; ($N*$M)]> for $V<T> where T: Copy {
			fn from(a: [T; ($N*$M)]) -> Self {
				$V::<T> { d: a }
			}
		}
	)
}

macro_rules! vnm_neg {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> Neg for $V<T> where T: Copy + Default + Num + Signed {
			type Output = $V<T>;
			fn neg(self) -> Self::Output {
				vnm_map![i, j; -self[(i, j)]; $V, $N, $M]
			}
		}
	)
}

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a*$b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a/$b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a%$b }) }

macro_rules! vnm_op_mat {
	($V:ident, $N:expr, $M:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<$V<T>> for $V<T> where T: Copy + Default + Num {
			type Output = $V<T>;
			fn $method(self, mat: $V<T>) -> Self::Output {
				vnm_map![i, j; $op!(self[(i, j)], mat[(i, j)]); $V, $N, $M]
			}
		}
	)
}

macro_rules! vnm_op_scal {
	($V:ident, $N:expr, $M:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Default + Num {
			type Output = $V<T>;
			fn $method(self, a: T) -> Self::Output {
				vnm_map![i, j; $op!(self[(i, j)], a); $V, $N, $M]
			}
		}
	)
}

macro_rules! vnm_op_mat_assign {
	($V:ident, $N:expr, $M:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<$V<T>> for $V<T> where T: Copy + Default + Num {
			fn $method(&mut self, mat: $V<T>) {
				*self = $op!(*self, mat);
			}
		}
	)
}

macro_rules! vnm_op_scal_assign {
	($V:ident, $N:expr, $M:expr, $Trait:ident, $method:ident, $op:ident) => (
		impl<T> $Trait<T> for $V<T> where T: Copy + Default + Num {
			fn $method(&mut self, a: T) {
				*self = $op!(*self, a);
			}
		}
	)
}

macro_rules! vnm_zero {
	($V:ident, $N:expr, $M:expr) => (
		impl<T> Zero for $V<T> where T: Copy + Default + Num {
			fn zero() -> Self {
				$V::<T> { d: [T::zero(); ($N*$M)] }
			}

			fn is_zero(&self) -> bool {
				for i in 0..($N*$M) {
					if !self.d[i].is_zero() {
						return false;
					}
				}
				true
			}
		}
	)
}

macro_rules! vnm_all {
	($V:ident, $N:expr, $M:expr) => (
		vnm_struct!($V, $N, $M);
		vnm_data!($V, $N, $M);
		vnm_fmt!($V, $N, $M);
		vnm_index!($V, $N, $M);
		vnm_new!($V, $N, $M);
		vnm_from!($V, $N, $M);

		vnm_neg!($V, $N, $M);

		vnm_op_mat!($V, $N, $M, Add, add, op_add);
		vnm_op_mat!($V, $N, $M, Sub, sub, op_sub);
		vnm_op_scal!($V, $N, $M, Mul, mul, op_mul);
		vnm_op_scal!($V, $N, $M, Div, div, op_div);
		vnm_op_scal!($V, $N, $M, Rem, rem, op_rem);
		vnm_op_mat_assign!($V, $N, $M, AddAssign, add_assign, op_add);
		vnm_op_mat_assign!($V, $N, $M, SubAssign, sub_assign, op_sub);
		vnm_op_scal_assign!($V, $N, $M, MulAssign, mul_assign, op_mul);
		vnm_op_scal_assign!($V, $N, $M, DivAssign, div_assign, op_div);
		vnm_op_scal_assign!($V, $N, $M, RemAssign, rem_assign, op_rem);

		vnm_zero!($V, $N, $M);
	)
}

vnm_all!(mat2x2, 2, 2);
vnm_all!(mat2x3, 2, 3);
vnm_all!(mat2x4, 2, 4);
vnm_all!(mat3x2, 3, 2);
vnm_all!(mat3x3, 3, 3);
vnm_all!(mat3x4, 3, 4);
vnm_all!(mat4x2, 4, 2);
vnm_all!(mat4x3, 4, 3);
vnm_all!(mat4x4, 4, 4);

macro_rules! vnm_transpose {
	($Vnm:ident, $Vmn:ident, $N:expr, $M:expr) => (
		impl<T> $Vnm<T> where T: Copy + Default {
			pub fn transpose(self) -> $Vmn<T> {
				vnm_map![i, j; self[(j, i)]; $Vmn, $M, $N]
			}
		}
	)
}

vnm_transpose!(mat2x2, mat2x2, 2, 2);
vnm_transpose!(mat2x3, mat3x2, 2, 3);
vnm_transpose!(mat2x4, mat4x2, 2, 4);
vnm_transpose!(mat3x2, mat2x3, 3, 2);
vnm_transpose!(mat3x3, mat3x3, 3, 3);
vnm_transpose!(mat3x4, mat4x3, 3, 4);
vnm_transpose!(mat4x2, mat2x4, 4, 2);
vnm_transpose!(mat4x3, mat3x4, 4, 3);
vnm_transpose!(mat4x4, mat4x4, 4, 4);

pub trait Outer<VT> {
	type Output;
	fn outer(self, other: VT) -> Self::Output;
}

macro_rules! vnm_outer {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		impl<T> Outer<$Vn<T>> for $Vm<T> where T: Copy + Default + Num {
			type Output = $Vnm<T>;
			fn outer(self, vec: $Vn<T>) -> Self::Output {
				let mut out = $Vnm::<T>::new();
				for j in 0..$M {
					for i in 0..$N {
						out[(i, j)] = self[j]*vec[i];
					}
				}
				out
			}
		}
	)
}

macro_rules! vnm_mul_vec {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		impl<T> Mul<$Vn<T>> for $Vnm<T> where T: Copy + Default + Num {
			type Output = $Vm<T>;
			fn mul(self, vec: $Vn<T>) -> Self::Output {
				let mut out = $Vm::<T>::zero();
				for j in 0..$M {
					let mut tmp = T::zero();
					for i in 0..$N {
						tmp = tmp + self[(i, j)]*vec[i];
					}
					out[j] = tmp;
				}
				out
			}
		}
	)
}

macro_rules! vnm_mul_vec_mat {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		impl<T> Mul<$Vnm<T>> for $Vm<T> where T: Copy + Default + Num {
			type Output = $Vn<T>;
			fn mul(self, mat: $Vnm<T>) -> Self::Output {
				let mut out = $Vn::<T>::zero();
				for i in 0..$N {
					let mut tmp = T::zero();
					for j in 0..$M {
						tmp = tmp + mat[(i, j)]*self[j];
					}
					out[i] = tmp;
				}
				out
			}
		}
	)
}

macro_rules! vnm_mul_vec_all {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		vnm_outer!($Vnm, $Vn, $Vm, $N, $M);
		vnm_mul_vec!($Vnm, $Vn, $Vm, $N, $M);
		vnm_mul_vec_mat!($Vnm, $Vn, $Vm, $N, $M);
	)
}

vnm_mul_vec_all!(mat2x2, vec2, vec2, 2, 2);
vnm_mul_vec_all!(mat2x3, vec2, vec3, 2, 3);
vnm_mul_vec_all!(mat2x4, vec2, vec4, 2, 4);
vnm_mul_vec_all!(mat3x2, vec3, vec2, 3, 2);
vnm_mul_vec_all!(mat3x3, vec3, vec3, 3, 3);
vnm_mul_vec_all!(mat3x4, vec3, vec4, 3, 4);
vnm_mul_vec_all!(mat4x2, vec4, vec2, 4, 2);
vnm_mul_vec_all!(mat4x3, vec4, vec3, 4, 3);
vnm_mul_vec_all!(mat4x4, vec4, vec4, 4, 4);

macro_rules! vnm_mul_mat {
	($Vnm:ident, $Vln:ident, $Vlm:ident, $N:expr, $M:expr, $L:expr) => (
		impl<T> Mul<$Vln<T>> for $Vnm<T> where T: Copy + Default + Num {
			type Output = $Vlm<T>;
			fn mul(self, mat: $Vln<T>) -> Self::Output {
				let mut out = $Vlm::<T>::zero();
				for j in 0..$M {
					for i in 0..$L {
						let mut tmp = T::zero();
						for k in 0..$N {
							tmp = tmp + self[(k, j)]*mat[(i, k)];
						}
						out[(i, j)] = tmp;
					}
				}
				out
			}
		}
	)
}

vnm_mul_mat!(mat2x2, mat2x2, mat2x2, 2, 2, 2);
vnm_mul_mat!(mat2x2, mat3x2, mat3x2, 2, 2, 3);
vnm_mul_mat!(mat2x2, mat4x2, mat4x2, 2, 2, 4);
vnm_mul_mat!(mat2x3, mat2x2, mat2x3, 2, 3, 2);
vnm_mul_mat!(mat2x3, mat3x2, mat3x3, 2, 3, 3);
vnm_mul_mat!(mat2x3, mat4x2, mat4x3, 2, 3, 4);
vnm_mul_mat!(mat2x4, mat2x2, mat2x4, 2, 4, 2);
vnm_mul_mat!(mat2x4, mat3x2, mat3x4, 2, 4, 3);
vnm_mul_mat!(mat2x4, mat4x2, mat4x4, 2, 4, 4);
vnm_mul_mat!(mat3x2, mat2x3, mat2x2, 3, 2, 2);
vnm_mul_mat!(mat3x2, mat3x3, mat3x2, 3, 2, 3);
vnm_mul_mat!(mat3x2, mat4x3, mat4x2, 3, 2, 4);
vnm_mul_mat!(mat3x3, mat2x3, mat2x3, 3, 3, 2);
vnm_mul_mat!(mat3x3, mat3x3, mat3x3, 3, 3, 3);
vnm_mul_mat!(mat3x3, mat4x3, mat4x3, 3, 3, 4);
vnm_mul_mat!(mat3x4, mat2x3, mat2x4, 3, 4, 2);
vnm_mul_mat!(mat3x4, mat3x3, mat3x4, 3, 4, 3);
vnm_mul_mat!(mat3x4, mat4x3, mat4x4, 3, 4, 4);
vnm_mul_mat!(mat4x2, mat2x4, mat2x2, 4, 2, 2);
vnm_mul_mat!(mat4x2, mat3x4, mat3x2, 4, 2, 3);
vnm_mul_mat!(mat4x2, mat4x4, mat4x2, 4, 2, 4);
vnm_mul_mat!(mat4x3, mat2x4, mat2x3, 4, 3, 2);
vnm_mul_mat!(mat4x3, mat3x4, mat3x3, 4, 3, 3);
vnm_mul_mat!(mat4x3, mat4x4, mat4x3, 4, 3, 4);
vnm_mul_mat!(mat4x4, mat2x4, mat2x4, 4, 4, 2);
vnm_mul_mat!(mat4x4, mat3x4, mat3x4, 4, 4, 3);
vnm_mul_mat!(mat4x4, mat4x4, mat4x4, 4, 4, 4);

macro_rules! vnm_one {
	($V:ident, $N:expr) => (
		impl<T> One for $V<T> where T: Copy + Default + Num {
			fn one() -> Self {
				let mut mat = $V::<T> { d: [T::zero(); ($N*$N)] };
				for i in 0..$N {
					mat.d[i*(1 + $N)] = T::one();
				}
				mat
			}
		}
	)
}

vnm_one!(mat2x2, 2);
vnm_one!(mat3x3, 3);
vnm_one!(mat4x4, 4);

macro_rules! vnm_submatrix {
	($Vs:ident, $Vr:ident, $N:expr) => (
		impl<T> $Vs<T> where T: Copy + Default {
			pub fn submatrix(self, x: usize, y: usize) -> $Vr<T> {
				vnm_map![i, j; self[(i + (i >= x) as usize, j + (j >= y) as usize)]; $Vr, $N-1, $N-1]
			}
		}
		
	)
}

impl<T> mat2x2<T> where T: Copy + Default {
	pub fn submatrix(self, x:usize, y:usize) -> T {
		self[(1 - x, 1 - y)]
	}
}

vnm_submatrix!(mat4x4, mat3x3, 4);
vnm_submatrix!(mat3x3, mat2x2, 3);

pub trait Det<T> {
	fn det(self) -> T;
}

impl<T> Det<T> for T where T: Copy + Default + Num {
	fn det(self) -> T {
		self
	}
}

macro_rules! vnm_cofactor {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Default + Num + Signed {
			pub fn cofactor(self, x: usize, y: usize) -> T {
				(if (x + y) % 2 == 0 { T::one() } else { -T::one() })*self.submatrix(x,y).det()
			}
		}
	)
}

vnm_cofactor!(mat4x4, 4);
vnm_cofactor!(mat3x3, 3);
vnm_cofactor!(mat2x2, 2);

/* Determinant */
macro_rules! vnm_det {
	($V:ident, $N:expr) => (
		impl<T> Det<T> for $V<T> where T: Copy + Default + Num + Signed {
			fn det(self) -> T {
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

vnm_det!(mat4x4, 4);
vnm_det!(mat3x3, 3);
vnm_det!(mat2x2, 2);

/* Adjugate matrix */
macro_rules! vnm_adj {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Default + Num + Signed {
			pub fn adj(self) -> $V<T> {
				vnm_map![i, j; self.cofactor(j, i); $V, $N, $N]
			}
		}
	)
}

vnm_adj!(mat4x4, 4);
vnm_adj!(mat3x3, 3);
vnm_adj!(mat2x2, 2);

/* Inverse matrix */
macro_rules! vnm_inverse {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Default + Num + Signed {
			pub fn inverse(self) -> $V<T> {
				self.adj()/self.det()
			}
		}
	)
}

vnm_inverse!(mat4x4, 4);
vnm_inverse!(mat3x3, 3);
vnm_inverse!(mat2x2, 2);

#[allow(non_camel_case_types)]
pub type mat2<T> = mat2x2<T>;
#[allow(non_camel_case_types)]
pub type mat3<T> = mat3x3<T>;
#[allow(non_camel_case_types)]
pub type mat4<T> = mat4x4<T>;