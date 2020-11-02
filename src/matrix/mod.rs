#[macro_use]
mod base;
pub use base::*;

//#[macro_use]
//mod ops;
//pub use ops::*;

//#[macro_use]
//mod transpose;
//pub use transpose::*;

//#[macro_use]
//mod product;
//pub use product::*;

//#[macro_use]
//mod square;
//pub use square::*;

//#[cfg(test)]
//mod tests;


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
use num_traits::{Zero, One, Signed, Float};
use num_integer::{self as nint, Integer};
use crate::{traits::*, vector::*};

pub use crate::traits::{Dot, Outer};


macro_rules! matrix_all { ($M:expr, $N:expr, $W:ident, $V:ident, $U:ident, $GI:ident) => (
	matrix_base!($M, $N, $W, $V, $U, $GI);
	//matrix_ops!($M, $N, $W);
) }

matrix_all!(2, 2, Matrix2x2, Vector2, Vector2, GroupIter2);
//matrix_all!(2, 3, Matrix2x3, Vector2, Vector3, GroupIter3);
//matrix_all!(2, 4, Matrix2x4, Vector2, Vector4, GroupIter4);
//matrix_all!(3, 2, Matrix3x2, Vector3, Vector2, GroupIter2);
//matrix_all!(3, 3, Matrix3x3, Vector3, Vector3, GroupIter3);
//matrix_all!(3, 4, Matrix3x4, Vector3, Vector4, GroupIter4);
//matrix_all!(4, 2, Matrix4x2, Vector4, Vector2, GroupIter2);
//matrix_all!(4, 3, Matrix4x3, Vector4, Vector3, GroupIter3);
//matrix_all!(4, 4, Matrix4x4, Vector4, Vector4, GroupIter4);
//
//matrix_transpose!(2, 2, Matrix2x2, Matrix2x2);
//matrix_transpose!(2, 3, Matrix2x3, Matrix3x2);
//matrix_transpose!(2, 4, Matrix2x4, Matrix4x2);
//matrix_transpose!(3, 2, Matrix3x2, Matrix2x3);
//matrix_transpose!(3, 3, Matrix3x3, Matrix3x3);
//matrix_transpose!(3, 4, Matrix3x4, Matrix4x3);
//matrix_transpose!(4, 2, Matrix4x2, Matrix2x4);
//matrix_transpose!(4, 3, Matrix4x3, Matrix3x4);
//matrix_transpose!(4, 4, Matrix4x4, Matrix4x4);
//
//matrix_product_vec!(2, 2, Matrix2x2, Vector2, Vector2);
//matrix_product_vec!(2, 3, Matrix2x3, Vector2, Vector3);
//matrix_product_vec!(2, 4, Matrix2x4, Vector2, Vector4);
//matrix_product_vec!(3, 2, Matrix3x2, Vector3, Vector2);
//matrix_product_vec!(3, 3, Matrix3x3, Vector3, Vector3);
//matrix_product_vec!(3, 4, Matrix3x4, Vector3, Vector4);
//matrix_product_vec!(4, 2, Matrix4x2, Vector4, Vector2);
//matrix_product_vec!(4, 3, Matrix4x3, Vector4, Vector3);
//matrix_product_vec!(4, 4, Matrix4x4, Vector4, Vector4);
//
//matrix_dot!(2, 2, 2, Matrix2x2, Matrix2x2, Matrix2x2);
//matrix_dot!(2, 2, 3, Matrix2x2, Matrix2x3, Matrix2x3);
//matrix_dot!(2, 2, 4, Matrix2x2, Matrix2x4, Matrix2x4);
//matrix_dot!(2, 3, 2, Matrix2x3, Matrix3x2, Matrix2x2);
//matrix_dot!(2, 3, 3, Matrix2x3, Matrix3x3, Matrix2x3);
//matrix_dot!(2, 3, 4, Matrix2x3, Matrix3x4, Matrix2x4);
//matrix_dot!(2, 4, 2, Matrix2x4, Matrix4x2, Matrix2x2);
//matrix_dot!(2, 4, 3, Matrix2x4, Matrix4x3, Matrix2x3);
//matrix_dot!(2, 4, 4, Matrix2x4, Matrix4x4, Matrix2x4);
//matrix_dot!(3, 2, 2, Matrix3x2, Matrix2x2, Matrix3x2);
//matrix_dot!(3, 2, 3, Matrix3x2, Matrix2x3, Matrix3x3);
//matrix_dot!(3, 2, 4, Matrix3x2, Matrix2x4, Matrix3x4);
//matrix_dot!(3, 3, 2, Matrix3x3, Matrix3x2, Matrix3x2);
//matrix_dot!(3, 3, 3, Matrix3x3, Matrix3x3, Matrix3x3);
//matrix_dot!(3, 3, 4, Matrix3x3, Matrix3x4, Matrix3x4);
//matrix_dot!(3, 4, 2, Matrix3x4, Matrix4x2, Matrix3x2);
//matrix_dot!(3, 4, 3, Matrix3x4, Matrix4x3, Matrix3x3);
//matrix_dot!(3, 4, 4, Matrix3x4, Matrix4x4, Matrix3x4);
//matrix_dot!(4, 2, 2, Matrix4x2, Matrix2x2, Matrix4x2);
//matrix_dot!(4, 2, 3, Matrix4x2, Matrix2x3, Matrix4x3);
//matrix_dot!(4, 2, 4, Matrix4x2, Matrix2x4, Matrix4x4);
//matrix_dot!(4, 3, 2, Matrix4x3, Matrix3x2, Matrix4x2);
//matrix_dot!(4, 3, 3, Matrix4x3, Matrix3x3, Matrix4x3);
//matrix_dot!(4, 3, 4, Matrix4x3, Matrix3x4, Matrix4x4);
//matrix_dot!(4, 4, 2, Matrix4x4, Matrix4x2, Matrix4x2);
//matrix_dot!(4, 4, 3, Matrix4x4, Matrix4x3, Matrix4x3);
//matrix_dot!(4, 4, 4, Matrix4x4, Matrix4x4, Matrix4x4);
