use crate::{matrix::*, traits::*, vector::*};
use core::ops::{Add, Mul};

impl<T, const M: usize, const N: usize> Outer<Vector<T, N>> for Vector<T, M>
where
    T: Mul<Output = T> + ImplicitClone,
{
    type Output = Matrix<T, M, N>;
    fn outer(self, other: Vector<T, N>) -> Self::Output {
        Matrix::indices().map(|(i, j)| self[i].clone() * other[j].clone())
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

impl<T, const M: usize, const N: usize> Dot<Vector<T, N>> for Matrix<T, M, N>
where
    T: Mul<Output = T> + Add<Output = T> + ImplicitClone,
{
    type Output = Vector<T, M>;
    fn dot(self, vec: Vector<T, N>) -> Self::Output {
        Vector::indices().map(|i| self.row(i).dot(vec.clone()))
    }
}

impl<T, const M: usize, const N: usize> Dot<Matrix<T, M, N>> for Vector<T, M>
where
    T: Mul<Output = T> + Add<Output = T> + ImplicitClone,
{
    type Output = Vector<T, N>;
    fn dot(self, mat: Matrix<T, M, N>) -> Self::Output {
        Vector::indices().map(|j| self.clone().dot(mat.col(j)))
    }
}

impl<T, const L: usize, const M: usize, const N: usize> Dot<Matrix<T, M, N>> for Matrix<T, L, M>
where
    T: Mul<Output = T> + Add<Output = T> + ImplicitClone,
{
    type Output = Matrix<T, L, N>;
    fn dot(self, mat: Matrix<T, M, N>) -> Self::Output {
        Matrix::indices().map(|(i, j)| self.row(i).dot(mat.col(j)))
    }
}
