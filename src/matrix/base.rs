use core::{
	convert::{TryFrom},
	ops::{Index, IndexMut},
	slice,
	fmt::{Display, Formatter, Result as FmtResult},
};
use crate::vector::*;


/// Matrix with fixed dimensions.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
	data: Vector<Vector<T, N>, M>,
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	/// Initialize matrix by closure.
	pub fn init<F: FnMut() -> T>(mut f: F) -> Self {
		Self { data: Vector::init(|| Vector::init(&mut f)) }
	}
}
impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N> where T: Default {
	/// Create matrix filled with default values.
	fn default() -> Self {
		Self::init(|| T::default())
	}
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N> where T: Default {
	/// Create default matrix.
	pub fn new() -> Self {
		Self::default()
	}
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N> where T: Clone {
	/// Create matrix which elements are filled with scalar value.
	pub fn fill(v: T) -> Self {
		Self::init(|| v.clone())
	}
	/// Fill with a scalar value reference.
	pub fn fill_ref(v: &T) -> Self {
		Self::init(|| v.clone())
	}
}

impl<T, const M: usize, const N: usize> From<Vector<Vector<T, N>, M>> for Matrix<T, M, N> {
	fn from(a: Vector<Vector<T, N>, M>) -> Self {
		Self { data: a }
	}
}
impl<T, const M: usize, const N: usize> From<&Vector<Vector<T, N>, M>> for Matrix<T, M, N> where T: Clone {
	fn from(ar: &Vector<Vector<T, N>, M>) -> Self {
		Matrix::from(ar.clone())
	}
}
impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
	fn from(a: [[T; N]; M]) -> Self {
		Matrix::from(Vector::from(a).map(|b| Vector::from(b)))
	}
}
impl<T, const M: usize, const N: usize> From<&[[T; N]; M]> for Matrix<T, M, N> where T: Clone {
	fn from(ar: &[[T; N]; M]) -> Self {
		Matrix::from(ar.clone())
	}
}

impl<T, const M: usize, const N: usize> Into<Vector<Vector<T, N>, M>> for Matrix<T, M, N> {
	fn into(self) -> Vector<Vector<T, N>, M> {
		self.data
	}
}
impl<T, const M: usize, const N: usize> Into<[[T; N]; M]> for Matrix<T, M, N> {
	fn into(self) -> [[T; N]; M] {
		self.data.map(|a| a.into()).into()
	}
}

impl<T, const M: usize, const N: usize> AsRef<Vector<Vector<T, N>, M>> for Matrix<T, M, N> {
	fn as_ref(&self) -> &Vector<Vector<T, N>, M> {
		&self.data
	}
}
impl<T, const M: usize, const N: usize> AsMut<Vector<Vector<T, N>, M>> for Matrix<T, M, N> {
	fn as_mut(&mut self) -> &mut Vector<Vector<T, N>, M> {
		&mut self.data
	}
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn as_array(&self) -> &[[T; N]; M] {
		unsafe { (self as *const _ as *const [[T; N]; M]).as_ref().unwrap() }
	}
	pub fn as_mut_array(&mut self) -> &mut [[T; N]; M] {
		unsafe { (self as *mut _ as *mut [[T; N]; M]).as_mut().unwrap() }
	}
}

impl<'a, T, const M: usize, const N: usize> Into<&'a Vector<Vector<T, N>, M>> for &'a Matrix<T, M, N> {
	fn into(self) -> &'a Vector<Vector<T, N>, M> {
		self.as_ref()
	}
}
impl<'a, T, const M: usize, const N: usize> Into<&'a mut Vector<Vector<T, N>, M>> for &'a mut Matrix<T, M, N> {
	fn into(self) -> &'a mut Vector<Vector<T, N>, M> {
		self.as_mut()
	}
}
impl<'a, T, const M: usize, const N: usize> Into<&'a [[T; N]; M]> for &'a Matrix<T, M, N> {
	fn into(self) -> &'a [[T; N]; M] {
		self.as_array()
	}
}
impl<'a, T, const M: usize, const N: usize> Into<&'a mut [[T; N]; M]> for &'a mut Matrix<T, M, N> {
	fn into(self) -> &'a mut [[T; N]; M] {
		self.as_mut_array()
	}
}

impl<'a, T, const M: usize, const N: usize> TryFrom<&'a [T]> for Matrix<T, M, N> where T: Copy {
	type Error = ();
	fn try_from(s: &'a [T]) -> Result<Self, ()> {
		if s.len() == M*N {
			Ok(Matrix::try_from_iter(s.iter().cloned()).unwrap())
		} else {
			Err(())
		}
	}
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	fn try_from_iter<I>(i: I) -> Result<Self, ()> where I: Iterator<Item=T> {
		<Vector<Vector<T, N>, M>>::try_from_iter(&mut GroupIter::new(i)).map(|a| a.into())
		.map(|a| Self { data: a })
	}
}

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<T, M, N> {
	type Output = T;
	fn index(&self, (j, i): (usize, usize)) -> &T {
		&self.data[j][i]
	}
}
impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<T, M, N> {
	fn index_mut(&mut self, (j, i): (usize, usize)) -> &mut T {
		&mut self.data[j][i]
	}
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	/// Returns iterator over matrix element refrences.
	pub fn iter(&self) -> impl Iterator<Item=&T> {
		FlatIter::new(self.data.iter()).unwrap()
	}
	/// Returns iterator over matrix element mutable refrences.
	pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
		FlatIter::new(self.data.iter_mut()).unwrap()
	}
}

impl<T, const M: usize, const N: usize> IntoIterator for Matrix<T, M, N> {
	type Item = T;
	type IntoIter = FlatIter<<Vector<Vector<T, N>, M> as IntoIterator>::IntoIter, Vector<T, N>, <Vector<T, N> as IntoIterator>::IntoIter>;
	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter::new(self.data.into_iter()).unwrap()
	}
}
impl<'a, T, const M: usize, const N: usize> IntoIterator for &'a Matrix<T, M, N> {
	type Item = &'a T;
	type IntoIter = FlatIter<slice::Iter<'a, Vector<T, N>>, &'a Vector<T, N>, slice::Iter<'a, T>>;
	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter::new(self.data.iter()).unwrap()
	}
}
impl<'a, T, const M: usize, const N: usize> IntoIterator for &'a mut Matrix<T, M, N> {
	type Item = &'a mut T;
	type IntoIter = FlatIter<slice::IterMut<'a, Vector<T, N>>, &'a mut Vector<T, N>, slice::IterMut<'a, T>>;
	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter::new(self.data.iter_mut()).unwrap()
	}
}

impl<const M: usize, const N: usize> Matrix<(usize, usize), M, N> {
	/// Create matrix which elements are tuples (j, i) where j and i are coordinates of the matrix cell.
	pub fn indices() -> Self {
		Self::try_from_iter((0..(M*N)).map(|x| (x / N, x % N))).unwrap()
	}
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	/// Call closure for each element of the matrix passing it by value.
	pub fn for_each<F: FnMut(T)>(self, mut f: F) {
		self.data.for_each(|a| a.for_each(&mut f))
	}
	/// Map matrix elements.
	pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Matrix<U, M, N> {
		self.data.map(|a| a.map(&mut f)).into()
	}
	/// Zip two matrices into one.
	pub fn zip<U>(self, other: Matrix<U, M, N>) -> Matrix<(T, U), M, N> {
		self.data.zip(other.data).map(|(a, b)| a.zip(b)).into()
	}
}
impl<T, U, const M: usize, const N: usize> Matrix<(T, U), M, N> {
	/// Unzip matrix of tuples into two matrices.
	pub fn unzip(self) -> (Matrix<T, M, N>, Matrix<U, M, N>) {
		let z = self.data.map(|a| a.unzip()).unzip();
		(z.0.into(), z.1.into())
	}
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn fold<S, F: Fn(S, T) -> S>(self, s: S, mut f: F) -> S {
		self.data.fold(s, |t, a| a.fold(t, &mut f))
	}
	pub fn fold_first<F: FnMut(T, T) -> T>(self, mut f: F) -> T {
		let mut iter = self.data.into_iter();
		let s = iter.next().unwrap().fold_first(&mut f);
		iter.fold(s, |t, a| a.fold(t, &mut f))
	}
	pub fn scan<S, U, F: FnMut(&mut S, T) -> U>(self, mut s: S, mut f: F) -> Matrix<U, M, N> {
		self.data.scan(&mut s, |r, a| a.scan(r, |r, x| f(*r, x))).into()
	}
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn into_inner(self) -> Vector<Vector<T, N>, M> {
		self.into()
	}
	pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> &T {
		self.as_ref().get_unchecked(i).get_unchecked(j)
	}
	pub unsafe fn get_unchecked_mut(&mut self, i: usize, j: usize) -> &mut T {
		self.as_mut().get_unchecked_mut(i).get_unchecked_mut(j)
	}
}

impl<T, const M: usize, const N: usize> Display for Matrix<T, M, N> where T: Display {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		writeln!(f)?;
		writeln!(f, "Matrix{}x{}(", M, N)?;
		for j in 0..M {
			write!(f, "  ")?;
			for i in 0..N {
				write!(f, "{}, ", self[(j, i)])?;
			}
			writeln!(f)?;
		}
		writeln!(f, ")")?;
		Ok(())
	}
}
