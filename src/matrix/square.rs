use crate::matrix::*;


macro_rules! matrix_one { ($N:expr, $W:ident) => (
	impl<T> One for $W<T> where T: One + Zero {
		/// Create identity matrix.
		fn one() -> Self {
			$W::indices().map(|(i, j)| if i == j { T::one() } else { T::zero() })
		}
	}
) }
matrix_one!(2, Matrix2x2);
matrix_one!(3, Matrix3x3);
matrix_one!(4, Matrix4x4);

macro_rules! matrix_submatrix { ($N:expr, $W:ident, $V:ident) => (
	impl<T> $W<T> where T: Clone {
		/// Take submatrix from original matrix.
		pub fn submatrix(&self, y: usize, x: usize) -> $V<T> {
			$V::indices().map(|(i, j)| self[(i + (i >= y) as usize, j + (j >= x) as usize)].clone())
		}
	}
) }
matrix_submatrix!(4, Matrix4x4, Matrix3x3);
matrix_submatrix!(3, Matrix3x3, Matrix2x2);
impl<T> Matrix2x2<T> where T: Clone {
	/// Take submatrix from original matrix.
	pub fn submatrix(&self, y: usize, x: usize) -> T {
		self[(1 - y, 1 - x)].clone()
	}
}

macro_rules! matrix_cofactor { ($N:expr, $W:ident) => (
	impl<T> $W<T> where T: Signed + Clone {
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
matrix_cofactor!(4, Matrix4x4);
matrix_cofactor!(3, Matrix3x3);
impl<T> Matrix2x2<T> where T: Signed + Clone {
	/// Find a cofactor of the matrix.
	pub fn cofactor(&self, y: usize, x: usize) -> T {
		(if (x + y) % 2 == 0 { T::one() } else { -T::one() }) * self.submatrix(y, x)
	}
}

macro_rules! matrix_det { ($N:expr, $W:ident) => (
	impl<T> $W<T> where T: Signed + Clone {
		/// Find a determinant of the matrix.
		pub fn det(&self) -> T {
			let i = 0;
			self.row(i).enumerate()
			.map(|(j, x)| x * self.cofactor(i, j))
			.sum()
		}
	}
) }
matrix_det!(4, Matrix4x4);
matrix_det!(3, Matrix3x3);
impl<T> Matrix2x2<T> where T: Signed + Clone {
	/// Find a determinant of the matrix.
	pub fn det(&self) -> T {
		self[(0, 0)].clone()*self[(1, 1)].clone() -
		self[(0, 1)].clone()*self[(1, 0)].clone()
	}
}

macro_rules! matrix_adj { ($N:expr, $W:ident) => (
	impl<T> $W<T> where T: Signed + Clone {
		/// Find an adjugate matrix.
		pub fn adj(&self) -> $W<T> {
			$W::indices().map(|(i, j)| self.cofactor(j, i))
		}
	}
) }
matrix_adj!(4, Matrix4x4);
matrix_adj!(3, Matrix3x3);
matrix_adj!(2, Matrix2x2);

macro_rules! matrix_inverse { ($N:expr, $W:ident) => (
	impl<T> $W<T> where T: Signed + Clone {
		/// Find an inverse matrix.
		pub fn inverse(&self) -> $W<T> {
			self.adj() / self.det()
		}
	}
) }
matrix_inverse!(4, Matrix4x4);
matrix_inverse!(3, Matrix3x3);
matrix_inverse!(2, Matrix2x2);
