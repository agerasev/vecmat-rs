
macro_rules! mat_one {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + One + Zero {
			pub fn one() -> Self {
				$V::from_map(|i, j| if i == j { T::one() } else { T::zero() })
			}
		}

		impl<T> One for $V<T> where T: Copy + Num + One + Zero {
			fn one() -> Self {
				$V::one()
			}
		}
	)
}

mat_one!(Mat2x2, 2);
mat_one!(Mat3x3, 3);
mat_one!(Mat4x4, 4);

macro_rules! mat_submatrix {
	($Vs:ident, $Vr:ident, $N:expr) => (
		impl<T> $Vs<T> where T: Copy {
			pub fn submatrix(self, x: usize, y: usize) -> $Vr<T> {
				$Vr::from_map(|i, j| self[(i + (i >= x) as usize, j + (j >= y) as usize)])
			}
		}
	)
}

impl<T> Mat2x2<T> where T: Copy {
	pub fn submatrix(self, x:usize, y:usize) -> T {
		self[(1 - x, 1 - y)]
	}
}

mat_submatrix!(Mat4x4, Mat3x3, 4);
mat_submatrix!(Mat3x3, Mat2x2, 3);

macro_rules! mat_cofactor {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Signed {
			pub fn cofactor(self, x: usize, y: usize) -> T {
				(if (x + y) % 2 == 0 { T::one() } else { -T::one() })*self.submatrix(x,y).det()
			}
		}
	)
}

impl<T> Mat2x2<T> where T: Copy + Num + Signed {
	pub fn cofactor(self, x: usize, y: usize) -> T {
		(if (x + y) % 2 == 0 { T::one() } else { -T::one() })*self.submatrix(x,y)
	}
}

mat_cofactor!(Mat4x4, 4);
mat_cofactor!(Mat3x3, 3);

/// Determinant
macro_rules! mat_det {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Signed {
			pub fn det(self) -> T {
				let mut tmp = T::zero();
				let j = 0;
				for i in 0..$N {
					tmp = tmp + self[(i, j)]*self.cofactor(i, j);
				}
				tmp
			}
		}
	)
}

impl<T> Mat2x2<T> where T: Copy + Num + Signed {
	pub fn det(self) -> T {
		self[(0, 0)]*self[(1, 1)] - self[(1, 0)]*self[(0, 1)]
	}
}

mat_det!(Mat4x4, 4);
mat_det!(Mat3x3, 3);

/// Adjugate matrix
macro_rules! mat_adj {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Signed {
			pub fn adj(self) -> $V<T> {
				$V::from_map(|i, j| self.cofactor(j, i))
			}
		}
	)
}

mat_adj!(Mat4x4, 4);
mat_adj!(Mat3x3, 3);
mat_adj!(Mat2x2, 2);

/// Inverse matrix
macro_rules! mat_inverse {
	($V:ident, $N:expr) => (
		impl<T> $V<T> where T: Copy + Num + Signed {
			pub fn inverse(self) -> $V<T> {
				self.adj()/self.det()
			}
		}
	)
}

mat_inverse!(Mat4x4, 4);
mat_inverse!(Mat3x3, 3);
mat_inverse!(Mat2x2, 2);

macro_rules! mat_mul_scal_rev {
	($V:ident, $T:ident) => (
		impl Mul<$V<$T>> for $T {
			type Output = $V<$T>;
			fn mul(self, a: $V<$T>) -> Self::Output {
				a*self
			}
		}
	)
}
