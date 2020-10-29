#[macro_use]
mod base;

#[macro_use]
mod dot;

#[macro_use]
mod int;

#[macro_use]
mod bool_;

#[macro_use]
mod vcmp;


macro_rules! vector_ops { ($N:expr, $V:ident) => (
	vector_ops_base!($N, $V);
	vector_dot!($N, $V);
	vector_int!($N, $V);
	vector_bool!($N, $V);
	vector_vcmp!($N, $V);
) }
