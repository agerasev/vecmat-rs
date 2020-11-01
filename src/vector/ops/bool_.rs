
macro_rules! vector_bool_not { ($N:expr, $V:ident) => (
    impl Not for $V<bool> {
        type Output = $V<bool>;
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

macro_rules! vector_bool_op { ($N:expr, $V:ident, $Trait:ident, $method:ident, $op:ident) => (
    impl $Trait for $V<bool> {
        type Output = $V<bool>;
        fn $method(self, other: $V<bool>) -> Self::Output {
            self.zip(other).map(|(x, y)| $op!(x, y))
        }
    }
) }
macro_rules! vector_bool_op_assign { ($N:expr, $V:ident, $Trait:ident, $method:ident, $op:ident) => (
    impl $Trait for $V<bool> {
        fn $method(&mut self, other: $V<bool>) {
			self.iter_mut().zip(other.into_iter()).for_each(|(s, y)| $op!(*s, y))
        }
    }
) }

macro_rules! vector_bool_any_all { ($N:expr, $V:ident) => (
    impl $V<bool> {
        pub fn any(self) -> bool {
            self.into_iter().any(|x| x)
        }
    }
    impl $V<bool> {
        pub fn all(self) -> bool {
            self.into_iter().all(|x| x)
        }
    }
) }


macro_rules! vector_bool { ($N:expr, $V:ident) => (
    vector_bool_not!($N, $V);

	vector_bool_op!($N, $V, BitAnd, bitand, op_bitand);
	vector_bool_op!($N, $V, BitOr, bitor, op_bitor);
	vector_bool_op!($N, $V, BitXor, bitxor, op_bitxor);

	vector_bool_op_assign!($N, $V, BitAndAssign, bitand_assign, op_bitand_assign);
	vector_bool_op_assign!($N, $V, BitOrAssign, bitor_assign, op_bitor_assign);
	vector_bool_op_assign!($N, $V, BitXorAssign, bitxor_assign, op_bitxor_assign);

	vector_bool_any_all!($N, $V);
) }
