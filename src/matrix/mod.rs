#[macro_use]
mod base;
pub use base::*;

//#[cfg(test)]
//mod tests;


use core::{
	convert::{TryFrom, TryInto},
	ops::{
		Index, IndexMut,
		Neg, Add, Sub, Mul, Div, Rem,
		AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
		Not, BitAnd, BitOr, BitXor,
		BitAndAssign, BitOrAssign, BitXorAssign,
	},
	cmp::{PartialOrd},
	iter::{IntoIterator},
	array::{TryFromSliceError},
	slice,
	fmt::{Display, Formatter, Result as FmtResult},
};
use num_traits::{Zero, Float};
use num_integer::{self as nint, Integer};
use crate::{traits::*, array::*};

pub use crate::traits::Dot;


macro_rules! matrix_all { ($M:expr, $N:expr, $V:ident, $A:ident) => (
	matrix_base!($M, $N, $V, $A);
	//matrix_ops!($N, $V);
) }

matrix_all!(2, 2, Matrix2x2, Array4Ext);
