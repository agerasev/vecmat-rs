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
