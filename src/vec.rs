extern crate num;

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::convert::From;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct v2<T> {
	pub x: T,
	pub y: T,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
struct _wrap<T> {
	pub val: T,
}

impl<T> From<T> for _wrap<T> {
	fn from(t: T) -> Self {
		_wrap::<T> {val: t}
	}
}

impl<T> From<(T,T)> for v2<T> where T: Copy {
	fn from(t: (T,T)) -> Self {
		v2::<T>{x: t.0, y: t.1}
	}
}

impl<T> From<[T;2]> for v2<T> where T: Copy {
	fn from(a: [T;2]) -> Self {
		v2::<T>{x: a[0], y: a[1]}
	}
}

impl<T> Add<v2<T>> for v2<T> where T: num::Num {
	type Output = v2<T>;
	fn add(self, other: v2<T>) -> Self::Output {
		v2::<T>{x: self.x + other.x, y: self.y + other.y}
	}
}

impl<T> Sub<v2<T>> for v2<T> where T: num::Num {
	type Output = v2<T>;
	fn sub(self, other: v2<T>) -> Self::Output {
		v2::<T>{x: self.x - other.x, y: self.y - other.y}
	}
}

impl<T> Mul<v2<T>> for v2<T> where T: num::Num {
	type Output = v2<T>;
	fn mul(self, v: v2<T>) -> Self::Output {
		v2::<T>{x: self.x*v.x, y: self.y*v.y}
	}
}

impl<T> Mul<T> for v2<T> where T: num::Num + Copy {
	type Output = v2<T>;
	fn mul(self, a: T) -> Self::Output {
		v2::<T>{x: self.x*a, y: self.y*a}
	}
}

impl<T> Mul<v2<T>> for _wrap<T> where T: num::Num + Copy {
	type Output = v2<T>;
	fn mul(self, v: v2<T>) -> Self::Output {
		v2::<T>{x: self.val*v.x, y: self.val*v.y}
	}
}

impl<T> Div<v2<T>> for v2<T> where T: num::Num {
	type Output = v2<T>;
	fn div(self, v: v2<T>) -> Self::Output {
		v2::<T>{x: self.x/v.x, y: self.y/v.y}
	}
}

impl<T> Div<T> for v2<T> where T: num::Num + Copy {
	type Output = v2<T>;
	fn div(self, a: T) -> Self::Output {
		v2::<T>{x: self.x/a, y: self.y/a}
	}
}

impl<T> Div<v2<T>> for _wrap<T> where T: num::Num + Copy {
	type Output = v2<T>;
	fn div(self, v: v2<T>) -> Self::Output {
		v2::<T>{x: self.val/v.x, y: self.val/v.y}
	}
}

impl<T> Neg for v2<T> where T: num::Signed {
	type Output = v2<T>;
	fn neg(self) -> Self::Output {
		v2::<T>{x: -self.x, y: -self.y}
	}
}

#[allow(non_camel_case_types)]
type v2bool = v2<bool>;
#[allow(non_camel_case_types)]
type v2i8 = v2<i8>;
#[allow(non_camel_case_types)]
type v2u8 = v2<u8>;
#[allow(non_camel_case_types)]
type v2i16 = v2<i16>;
#[allow(non_camel_case_types)]
type v2u16 = v2<u16>;
#[allow(non_camel_case_types)]
type v2i32 = v2<i32>;
#[allow(non_camel_case_types)]
type v2u32 = v2<u32>;
#[allow(non_camel_case_types)]
type v2f32 = v2<f32>;
#[allow(non_camel_case_types)]
type v2f64 = v2<f64>;
