#[macro_use]
mod base;

#[macro_use]
mod int;

#[macro_use]
mod bool_;

#[macro_use]
mod vcmp;


macro_rules! matrix_ops { ($M:expr, $N:expr, $W:ident) => (
	matrix_ops_base!($M, $N, $W);
	matrix_int!($M, $N, $W);
	matrix_bool!($M, $N, $W);
	matrix_vcmp!($M, $N, $W);
) }
