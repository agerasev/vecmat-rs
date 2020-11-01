
macro_rules! matrix_bool_not { ($M:expr, $N:expr, $W:ident) => (
    impl Not for $W<bool> {
        type Output = $W<bool>;
        fn not(self) -> Self::Output {
            self.map(|x| !x)
        }
    }
) }

macro_rules! op_bitand { ($a:expr, $b:expr) => ({ $a & $b }) }
macro_rules! op_bitor  { ($a:expr, $b:expr) => ({ $a | $b }) }
macro_rules! op_bitxor { ($a:expr, $b:expr) => ({ $a ^ $b }) }

macro_rules! op_bitand_assign { ($a:expr, $b:expr) => ({ $a &= $b }) }
macro_rules! op_bitor_assign  { ($a:expr, $b:expr) => ({ $a |= $b }) }
macro_rules! op_bitxor_assign { ($a:expr, $b:expr) => ({ $a ^= $b }) }

macro_rules! matrix_bool_op { ($M:expr, $N:expr, $W:ident, $Trait:ident, $method:ident, $op:ident) => (
    impl $Trait for $W<bool> {
        type Output = $W<bool>;
        fn $method(self, other: $W<bool>) -> Self::Output {
            self.zip(other).map(|(x, y)| $op!(x, y))
        }
    }
) }
macro_rules! matrix_bool_op_assign { ($M:expr, $N:expr, $W:ident, $Trait:ident, $method:ident, $op:ident) => (
    impl $Trait for $W<bool> {
        fn $method(&mut self, other: $W<bool>) {
			self.iter_mut().zip(other.into_iter()).for_each(|(s, y)| $op!(*s, y))
        }
    }
) }

macro_rules! matrix_bool_any_all { ($M:expr, $N:expr, $W:ident) => (
    impl $W<bool> {
        pub fn any(self) -> bool {
            self.into_iter().any(|x| x)
        }
    }
    impl $W<bool> {
        pub fn all(self) -> bool {
            self.into_iter().all(|x| x)
        }
    }
) }


macro_rules! matrix_bool { ($M:expr, $N:expr, $W:ident) => (
    matrix_bool_not!($M, $N, $W);

	matrix_bool_op!($M, $N, $W, BitAnd, bitand, op_bitand);
	matrix_bool_op!($M, $N, $W, BitOr, bitor, op_bitor);
	matrix_bool_op!($M, $N, $W, BitXor, bitxor, op_bitxor);

	matrix_bool_op_assign!($M, $N, $W, BitAndAssign, bitand_assign, op_bitand_assign);
	matrix_bool_op_assign!($M, $N, $W, BitOrAssign, bitor_assign, op_bitor_assign);
	matrix_bool_op_assign!($M, $N, $W, BitXorAssign, bitxor_assign, op_bitxor_assign);

	matrix_bool_any_all!($M, $N, $W);
) }
