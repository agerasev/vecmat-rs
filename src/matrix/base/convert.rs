use crate::{vector::GroupIter, Matrix, Vector};
use core::convert::TryFrom;

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
impl<T, const M: usize, const N: usize> From<Matrix<T, M, N>> for Vector<Vector<T, N>, M> {
    fn from(mat: Matrix<T, M, N>) -> Self {
        mat.into_vector_of_vectors()
    }
}
impl<T, const M: usize, const N: usize> From<Matrix<T, M, N>> for [Vector<T, N>; M] {
    fn from(mat: Matrix<T, M, N>) -> Self {
        mat.into_array_of_vectors()
    }
}
impl<T, const M: usize, const N: usize> From<Matrix<T, M, N>> for [[T; N]; M] {
    fn from(mat: Matrix<T, M, N>) -> Self {
        mat.into_array_of_arrays()
    }
}

impl<T, const M: usize, const N: usize> From<&Vector<Vector<T, N>, M>> for Matrix<T, M, N>
where
    T: Copy,
{
    fn from(ar: &Vector<Vector<T, N>, M>) -> Self {
        Self::from_vector_of_vectors(*ar)
    }
}
impl<T, const M: usize, const N: usize> From<&[Vector<T, N>; M]> for Matrix<T, M, N>
where
    T: Copy,
{
    fn from(ar: &[Vector<T, N>; M]) -> Self {
        Self::from_array_of_vectors(*ar)
    }
}
impl<T, const M: usize, const N: usize> From<&[[T; N]; M]> for Matrix<T, M, N>
where
    T: Copy,
{
    fn from(ar: &[[T; N]; M]) -> Self {
        Self::from_array_of_arrays(*ar)
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
/*
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
*/

impl<'a, T, const M: usize, const N: usize> From<&'a Matrix<T, M, N>>
    for &'a Vector<Vector<T, N>, M>
{
    fn from(mr: &'a Matrix<T, M, N>) -> Self {
        mr.as_vector_of_vectors()
    }
}
impl<'a, T, const M: usize, const N: usize> From<&'a mut Matrix<T, M, N>>
    for &'a mut Vector<Vector<T, N>, M>
{
    fn from(mr: &'a mut Matrix<T, M, N>) -> Self {
        mr.as_mut_vector_of_vectors()
    }
}
impl<'a, T, const M: usize, const N: usize> From<&'a Matrix<T, M, N>> for &'a [[T; N]; M] {
    fn from(mr: &'a Matrix<T, M, N>) -> Self {
        mr.as_array_of_arrays()
    }
}
impl<'a, T, const M: usize, const N: usize> From<&'a mut Matrix<T, M, N>> for &'a mut [[T; N]; M] {
    fn from(mr: &'a mut Matrix<T, M, N>) -> Self {
        mr.as_mut_array_of_arrays()
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn try_from_iter_of_vectors<I>(i: I) -> Option<Self>
    where
        I: Iterator<Item = Vector<T, N>>,
    {
        <Vector<Vector<T, N>, M>>::try_from_iter(i).map(Self::from_vector_of_vectors)
    }
    pub fn try_from_iter<I>(i: I) -> Option<Self>
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
