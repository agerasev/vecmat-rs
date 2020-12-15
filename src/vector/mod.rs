mod base;
//mod ops;
//mod dot;
//mod spec;

//#[cfg(feature = "random")]
//mod random;

//#[cfg(feature = "approx")]
//mod approx;

//#[cfg(test)]
//mod tests;

pub use base::*;
//pub use ops::*;
//pub use dot::*;
//pub use spec::*;

//#[cfg(feature = "random")]
//pub use random::*;

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;
pub type Vector8<T> = Vector<T, 8>;
pub type Vector16<T> = Vector<T, 16>;

/*
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

#[macro_use]
#[cfg(feature = "random")]
mod random;

#[macro_use]
#[cfg(feature = "approx")]
mod approx;

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
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
#[cfg(feature = "random")]
use core::marker::PhantomData;

use num_traits::{Zero, Float};
use num_integer::{self as nint, Integer};
use crate::{traits::*, };
#[cfg(feature = "approx")]
use ::approx::{AbsDiffEq, abs_diff_eq};
#[cfg(feature = "random")]
use rand::Rng;
#[cfg(feature = "random")]
use rand_distr::Distribution;
#[cfg(feature = "random")]
use crate::distributions::*;

pub use crate::traits::Dot;


macro_rules! vector_all { ($N:expr, $V:ident, $II:ident, $GI:ident, $D:ident) => (
    vector_base!($N, $V, $II, $GI);
    vector_ops!($N, $V);
    vector_dot!($N, $V);
    #[cfg(feature = "random")]
    vector_random!($N, $V, $D);
    #[cfg(feature = "approx")]
    vector_approx!($N, $V);
) }

vector_all!(2, Vector2, IntoIter2, GroupIter2, VectorDistribution2);
vector_all!(3, Vector3, IntoIter3, GroupIter3, VectorDistribution3);
vector_all!(4, Vector4, IntoIter4, GroupIter4, VectorDistribution4);
vector_all!(5, Vector5, IntoIter5, GroupIter5, VectorDistribution5);
vector_all!(6, Vector6, IntoIter6, GroupIter6, VectorDistribution6);
vector_all!(7, Vector7, IntoIter7, GroupIter7, VectorDistribution7);
vector_all!(8, Vector8, IntoIter8, GroupIter8, VectorDistribution8);
vector_all!(9, Vector9, IntoIter9, GroupIter9, VectorDistribution9);
vector_all!(10, Vector10, IntoIter10, GroupIter10, VectorDistribution10);
vector_all!(11, Vector11, IntoIter11, GroupIter11, VectorDistribution11);
vector_all!(12, Vector12, IntoIter12, GroupIter12, VectorDistribution12);
vector_all!(13, Vector13, IntoIter13, GroupIter13, VectorDistribution13);
vector_all!(14, Vector14, IntoIter14, GroupIter14, VectorDistribution14);
vector_all!(15, Vector15, IntoIter15, GroupIter15, VectorDistribution15);
vector_all!(16, Vector16, IntoIter16, GroupIter16, VectorDistribution16);
*/
