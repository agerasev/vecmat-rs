#[macro_use]
mod init;
pub use init::*;

#[macro_use]
mod iter;
pub use iter::*;

#[macro_use]
mod format;
pub use format::*;

#[cfg(all(test, feature = "std"))]
mod tests;


macro_rules! vector_base { ($N:expr, $V:ident, $II:ident, $GI:ident) => (
	vector_init!($N, $V);
	vector_iter!($N, $V, $II, $GI);
	vector_format!($N, $V);
) }
