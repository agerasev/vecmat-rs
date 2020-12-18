use crate::{vector::*};
use core::{
    slice,
};
use super::Matrix;

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
