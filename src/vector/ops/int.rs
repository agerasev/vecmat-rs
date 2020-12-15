
macro_rules! vector_div_mod_floor {
	($N:expr, $V:ident) => (
		impl<T> $V<T> where T: Integer {
			pub fn div_floor(self, other: $V<T>) -> $V<T> {
				self.zip(other).map(|(x, y)| nint::div_floor(x, y))
			}
			pub fn mod_floor(self, other: $V<T>) -> $V<T> {
				self.zip(other).map(|(x, y)| nint::mod_floor(x, y))
			}
			pub fn div_mod_floor(self, other: $V<T>) -> ($V<T>, $V<T>) {
				self.zip(other).map(|(x, y)| nint::div_mod_floor(x, y)).unzip()
			}
		}
	)
}


macro_rules! vector_int { ($N:expr, $V:ident) => (
    //vector_div_rem!($N, $V);
    vector_div_mod_floor!($N, $V);
) }
