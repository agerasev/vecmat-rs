
macro_rules! vector_metrics { ($N:expr, $V:ident) => (
	impl<T> NormL1 for $V<T> where T: Float {
		type Output = T;
		fn norm_l1(self) -> T {
			self.map(|x| x.abs()).sum()
		}
	}
	impl<T> NormL2 for $V<T> where T: Float {
		type Output = T;
		fn norm_l2(self) -> T {
			self.map(|x| x*x).sum().sqrt()
		}
	}
	impl<T> NormLInf for $V<T> where T: Float {
		type Output = T;
		fn norm_l_inf(self) -> T {
			self.map(|x| x.abs()).fold_first(|x, y| x.max(y))
		}
	}
) }

macro_rules! vector_dot { ($N:expr, $V:ident) => (
    vector_metrics!($N, $V);

	impl<T> Dot<$V<T>> for $V<T> where T: Mul<Output=T> + Add<Output=T> {
		type Output = T;
		fn dot(self, other: $V<T>) -> Self::Output {
			self.zip(other).map(|(x, y)| x * y).sum()
		}
	}
	impl<T> $V<T> where T: Add<Output=T> + Mul<Output=T> + Clone {
		pub fn square_length(self) -> T {
			self.map(|x| x.clone()*x).sum()
		}
	}
	impl<T> $V<T> where T: Float + Clone {
		pub fn length(self) -> T {
			self.square_length().sqrt()
		}
		pub fn normalize(self) -> $V<T> {
			self / self.length()
		}
	}
) }
