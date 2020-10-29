#[macro_use]
mod base;
pub use base::*;

#[macro_use]
mod ops;
pub use ops::*;

#[macro_use]
mod spec;
pub use spec::*;


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
use num_traits::{Zero, One, Float};
use num_integer::{self as nint, Integer};
use crate::{traits::*, array::*};


macro_rules! vector_all { ($N:expr, $V:ident, $A:ident) => (
	vector_base!($N, $V, $A);
	vector_ops!($N, $V);
) }

vector_all!(2, Vector2, Array2Ext);
vector_all!(3, Vector3, Array3Ext);
vector_all!(4, Vector4, Array4Ext);


#[cfg(test)]
#[test]
fn dummy_test() {
    Vector3::<f64>::new();
}


//#[cfg(test)]
//mod tests;
