macro_rules! vector_format { ($N:expr, $V:ident) => (
	impl<T> Debug for $V<T> where T: Debug {
		fn fmt(&self, f: &mut Formatter) -> FmtResult {
			write!(f, "{}(", stringify!($V))?;
			for i in 0..$N-1 {
				write!(f, "{:?}, ", self[i])?;
			}
			write!(f, "{:?})", self[$N-1])?;
			Ok(())
		}
    }

    impl<T> Display for $V<T> where T: Display {
		fn fmt(&self, f: &mut Formatter) -> FmtResult {
			write!(f, "{}(", stringify!($V))?;
			for i in 0..$N-1 {
				write!(f, "{}, ", self[i])?;
			}
			write!(f, "{})", self[$N-1])?;
			Ok(())
		}
	}
) }
