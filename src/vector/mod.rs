#[macro_use]
mod base;
pub use base::*;

#[macro_use]
mod ops;
pub use ops::*;

#[macro_use]
mod dot;
pub use dot::*;

#[macro_use]
mod spec;
pub use spec::*;

#[cfg(test)]
mod tests;


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

macro_rules! vector_all { ($N:expr, $V:ident, $A:ident) => (
	vector_base!($N, $V, $A);
	vector_ops!($N, $V);
	vector_dot!($N, $V);
) }

vector_all!(2, Vector2, Array2Ext);
vector_all!(3, Vector3, Array3Ext);
vector_all!(4, Vector4, Array4Ext);
vector_all!(5, Vector5, Array5Ext);
vector_all!(6, Vector6, Array6Ext);
vector_all!(7, Vector7, Array7Ext);
vector_all!(8, Vector8, Array8Ext);
vector_all!(9, Vector9, Array9Ext);
vector_all!(10, Vector10, Array10Ext);
vector_all!(11, Vector11, Array11Ext);
vector_all!(12, Vector12, Array12Ext);
vector_all!(13, Vector13, Array13Ext);
vector_all!(14, Vector14, Array14Ext);
vector_all!(15, Vector15, Array15Ext);
vector_all!(16, Vector16, Array16Ext);
