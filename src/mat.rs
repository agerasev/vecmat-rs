use std::ops::{Index, IndexMut};
//use num::{Num, Zero, One};


macro_rules! mn_struct {
	($M:ident, $N:expr) => (
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy)]
		pub struct $M<T: Copy> {
			pub d: [T; $N*$N],
		}
	)
}

macro_rules! mn_index {
	($M:ident, $N:expr) => (
		impl<T> Index<(usize, usize)> for $M<T> where T: Copy {
			type Output = T;
			fn index(&self, ij: (usize, usize)) -> &Self::Output {
				&self.d[ij.0 + ij.1*$N]
			}
		}

		impl<T> IndexMut<(usize, usize)> for $M<T> where T: Copy {
			fn index_mut(&mut self, ij: (usize, usize)) -> &mut Self::Output {
				&mut self.d[ij.0 + ij.1*$N]
			}
		}
	)
}

/*
macro_rules! mn_zero {
	($M:ident, $N:expr) => (
		impl<T> Zero for $M<T> where T: Copy + Num + Zero {
			fn zero() -> Self {
				$M::<T> { d: [T::zero(); ($N*$N)] }
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

macro_rules! mn_one {
	($M:ident, $N:expr) => (
		impl<T> One for $M<T> where T: Copy + Num + Zero + One {
			fn one() -> Self {
				let mut mat = $M::<T> { d: [T::zero(); ($N*$N)] };
				for i in 0..$N {
					mat.d[i*(1 + $N)] = T::one();
				}
				mat
			}
		}
	)
}


macro_rules! mn_new {
	($M:ident, $N:expr) => (
		impl<T> $M<T> where T: Copy + Num + Zero {
			pub fn new() -> Self {
				$M::<T>::zero()
			}
		}
	)
}
*/

macro_rules! mn_all {
	($M:ident, $N:expr) => (
		mn_struct!($M, $N);
		mn_index!($M, $N);
		//mn_zero!($M, $N);
		//mn_one!($M, $N);
		//mn_new!($M, $N);
	)
}

mn_all!(mat2, 2);
mn_all!(mat3, 3);
mn_all!(mat4, 4);