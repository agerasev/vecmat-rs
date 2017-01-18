use num::{Num};
use std::ops::{Add};

macro_rules! vn_struct {
	($V:ident, $N:expr) => (
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy)]
		pub struct $V<T: Copy> {
			pub d: [T; $N],
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

macro_rules! vn_add {
	($V:ident, $N:expr) => (
		impl<T> Add<$V<T>> for $V<T> where T: Copy + Num {
			type Output = $V<T>;
			fn add(self, other: $V<T>) -> Self::Output {
				let mut out = $V::<T> { d: self.d };
				for i in 0..$N {
					out.d[i] = self.d[i] + other.d[i];
				}
				out
			}
		}
	)
}

macro_rules! vn_all {
	($V:ident, $N:expr) => (
		vn_struct!($V, $N);
		vn_from!($V, $N);
		vn_add!($V, $N);
	)
}

vn_all!(v2, 2);
vn_all!(v3, 3);
vn_all!(v4, 4);