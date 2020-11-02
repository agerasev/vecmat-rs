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
	mem::{self, MaybeUninit},
    ptr,
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
	slice,
	fmt::{Display, Formatter, Result as FmtResult},
};
use num_traits::{Zero, Signed, Float};
use num_integer::{self as nint, Integer};
use crate::{traits::*};

pub use crate::traits::Dot;

macro_rules! vector_all { ($N:expr, $V:ident, $II:ident) => (
	vector_base!($N, $V, $II);
	vector_ops!($N, $V);
	vector_dot!($N, $V);
) }

vector_all!(2, Vector2, IntoIter2);
vector_all!(3, Vector3, IntoIter3);
vector_all!(4, Vector4, IntoIter4);
vector_all!(5, Vector5, IntoIter5);
vector_all!(6, Vector6, IntoIter6);
vector_all!(7, Vector7, IntoIter7);
vector_all!(8, Vector8, IntoIter8);
vector_all!(9, Vector9, IntoIter9);
vector_all!(10, Vector10, IntoIter10);
vector_all!(11, Vector11, IntoIter11);
vector_all!(12, Vector12, IntoIter12);
vector_all!(13, Vector13, IntoIter13);
vector_all!(14, Vector14, IntoIter14);
vector_all!(15, Vector15, IntoIter15);
vector_all!(16, Vector16, IntoIter16);
