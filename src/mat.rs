use std::ops::{Index, IndexMut, Neg};
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use num::{/*Num, Zero, One,*/ Signed};


macro_rules! vnm_struct {
	($V:ident, $N:expr, $M:expr) => (
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy, PartialEq)]
		pub struct $V<T: Copy> {
			pub d: [T; $N*$M],
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
		impl<T> Neg for $V<T> where T: Copy + Default + Signed {
			type Output = $V<T>;
			fn neg(self) -> Self::Output {
				vnm_map![i, j; -self[(i, j)]; $V, $N, $M]
			}
		}
	)
}

/*
macro_rules! vnm_zero {
	($V:ident, $N:expr) => (
		impl<T> Zero for $V<T> where T: Copy + Num + Zero {
			fn zero() -> Self {
				$V::<T> { d: [T::zero(); ($N*$N)] }
			}

			fn is_zero(&self) -> bool {
				for i in 0..($N*$N) {
					if !self.d[i].is_zero() {
						return false;
					}
				}
				true
			}
		}
	)
}

macro_rules! vnm_one {
	($V:ident, $N:expr) => (
		impl<T> One for $V<T> where T: Copy + Num + Zero + One {
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
*/

macro_rules! vnm_all {
	($V:ident, $N:expr, $M:expr) => (
		vnm_struct!($V, $N, $M);
		vnm_fmt!($V, $N, $M);
		vnm_index!($V, $N, $M);
		vnm_new!($V, $N, $M);
		vnm_from!($V, $N, $M);
		vnm_neg!($V, $N, $M);
		//vnm_zero!($V, $N);
		//vnm_one!($V, $N);
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

#[allow(non_camel_case_types)]
type mat2<T> = mat2x2<T>;
#[allow(non_camel_case_types)]
type mat3<T> = mat3x3<T>;
#[allow(non_camel_case_types)]
type mat4<T> = mat4x4<T>;