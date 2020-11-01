
macro_rules! matrix_div_mod_floor {
	($M:expr, $N:expr, $W:ident) => (
		impl<T> $W<T> where T: Integer {
			pub fn div_floor(self, other: $W<T>) -> $W<T> {
				self.zip(other).map(|(x, y)| nint::div_floor(x, y))
			}
			pub fn mod_floor(self, other: $W<T>) -> $W<T> {
				self.zip(other).map(|(x, y)| nint::mod_floor(x, y))
			}
			pub fn div_mod_floor(self, other: $W<T>) -> ($W<T>, $W<T>) {
				self.zip(other).map(|(x, y)| nint::div_mod_floor(x, y)).unzip()
			}
		}
	)
}

macro_rules! matrix_int { ($M:expr, $N:expr, $W:ident) => (
    //matrix_div_rem!($M, $W);
    matrix_div_mod_floor!($M, $N, $W);
) }
