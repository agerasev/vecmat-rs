use num_traits::{Zero, One, Signed};
use crate::{matrix::*};


impl<T, const N: usize> One for Matrix<T, N, N> where T: One + Zero {
	/// Create identity matrix.
	fn one() -> Self {
		Matrix::indices().map(|(i, j)| if i == j { T::one() } else { T::zero() })
	}
}

// FIXME: Use const generics expression and bounds
macro_rules! matrix_submatrix { ($N:expr, $L:expr) => (
	impl<T> Matrix<T, $N, $N> where T: Clone {
		/// Take submatrix from original matrix.
		pub fn submatrix(&self, y: usize, x: usize) -> Matrix<T, $L, $L> {
			Matrix::indices().map(|(i, j)| self[(i + (i >= y) as usize, j + (j >= x) as usize)].clone())
		}
	}
) }
matrix_submatrix!(4, 3);
matrix_submatrix!(3, 2);
impl<T> Matrix<T, 2, 2> where T: Clone {
	/// Take submatrix from original matrix.
	pub fn submatrix(&self, y: usize, x: usize) -> T {
		self[(1 - y, 1 - x)].clone()
	}
}

// FIXME: Use const generics expression and bounds
macro_rules! matrix_cofactor { ($N:expr) => (
	impl<T> Matrix<T, $N, $N> where T: Signed + Clone {
		/// Find a cofactor of the matrix.
		pub fn cofactor(&self, y: usize, x: usize) -> T {
			(if (x + y) % 2 == 0 {
				T::one()
			} else {
				-T::one()
			}) * self.submatrix(y, x).det()
		}
	}
) }
matrix_cofactor!(4);
matrix_cofactor!(3);
impl<T> Matrix<T, 2, 2> where T: Signed + Clone {
	/// Find a cofactor of the matrix.
	pub fn cofactor(&self, y: usize, x: usize) -> T {
		(if (x + y) % 2 == 0 { T::one() } else { -T::one() }) * self.submatrix(y, x)
	}
}

// FIXME: Use const generics expression and bounds
macro_rules! matrix_det { ($N:expr) => (
	impl<T> Matrix<T, $N, $N> where T: Signed + Clone {
		/// Find a determinant of the matrix.
		pub fn det(&self) -> T {
			let i = 0;
			self.row(i).enumerate()
			.map(|(j, x)| x * self.cofactor(i, j))
			.sum()
		}
	}
) }
matrix_det!(4);
matrix_det!(3);
impl<T> Matrix<T, 2, 2> where T: Signed + Clone {
	/// Find a determinant of the matrix.
	pub fn det(&self) -> T {
		self[(0, 0)].clone()*self[(1, 1)].clone() -
		self[(0, 1)].clone()*self[(1, 0)].clone()
	}
}

macro_rules! matrix_adj { ($N:expr) => (
	impl<T> Matrix<T, $N, $N> where T: Signed + Clone {
		/// Find an adjugate matrix.
		pub fn adj(&self) -> Matrix<T, $N, $N> {
			Matrix::indices().map(|(i, j)| self.cofactor(j, i))
		}
	}
) }
matrix_adj!(4);
matrix_adj!(3);
matrix_adj!(2);

macro_rules! matrix_inverse { ($N:expr) => (
	impl<T> Matrix<T, $N, $N> where T: Signed + Clone {
		/// Find an inverse matrix.
		pub fn inverse(&self) -> Matrix<T, $N, $N> {
			self.adj() / self.det()
		}
	}
) }
matrix_inverse!(4);
matrix_inverse!(3);
matrix_inverse!(2);
