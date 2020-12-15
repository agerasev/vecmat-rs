
macro_rules! matrix_metrics { ($M:expr, $N:expr, $W:ident) => (
	impl<T> NormL1 for $W<T> where T: Float {
		type Output = T;
		fn norm_l1(self) -> T {
			self.map(|x| x.abs()).sum()
		}
	}
	impl<T> NormL2 for $W<T> where T: Float {
		type Output = T;
		fn norm_l2(self) -> T {
			self.map(|x| x*x).sum().sqrt()
		}
	}
	impl<T> NormLInf for $W<T> where T: Float {
		type Output = T;
		fn norm_l_inf(self) -> T {
			self.map(|x| x.abs()).fold_first(|x, y| x.max(y))
		}
	}
) }

macro_rules! matrix_dot { ($M:expr, $N:expr, $W:ident) => (
    matrix_metrics!($M, $N, $W);

	impl<T> Dot<$W<T>> for $W<T> where T: Mul<Output=T> + Add<Output=T> {
		type Output = T;
		fn dot(self, other: $W<T>) -> Self::Output {
			self.zip(other).map(|(x, y)| x * y).sum()
		}
	}
	impl<T> $W<T> where T: Add<Output=T> + Mul<Output=T> + Clone {
		pub fn square_length(self) -> T {
			self.map(|x| x.clone()*x).sum()
		}
	}
	impl<T> $W<T> where T: Float + Clone {
		pub fn length(self) -> T {
			self.square_length().sqrt()
		}
		pub fn normalize(self) -> $W<T> {
			self / self.length()
		}
	}
) }
