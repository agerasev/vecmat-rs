#[macro_use]
mod init;
pub use init::*;

#[macro_use]
mod iter;
pub use iter::*;

#[cfg(test)]
mod tests;


macro_rules! vector_display { ($N:expr, $V:ident) => (
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


macro_rules! vector_base { ($N:expr, $V:ident, $II:ident) => (
	vector_init!($N, $V);
	vector_iter!($N, $V, $II);
	vector_display!($N, $V);
) }
