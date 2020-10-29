use core::{
	convert::{TryFrom, TryInto},
	ops::{Index, IndexMut},
	iter::{IntoIterator},
	array::{TryFromSliceError},
	slice,
	fmt::{Display, Formatter, Result as FmtResult},
};
use crate::{array::*};


/// Vector of fixed-size.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<T, const N: usize> {
	data: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
	/// Initialize vector by closure.
	pub fn init<F: FnMut() -> T>(f: F) -> Self {
		Self { data: <[T; N]>::init_ext(f) }
	}
}
impl<T, const N: usize> Default for Vector<T, N> where T: Default {
	/// Create vector filled with default values.
	fn default() -> Self {
		Self::init(|| T::default())
	}
}
impl<T, const N: usize> Vector<T, N> where T: Default {
	/// Create default vector.
	pub fn new() -> Self {
		Self::default()
	}
}
impl<T, const N: usize> Vector<T, N> where T: Clone {
	/// Create vector which elements are filled with scalar value.
	pub fn fill(v: T) -> Self {
		Self::init(|| v.clone())
	}
	/// Fill with a scalar value reference.
	pub fn fill_ref(v: &T) -> Self {
		Self::init(|| v.clone())
	}
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
	fn from(a: [T; N]) -> Self {
		Self { data: a }
	}
}
impl<T, const N: usize> From<&[T; N]> for Vector<T, N> where T: Clone {
	fn from(ar: &[T; N]) -> Self {
		Self { data: ar.clone() }
	}
}

impl<T, const N: usize> Into<[T; N]> for Vector<T, N> {
	fn into(self) -> [T; N] {
		self.data
	}
}
impl<'a, T, const N: usize> Into<&'a [T; N]> for &'a Vector<T, N> {
	fn into(self) -> &'a [T; N] {
		&self.data
	}
}
impl<'a, T, const N: usize> Into<&'a mut [T; N]> for &'a mut Vector<T, N> {
	fn into(self) -> &'a mut [T; N] {
		&mut self.data
	}
}

impl<T, const N: usize> AsRef<[T; N]> for Vector<T, N> {
	fn as_ref(&self) -> &[T; N] {
		&self.data
	}
}
impl<T, const N: usize> AsMut<[T; N]> for Vector<T, N> {
	fn as_mut(&mut self) -> &mut [T; N] {
		&mut self.data
	}
}

impl<'a, T, const N: usize> TryFrom<&'a [T]> for Vector<T, N> where T: Copy {
	type Error = TryFromSliceError;
	fn try_from(s: &'a [T]) -> Result<Self, Self::Error> {
		s.try_into().map(|a| Self { data: a })
	}
}
impl<T, const N: usize> Vector<T, N> {
	fn try_from_iter<I>(mut i: I) -> Result<Self, ()> where I: Iterator<Item=T> {
		<[T; N]>::from_iter_ext(&mut i).map(|a| a.into()).ok_or(())
	}
}


impl<T, const N: usize> Index<usize> for Vector<T, N> {
	type Output = T;
	fn index(&self, i: usize) -> &T {
		&self.data[i]
	}
}
impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
	fn index_mut(&mut self, i: usize) -> &mut T {
		&mut self.data[i]
	}
}


impl<T, const N: usize> Vector<T, N> {
	/// Returns iterator over vector element refrences.
	pub fn iter(&self) -> impl Iterator<Item=&T> {
		self.data.iter()
	}
	/// Returns iterator over vector element mutable refrences.
	pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
		self.data.iter_mut()
	}
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
	type Item = T;
	type IntoIter = <[T; N] as ArrayExt<T, N>>::IntoIter_;
	fn into_iter(self) -> Self::IntoIter {
		self.data.into_iter_ext()
	}
}
impl<'a, T, const N: usize> IntoIterator for &'a Vector<T, N> {
	type Item = &'a T;
	type IntoIter = slice::Iter<'a, T>;
	fn into_iter(self) -> Self::IntoIter {
		self.data.iter()
	}
}
impl<'a, T, const N: usize> IntoIterator for &'a mut Vector<T, N> {
	type Item = &'a mut T;
	type IntoIter = slice::IterMut<'a, T>;
	fn into_iter(self) -> Self::IntoIter {
		self.data.iter_mut()
	}
}

impl<const N: usize> Vector<usize, N> {
	/// Create vector which element value equals to its position in vector.
	pub fn range() -> Self {
		Self::try_from_iter(0..N).unwrap()
	}
}

impl<T, const N: usize> Vector<T, N> {
	/// Call closure for each element of the vector passing it by value.
	pub fn for_each<F: FnMut(T)>(self, f: F) {
		self.data.for_each_ext(f)
	}
	/// Map vector elements.
	pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> Vector<U, N> {
		self.data.map_ext(f).into()
	}
	/// Zip two vectors into one.
	pub fn zip<U>(self, other: Vector<U, N>) -> Vector<(T, U), N> {
		self.data.zip_ext(other.data).into()
	}
}
impl<T, U, const N: usize> Vector<(T, U), N> {
	/// Unzip vector of tuples into two vectors.
	pub fn unzip(self) -> (Vector<T, N>, Vector<U, N>) {
		let z = self.data.unzip_ext();
		(z.0.into(), z.1.into())
	}
}
impl<T, const N: usize> Vector<T, N> {
	pub fn fold<S, F: FnMut(S, T) -> S>(self, s: S, f: F) -> S {
		self.data.fold_ext(s, f)
	}
	pub fn fold_first<F: FnMut(T, T) -> T>(self, f: F) -> T {
		self.data.fold_first_ext(f)
	}
	pub fn scan<S, U, F: FnMut(&mut S, T) -> U>(self, s: S, f: F) -> [U; N] {
		self.data.scan_ext(s, f).into()
	}
}


impl<T, const N: usize> Display for Vector<T, N> where T: Display {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "Vector{}(", N)?;
		for i in 0..N-1 {
			write!(f, "{}, ", self[i])?;
		}
		write!(f, "{})", self[N-1])?;
		Ok(())
	}
}
