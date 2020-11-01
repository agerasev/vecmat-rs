
macro_rules! matrix_neg { ($M:expr, $N:expr, $W:ident) => (
	impl<T> Neg for $W<T> where T: Neg<Output=T> {
		type Output = $W<T>;
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

macro_rules! matrix_op_vec { ($M:expr, $N:expr, $W:ident, $Trait:ident, $method:ident, $op:ident) => (
	impl<T> $Trait for $W<T> where T: $Trait<Output=T> {
		type Output = $W<T>;
		fn $method(self, vec: $W<T>) -> Self::Output {
			self.zip(vec).map(|(x, y)| $op!(x, y))
		}
	}
) }
macro_rules! matrix_op_scal { ($M:expr, $N:expr, $W:ident, $Trait:ident, $method:ident, $op:ident) => (
	impl<T> $Trait<T> for $W<T> where T: $Trait<Output=T> + Clone {
		type Output = $W<T>;
		fn $method(self, a: T) -> Self::Output {
			self.map(|v| $op!(v, a.clone()))
		}
	}
) }
macro_rules! matrix_op_vec_assign { ($M:expr, $N:expr, $W:ident, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
	impl<T> $Trait for $W<T> where T: $Trait {
		fn $method(&mut self, vec: $W<T>) {
			self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| { $op!(*s, x); })
		}
	}
) }
macro_rules! matrix_op_scal_assign { ($M:expr, $N:expr, $W:ident, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
	impl<T> $Trait<T> for $W<T> where T: $Trait + Clone {
		fn $method(&mut self, a: T) {
			self.iter_mut().for_each(|s| { $op!(*s, a.clone()); })
		}
	}
) }

macro_rules! matrix_ops_all { ($M:expr, $N:expr, $W:ident, $Trait:ident, $method:ident, $op:ident) => (
	matrix_op_vec!($M, $N, $W, $Trait, $method, $op);
	matrix_op_scal!($M, $N, $W, $Trait, $method, $op);
) }
macro_rules! matrix_ops_all_assign { ($M:expr, $N:expr, $W:ident, $Trait:ident, $BaseTrait:ident, $method:ident, $op:ident) => (
	matrix_op_vec_assign!($M, $N, $W, $Trait, $BaseTrait, $method, $op);
	matrix_op_scal_assign!($M, $N, $W, $Trait, $BaseTrait, $method, $op);
) }

macro_rules! matrix_zero { ($M:expr, $N:expr, $W:ident) => (
	impl<T> Zero for $W<T> where T: Zero {
		fn zero() -> Self {
			Self::init(|| T::zero())
		}
		fn is_zero(&self) -> bool {
			self.iter().all(|x| x.is_zero())
		}
	}
	impl<T> $W<T> where T: PartialOrd + Zero + Neg<Output=T> {
		fn abs(self) -> Self {
			self.map(|x| if x < T::zero() { -x } else { x })
		}
	}
) }

macro_rules! matrix_reduce { ($M:expr, $N:expr, $W:ident) => (
	impl<T> $W<T> {
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

macro_rules! matrix_norm { ($M:expr, $N:expr, $W:ident) => (
	impl<T> NormL1 for $W<T> where T: Signed + PartialOrd {
		type Output = T;
		fn norm_l1(self) -> T {
			self.abs().sum()
		}
	}
	impl<T> NormL2 for $W<T> where T: Float {
		type Output = T;
		fn norm_l2(self) -> T {
			self.map(|x| x*x).sum().sqrt()
		}
	}
	impl<T> NormLInf for $W<T> where T: Signed + PartialOrd {
		type Output = T;
		fn norm_l_inf(self) -> T {
			self.abs().max()
		}
	}
) }

macro_rules! matrix_ops_base { ($M:expr, $N:expr, $W:ident) => (
	matrix_neg!($M, $N, $W);

	matrix_op_vec!($M, $N, $W, Add, add, op_add);
	matrix_op_vec!($M, $N, $W, Sub, sub, op_sub);
	matrix_ops_all!($M, $N, $W, Mul, mul, op_mul);
	matrix_ops_all!($M, $N, $W, Div, div, op_div);
	matrix_ops_all!($M, $N, $W, Rem, rem, op_rem);

	matrix_op_vec_assign!($M, $N, $W, AddAssign, Add, add_assign, op_add_assign);
	matrix_op_vec_assign!($M, $N, $W, SubAssign, Sub, sub_assign, op_sub_assign);
	matrix_ops_all_assign!($M, $N, $W, MulAssign, Mul, mul_assign, op_mul_assign);
	matrix_ops_all_assign!($M, $N, $W, DivAssign, Div, div_assign, op_div_assign);
	matrix_ops_all_assign!($M, $N, $W, RemAssign, Rem, rem_assign, op_rem_assign);

	matrix_zero!($M, $N, $W);
	matrix_reduce!($M, $N, $W);
	matrix_norm!($M, $N, $W);
) }
