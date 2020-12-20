#![allow(clippy::missing_safety_doc)]

use super::Matrix;
use crate::{traits::*, vector::*};
use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    ptr,
};

impl<T, const M: usize, const N: usize> Matrix<MaybeUninit<T>, M, N> {
    /// Transpose `MaybeUninit<Matrix<T, M, N>>` into `Matrix<MaybeUninit<T>, M, N>`.
    fn from_uninit(uninit: MaybeUninit<Matrix<T, M, N>>) -> Self {
        // TODO: Use `mem::transmute` when it will be possible.
        unsafe { ptr::read(&uninit as *const _ as *const Matrix<MaybeUninit<T>, M, N>) }
    }
    /// Transpose `Matrix<MaybeUninit<T>, M, N>` into `MaybeUninit<Matrix<T, M, N>>`.
    fn into_uninit(self) -> MaybeUninit<Matrix<T, M, N>> {
        // TODO: Use `mem::transmute` when it will be possible.
        unsafe { ptr::read(&self as *const _ as *const MaybeUninit<Matrix<T, M, N>>) }
    }
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    /// Create a matrix with uninitialized content.
    pub fn uninit() -> Matrix<MaybeUninit<T>, M, N> {
        Matrix::from_uninit(MaybeUninit::uninit())
    }
}
impl<T, const M: usize, const N: usize> Matrix<MaybeUninit<T>, M, N> {
    /// Assume that matrix content is initialized.
    pub unsafe fn assume_init(self) -> Matrix<T, M, N> {
        self.into_uninit().assume_init()
    }
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
    pub fn row_ref(&self, i: usize) -> &Vector<T, N> {
        &self.as_vector_of_vectors()[i]
    }
    pub fn row_mut(&mut self, i: usize) -> &mut Vector<T, N> {
        &mut self.as_mut_vector_of_vectors()[i]
    }
}
impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: ImplicitClone,
{
    pub fn row(&self, i: usize) -> Vector<T, N> {
        self.row_ref(i).clone()
    }
    pub fn col(&self, j: usize) -> Vector<T, M> {
        Vector::indices().map(|i| self[(i, j)].clone())
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    /// Get pointer to the first element.
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr() as *const T
    }
    /// Get mutable pointer to the first element.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr() as *mut T
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
