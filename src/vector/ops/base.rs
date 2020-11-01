use core::ops::{Mul};
use crate::vector::*;


macro_rules! vector_neg { ($N:expr, $V:ident) => (
	impl<T> Neg for $V<T> where T: Neg<Output=T> {
		type Output = $V<T>;
		fn neg(self) -> Self::Output {
			self.map(|v| -v)
		}
	}
) }

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a * $b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a / $b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a % $b }) }

macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! vector_op_vec { ($N:expr, $V:ident, $Trait:ident, $method:ident, $op:ident) => (
	impl<T> $Trait for $V<T> where T: $Trait<Output=T> {
		type Output = $V<T>;
		fn $method(self, vec: $V<T>) -> Self::Output {
			self.zip(vec).map(|(x, y)| $op!(x, y))
		}
	}
) }
macro_rules! vector_op_scal { ($N:expr, $V:ident, $Trait:ident, $method:ident, $op:ident) => (
	impl<T> $Trait<T> for $V<T> where T: $Trait<Output=T> + Clone {
		type Output = $V<T>;
		fn $method(self, a: T) -> Self::Output {
			self.map(|v| $op!(v, a.clone()))
		}
	}
) }
macro_rules! vector_op_vec_assign { ($N:expr, $V:ident, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
	impl<T> $Trait for $V<T> where T: $Trait {
		fn $method(&mut self, vec: $V<T>) {
			self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| { $op!(*s, x); })
		}
	}
) }
macro_rules! vector_op_scal_assign { ($N:expr, $V:ident, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
	impl<T> $Trait<T> for $V<T> where T: $Trait + Clone {
		fn $method(&mut self, a: T) {
			self.iter_mut().for_each(|s| { $op!(*s, a.clone()); })
		}
	}
) }

macro_rules! vector_ops_all { ($N:expr, $V:ident, $Trait:ident, $method:ident, $op:ident) => (
	vector_op_vec!($N, $V, $Trait, $method, $op);
	vector_op_scal!($N, $V, $Trait, $method, $op);
) }
macro_rules! vector_ops_all_assign { ($N:expr, $V:ident, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
	vector_op_vec_assign!($N, $V, $Trait, $BaseTrait, $method, $op);
	vector_op_scal_assign!($N, $V, $Trait, $BaseTrait, $method, $op);
) }

macro_rules! vector_zero { ($N:expr, $V:ident) => (
	impl<T> Zero for $V<T> where T: Zero {
		fn zero() -> Self {
			Self::init(|| T::zero())
		}
		fn is_zero(&self) -> bool {
			self.iter().all(|x| x.is_zero())
		}
	}
) }

macro_rules! vector_reduce { ($N:expr, $V:ident) => (
	impl<T> $V<T> {
		pub fn sum(self) -> T where T: Add<Output=T> {
			self.fold_first(|x, y| x + y)
		}
		pub fn max(self) -> T where T: PartialOrd {
			self.fold_first(|x, y| if x < y { y } else { x })
		}
		pub fn min(self) -> T where T: PartialOrd {
			self.fold_first(|x, y| if x < y { x } else { y })
		}
	}
) }

macro_rules! vector_ops_base { ($N:expr, $V:ident) => (
	vector_neg!($N, $V);

	vector_op_vec!($N, $V, Add, add, op_add);
	vector_op_vec!($N, $V, Sub, sub, op_sub);
	vector_ops_all!($N, $V, Mul, mul, op_mul);
	vector_ops_all!($N, $V, Div, div, op_div);
	vector_ops_all!($N, $V, Rem, rem, op_rem);

	vector_op_vec_assign!($N, $V, AddAssign, Add, add_assign, op_add_assign);
	vector_op_vec_assign!($N, $V, SubAssign, Sub, sub_assign, op_sub_assign);
	vector_ops_all_assign!($N, $V, MulAssign, Mul, mul_assign, op_mul_assign);
	vector_ops_all_assign!($N, $V, DivAssign, Div, div_assign, op_div_assign);
	vector_ops_all_assign!($N, $V, RemAssign, Rem, rem_assign, op_rem_assign);

	vector_zero!($N, $V);
	vector_reduce!($N, $V);
) }
