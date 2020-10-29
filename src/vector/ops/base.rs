use core::{
	ops::{
		Neg, Add, Sub, Mul, Div, Rem,
		AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
	},
	cmp::{PartialOrd},
	iter::{IntoIterator},
};
use num_traits::{Zero};
use crate::{vector::*};



impl<T, const N: usize> Neg for Vector<T, N> where T: Neg<Output=T> {
	type Output = Vector<T, N>;
	fn neg(self) -> Self::Output {
		self.map(|v| -v)
	}
}

impl<T, const N: usize> Add for Vector<T, N> where T: Add<Output=T> {
	type Output = Vector<T, N>;
	fn add(self, vec: Vector<T, N>) -> Self::Output {
		self.zip(vec).map(|(x, y)| x + y)
	}
}
impl<T, const N: usize> Sub for Vector<T, N> where T: Sub<Output=T> {
	type Output = Vector<T, N>;
	fn sub(self, vec: Vector<T, N>) -> Self::Output {
		self.zip(vec).map(|(x, y)| x - y)
	}
}
impl<T, const N: usize> Mul for Vector<T, N> where T: Mul<Output=T> {
	type Output = Vector<T, N>;
	fn mul(self, vec: Vector<T, N>) -> Self::Output {
		self.zip(vec).map(|(x, y)| x * y)
	}
}
impl<T, const N: usize> Div for Vector<T, N> where T: Div<Output=T> {
	type Output = Vector<T, N>;
	fn div(self, vec: Vector<T, N>) -> Self::Output {
		self.zip(vec).map(|(x, y)| x / y)
	}
}
impl<T, const N: usize> Rem for Vector<T, N> where T: Rem<Output=T> {
	type Output = Vector<T, N>;
	fn rem(self, vec: Vector<T, N>) -> Self::Output {
		self.zip(vec).map(|(x, y)| x % y)
	}
}

impl<T, const N: usize> Mul<T> for Vector<T, N> where T: Mul<Output=T> + Clone {
	type Output = Vector<T, N>;
	fn mul(self, a: T) -> Self::Output {
		self.map(|v| v * a.clone())
	}
}
impl<T, const N: usize> Div<T> for Vector<T, N> where T: Div<Output=T> + Clone {
	type Output = Vector<T, N>;
	fn div(self, a: T) -> Self::Output {
		self.map(|v| v / a.clone())
	}
}
impl<T, const N: usize> Rem<T> for Vector<T, N> where T: Rem<Output=T> + Clone {
	type Output = Vector<T, N>;
	fn rem(self, a: T) -> Self::Output {
		self.map(|v| v % a.clone())
	}
}

impl<T, const N: usize> AddAssign for Vector<T, N> where T: AddAssign {
	fn add_assign(&mut self, vec: Vector<T, N>) {
		self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| { *s += x; })
	}
}
impl<T, const N: usize> SubAssign for Vector<T, N> where T: SubAssign {
	fn sub_assign(&mut self, vec: Vector<T, N>) {
		self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| { *s -= x; })
	}
}
impl<T, const N: usize> MulAssign for Vector<T, N> where T: MulAssign {
	fn mul_assign(&mut self, vec: Vector<T, N>) {
		self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| { *s *= x; })
	}
}
impl<T, const N: usize> DivAssign for Vector<T, N> where T: DivAssign {
	fn div_assign(&mut self, vec: Vector<T, N>) {
		self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| { *s /= x; })
	}
}
impl<T, const N: usize> RemAssign for Vector<T, N> where T: RemAssign {
	fn rem_assign(&mut self, vec: Vector<T, N>) {
		self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| { *s %= x; })
	}
}

impl<T, const N: usize> MulAssign<T> for Vector<T, N> where T: MulAssign + Clone {
	fn mul_assign(&mut self, a: T) {
		self.iter_mut().for_each(|s| { *s *= a.clone(); })
	}
}
impl<T, const N: usize> DivAssign<T> for Vector<T, N> where T: DivAssign + Clone {
	fn div_assign(&mut self, a: T) {
		self.iter_mut().for_each(|s| { *s /= a.clone(); })
	}
}
impl<T, const N: usize> RemAssign<T> for Vector<T, N> where T: RemAssign + Clone {
	fn rem_assign(&mut self, a: T) {
		self.iter_mut().for_each(|s| { *s %= a.clone(); })
	}
}

impl<T, const N: usize> Zero for Vector<T, N> where T: Zero {
	fn zero() -> Self {
		Self::init(|| T::zero())
	}
	fn is_zero(&self) -> bool {
		self.iter().all(|x| x.is_zero())
	}
}

impl<T, const N: usize> Vector<T, N> {
	pub fn sum(self) -> T where T: Add<Output=T> {
		self.fold_first(|x, y| x + y)
	}
	pub fn max(self) -> T where T: PartialOrd {
		self.fold_first(|x, y| if x < y { y } else { x })
	}
	pub fn min(self) -> T where T: PartialOrd {
		self.fold_first(|x, y| if x < y { x } else { y })
	}
}


macro_rules! vec_mul_scal_rev { ($N:expr, $T:ident) => (
    impl Mul<Vector<$T, $N>> for $T {
        type Output = Vector<$T, $N>;
        fn mul(self, a: Vector<$T, $N>) -> Self::Output {
            a * self
        }
    }
) }

// T * VecN<T> workaround
cartesian!(
	vec_mul_scal_rev,
	[2, 3, 4],
	[i8, u8, i16, u16, i32, u32, i64, u64, f32, f64]
);
