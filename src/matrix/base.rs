#![allow(clippy::missing_safety_doc)]

use crate::{traits::*, vector::*};
use core::{
    convert::TryFrom,
    ops::{Index, IndexMut},
    slice,
};

/// Matrix with fixed dimensions.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: Vector<Vector<T, N>, M>,
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    /// Initialize matrix by closure.
    pub fn init<F: FnMut() -> T>(mut f: F) -> Self {
        Self {
            data: Vector::init(|| Vector::init(&mut f)),
        }
    }
}
impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
where
    T: Default,
{
    /// Create matrix filled with default values.
    fn default() -> Self {
        Self::init(T::default)
    }
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Default,
{
    /// Create default matrix.
    pub fn new() -> Self {
        Self::default()
    }
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: ImplicitClone,
{
    /// Create matrix which elements are filled with scalar value.
    pub fn fill(v: T) -> Self {
        Self::init(|| v.clone())
    }
    /// Fill with a scalar value reference.
    pub fn fill_ref(v: &T) -> Self {
        Self::init(|| v.clone())
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn from_vector_of_vectors(a: Vector<Vector<T, N>, M>) -> Self {
        Self { data: a }
    }
    pub fn from_array_of_vectors(a: [Vector<T, N>; M]) -> Self {
        Self::from_vector_of_vectors(Vector::from_array(a))
    }
    pub fn from_array_of_arrays(a: [[T; N]; M]) -> Self {
        Self::from_vector_of_vectors(Vector::from_array(a).map(Vector::from_array))
    }
    pub fn into_vector_of_vectors(self) -> Vector<Vector<T, N>, M> {
        self.data
    }
    pub fn into_array_of_vectors(self) -> [Vector<T, N>; M] {
        self.into_vector_of_vectors().into_array()
    }
    pub fn into_array_of_arrays(self) -> [[T; N]; M] {
        self.into_vector_of_vectors()
            .map(|a| a.into_array())
            .into_array()
    }
}

impl<T, const M: usize, const N: usize> From<Vector<Vector<T, N>, M>> for Matrix<T, M, N> {
    fn from(a: Vector<Vector<T, N>, M>) -> Self {
        Self::from_vector_of_vectors(a)
    }
}
impl<T, const M: usize, const N: usize> From<[Vector<T, N>; M]> for Matrix<T, M, N> {
    fn from(a: [Vector<T, N>; M]) -> Self {
        Self::from_array_of_vectors(a)
    }
}
impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
    fn from(a: [[T; N]; M]) -> Self {
        Self::from_array_of_arrays(a)
    }
}
impl<T, const M: usize, const N: usize> Into<Vector<Vector<T, N>, M>> for Matrix<T, M, N> {
    fn into(self) -> Vector<Vector<T, N>, M> {
        self.into_vector_of_vectors()
    }
}
impl<T, const M: usize, const N: usize> Into<[Vector<T, N>; M]> for Matrix<T, M, N> {
    fn into(self) -> [Vector<T, N>; M] {
        self.into_array_of_vectors()
    }
}
impl<T, const M: usize, const N: usize> Into<[[T; N]; M]> for Matrix<T, M, N> {
    fn into(self) -> [[T; N]; M] {
        self.into_array_of_arrays()
    }
}

impl<T, const M: usize, const N: usize> From<&Vector<Vector<T, N>, M>> for Matrix<T, M, N>
where
    T: ImplicitClone,
{
    fn from(ar: &Vector<Vector<T, N>, M>) -> Self {
        Self::from_vector_of_vectors(ar.clone())
    }
}
impl<T, const M: usize, const N: usize> From<&[Vector<T, N>; M]> for Matrix<T, M, N>
where
    T: ImplicitClone,
{
    fn from(ar: &[Vector<T, N>; M]) -> Self {
        Self::from_array_of_vectors(ar.clone())
    }
}
impl<T, const M: usize, const N: usize> From<&[[T; N]; M]> for Matrix<T, M, N>
where
    T: ImplicitClone,
{
    fn from(ar: &[[T; N]; M]) -> Self {
        Self::from_array_of_arrays(ar.clone())
    }
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn as_vector_of_vectors(&self) -> &Vector<Vector<T, N>, M> {
        &self.data
    }
    pub fn as_mut_vector_of_vectors(&mut self) -> &mut Vector<Vector<T, N>, M> {
        &mut self.data
    }
    pub fn as_array_of_vectors(&self) -> &[Vector<T, N>; M] {
        self.as_vector_of_vectors().as_array()
    }
    pub fn as_mut_array_of_vectors(&mut self) -> &mut [Vector<T, N>; M] {
        self.as_mut_vector_of_vectors().as_mut_array()
    }
    pub fn as_array_of_arrays(&self) -> &[[T; N]; M] {
        unsafe { (self as *const _ as *const [[T; N]; M]).as_ref().unwrap() }
    }
    pub fn as_mut_array_of_arrays(&mut self) -> &mut [[T; N]; M] {
        unsafe { (self as *mut _ as *mut [[T; N]; M]).as_mut().unwrap() }
    }
}
impl<T, const M: usize, const N: usize> AsRef<Vector<Vector<T, N>, M>> for Matrix<T, M, N> {
    fn as_ref(&self) -> &Vector<Vector<T, N>, M> {
        self.as_vector_of_vectors()
    }
}
impl<T, const M: usize, const N: usize> AsMut<Vector<Vector<T, N>, M>> for Matrix<T, M, N> {
    fn as_mut(&mut self) -> &mut Vector<Vector<T, N>, M> {
        self.as_mut_vector_of_vectors()
    }
}
impl<T, const M: usize, const N: usize> AsRef<[Vector<T, N>; M]> for Matrix<T, M, N> {
    fn as_ref(&self) -> &[Vector<T, N>; M] {
        self.as_array_of_vectors()
    }
}
impl<T, const M: usize, const N: usize> AsMut<[Vector<T, N>; M]> for Matrix<T, M, N> {
    fn as_mut(&mut self) -> &mut [Vector<T, N>; M] {
        self.as_mut_array_of_vectors()
    }
}
impl<T, const M: usize, const N: usize> AsRef<[[T; N]; M]> for Matrix<T, M, N> {
    fn as_ref(&self) -> &[[T; N]; M] {
        self.as_array_of_arrays()
    }
}
impl<T, const M: usize, const N: usize> AsMut<[[T; N]; M]> for Matrix<T, M, N> {
    fn as_mut(&mut self) -> &mut [[T; N]; M] {
        self.as_mut_array_of_arrays()
    }
}

impl<'a, T, const M: usize, const N: usize> Into<&'a Vector<Vector<T, N>, M>>
    for &'a Matrix<T, M, N>
{
    fn into(self) -> &'a Vector<Vector<T, N>, M> {
        self.as_vector_of_vectors()
    }
}
impl<'a, T, const M: usize, const N: usize> Into<&'a mut Vector<Vector<T, N>, M>>
    for &'a mut Matrix<T, M, N>
{
    fn into(self) -> &'a mut Vector<Vector<T, N>, M> {
        self.as_mut_vector_of_vectors()
    }
}
impl<'a, T, const M: usize, const N: usize> Into<&'a [[T; N]; M]> for &'a Matrix<T, M, N> {
    fn into(self) -> &'a [[T; N]; M] {
        self.as_array_of_arrays()
    }
}
impl<'a, T, const M: usize, const N: usize> Into<&'a mut [[T; N]; M]> for &'a mut Matrix<T, M, N> {
    fn into(self) -> &'a mut [[T; N]; M] {
        self.as_mut_array_of_arrays()
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    fn try_from_iter_of_vectors<I>(i: I) -> Option<Self>
    where
        I: Iterator<Item = Vector<T, N>>,
    {
        <Vector<Vector<T, N>, M>>::try_from_iter(i).map(Self::from_vector_of_vectors)
    }
    fn try_from_iter<I>(i: I) -> Option<Self>
    where
        I: Iterator<Item = T>,
    {
        Self::try_from_iter_of_vectors(GroupIter::new(i))
    }
}
impl<'a, T, const M: usize, const N: usize> TryFrom<&'a [Vector<T, N>]> for Matrix<T, M, N>
where
    T: Copy,
{
    type Error = ();
    fn try_from(s: &'a [Vector<T, N>]) -> Result<Self, ()> {
        if s.len() == M {
            Ok(Matrix::try_from_iter_of_vectors(s.iter().cloned()).unwrap())
        } else {
            Err(())
        }
    }
}
impl<'a, T, const M: usize, const N: usize> TryFrom<&'a [T]> for Matrix<T, M, N>
where
    T: Copy,
{
    type Error = ();
    fn try_from(s: &'a [T]) -> Result<Self, ()> {
        if s.len() == M * N {
            Ok(Matrix::try_from_iter(s.iter().cloned()).unwrap())
        } else {
            Err(())
        }
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
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        FlatIter::new(self.data.iter()).unwrap()
    }
    /// Returns iterator over matrix element mutable refrences.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        FlatIter::new(self.data.iter_mut()).unwrap()
    }
}

impl<T, const M: usize, const N: usize> IntoIterator for Matrix<T, M, N> {
    type Item = T;

    #[allow(clippy::type_complexity)]
    type IntoIter = FlatIter<
        <Vector<Vector<T, N>, M> as IntoIterator>::IntoIter,
        Vector<T, N>,
        <Vector<T, N> as IntoIterator>::IntoIter,
    >;

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
    type IntoIter =
        FlatIter<slice::IterMut<'a, Vector<T, N>>, &'a mut Vector<T, N>, slice::IterMut<'a, T>>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.data.iter_mut()).unwrap()
    }
}

impl<const M: usize, const N: usize> Matrix<(usize, usize), M, N> {
    /// Create matrix which elements are tuples (j, i) where j and i are coordinates of the matrix cell.
    pub fn indices() -> Self {
        Self::try_from_iter((0..(M * N)).map(|x| (x / N, x % N))).unwrap()
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
        self.data
            .scan(&mut s, |r, a| a.scan(r, |r, x| f(*r, x)))
            .into()
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn into_inner(self) -> Vector<Vector<T, N>, M> {
        self.into()
    }
    pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> &T {
        self.as_vector_of_vectors()
            .get_unchecked(i)
            .get_unchecked(j)
    }
    pub unsafe fn get_unchecked_mut(&mut self, i: usize, j: usize) -> &mut T {
        self.as_mut_vector_of_vectors()
            .get_unchecked_mut(i)
            .get_unchecked_mut(j)
    }
}
