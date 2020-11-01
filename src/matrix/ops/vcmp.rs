
macro_rules! matrix_vcmp { ($M:expr, $N:expr, $V:ident) => (
	impl<T> $V<T> where T: PartialEq {
		pub fn veq(self, other: $V<T>) -> $V<bool> {
			self.zip(other).map(|(x, y)| x == y)
		}
		pub fn vne(self, other: $V<T>) -> $V<bool> {
			self.zip(other).map(|(x, y)| x != y)
		}
	}
	impl<T> $V<T> where T: PartialOrd {
		pub fn vlt(self, other: $V<T>) -> $V<bool> {
			self.zip(other).map(|(x, y)| x < y)
		}
		pub fn vle(self, other: $V<T>) -> $V<bool> {
			self.zip(other).map(|(x, y)| x <= y)
		}
		pub fn vgt(self, other: $V<T>) -> $V<bool> {
			self.zip(other).map(|(x, y)| x > y)
		}
		pub fn vge(self, other: $V<T>) -> $V<bool> {
			self.zip(other).map(|(x, y)| x >= y)
		}
	}
) }
