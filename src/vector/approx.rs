
macro_rules! vector_approx { ($N:expr, $V:ident) => (
	#[cfg(test)]
	impl<T> AbsDiffEq for $V<T> where T: AbsDiffEq<Epsilon=T> + Clone {
		type Epsilon = T;
		fn default_epsilon() -> Self::Epsilon {
			T::default_epsilon()
		}
		fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
			self.clone().zip(other.clone()).map(|(x, y)| abs_diff_eq!(x, y, epsilon=epsilon.clone())).all()
		}
	}
) }
