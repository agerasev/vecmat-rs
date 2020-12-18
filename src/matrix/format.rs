
impl<T, const M: usize, const N: usize> Display for Matrix<T, M, N> where T: Display {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		writeln!(f)?;
		writeln!(f, "Matrix{}x{}(", M, N)?;
		for j in 0..M {
			write!(f, "  ")?;
			for i in 0..N {
				write!(f, "{}, ", self[(j, i)])?;
			}
			writeln!(f)?;
		}
		writeln!(f, ")")?;
		Ok(())
	}
}
