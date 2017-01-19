#[allow(unused_imports)]
use mat::*;

macro_rules! vnm_arr {
	[$i:ident, $j:ident; $v:expr; $N:expr, $M:expr] => ({
		let ($i, $j) = (0, 0);
		let mut arr = [$v; $N*$M];
		for $j in 0..$M {
			for $i in 0..$N {
				arr[$i + $j*$N] = $v;
			}
		}
		arr
	})
}

macro_rules! vnm_content_test {
	($V:ident, $N:expr, $M:expr) => (
		let mut m = $V { d: vnm_arr![i, j; i + j; $N, $M] };
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m.d[i + $N*j], i + j);
			}
		}

		let z = $V { d: [0; $N*$M] };
		for i in 0..($N*$M) {
			assert_eq!(z.d[i], 0);
		}

		for i in 0..($N*$M) {
			m.d[i] = i + 2;
		}
		for i in 0..($N*$M) {
			assert_eq!(m.d[i], i + 2);
		}
	)
}

#[test]
fn content() {
	vnm_content_test!(mat2x2, 2, 2);
	vnm_content_test!(mat2x3, 2, 3);
	vnm_content_test!(mat2x4, 2, 4);
	vnm_content_test!(mat3x2, 3, 2);
	vnm_content_test!(mat3x3, 3, 3);
	vnm_content_test!(mat3x4, 3, 4);
	vnm_content_test!(mat4x2, 4, 2);
	vnm_content_test!(mat4x3, 4, 3);
	vnm_content_test!(mat4x4, 4, 4);
}

macro_rules! vnm_eq_test {
	($V:ident, $N:expr, $M:expr) => (
		let va = $V::<usize> { d: vnm_arr![i, j; i + j; $N, $M] };
		let vb = $V::<usize> { d: vnm_arr![i, j; i + j; $N, $M] };
		assert_eq!(va, vb);
	)
}

#[test]
fn eq() {
	vnm_eq_test!(mat2x2, 2, 2);
	vnm_eq_test!(mat2x3, 2, 3);
	vnm_eq_test!(mat2x4, 2, 4);
	vnm_eq_test!(mat3x2, 3, 2);
	vnm_eq_test!(mat3x3, 3, 3);
	vnm_eq_test!(mat3x4, 3, 4);
	vnm_eq_test!(mat4x2, 4, 2);
	vnm_eq_test!(mat4x3, 4, 3);
	vnm_eq_test!(mat4x4, 4, 4);
}

macro_rules! vnm_copy_test {
	($V:ident, $N:expr, $M:expr) => (
		let m = $V::<usize> { d: vnm_arr![i, j; i + j; $N, $M] };
		let cm = m;
		assert_eq!(m, cm);
	)
}

#[test]
fn copy() {
	vnm_copy_test!(mat2x2, 2, 2);
	vnm_copy_test!(mat2x3, 2, 3);
	vnm_copy_test!(mat2x4, 2, 4);
	vnm_copy_test!(mat3x2, 3, 2);
	vnm_copy_test!(mat3x3, 3, 3);
	vnm_copy_test!(mat3x4, 3, 4);
	vnm_copy_test!(mat4x2, 4, 2);
	vnm_copy_test!(mat4x3, 4, 3);
	vnm_copy_test!(mat4x4, 4, 4);
}

macro_rules! vnm_index_test {
	($V:ident, $N:expr, $M:expr) => (
		let mut m = $V::<usize> { d: vnm_arr![i, j; i + j; $N, $M] };
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m[(i, j)], i + j);
			}
		}
		for j in 0..$M {
			for i in 0..$N {
				m[(i, j)] = i*j;
			}
		}
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m[(i, j)], i*j);
			}
		}
	)
}

#[test]
fn index() {
	vnm_index_test!(mat2x2, 2, 2);
	vnm_index_test!(mat2x3, 2, 3);
	vnm_index_test!(mat2x4, 2, 4);
	vnm_index_test!(mat3x2, 3, 2);
	vnm_index_test!(mat3x3, 3, 3);
	vnm_index_test!(mat3x4, 3, 4);
	vnm_index_test!(mat4x2, 4, 2);
	vnm_index_test!(mat4x3, 4, 3);
	vnm_index_test!(mat4x4, 4, 4);
}

/*
macro_rules! vnm_zero_test {
	($V:ident, $N:expr) => (
		let z = $V::<i32>::zero();
		for i in 0..($N*$N) {
			assert_eq!(z.d[i], 0);
		}
		assert!(z.is_zero());
		
		let nz = $V::<i32> { d: [1; ($N*$N)] };
		assert!(!nz.is_zero());
	)
}

#[test]
fn zero() {
	vnm_zero_test!(mat2, 2);
	vnm_zero_test!(mat3, 3);
	vnm_zero_test!(mat4, 4);
}

macro_rules! vnm_one_test {
	($V:ident, $N:expr) => (
		let o = $V::<i32>::one();
		for j in 0..$N {
			for i in 0..$N {
				assert_eq!(o.d[i], if i == j { 1 } else { 0 });
			}
		}
	)
}

#[test]
fn one() {
	vnm_one_test!(mat2, 2);
	vnm_one_test!(mat3, 3);
	vnm_one_test!(mat4, 4);
}
*/

macro_rules! vnm_new_test {
	($V:ident, $N:expr, $M:expr) => (
		$V::<i32>::new();
	)
}

#[test]
fn new() {
	vnm_new_test!(mat2x2, 2, 2);
	vnm_new_test!(mat2x3, 2, 3);
	vnm_new_test!(mat2x4, 2, 4);
	vnm_new_test!(mat3x2, 3, 2);
	vnm_new_test!(mat3x3, 3, 3);
	vnm_new_test!(mat3x4, 3, 4);
	vnm_new_test!(mat4x2, 4, 2);
	vnm_new_test!(mat4x3, 4, 3);
	vnm_new_test!(mat4x4, 4, 4);
}


macro_rules! vnm_from_test {
	($V:ident, $N:expr, $M:expr) => (
		let mf: $V<usize> = $V::from(vnm_arr![i, j; i + j; $N, $M]);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(mf[(i, j)], i + j);
			}
		}

		let mi: $V<usize> = vnm_arr![i, j; i*j; $N, $M].into();
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(mi[(i, j)], i*j);
			}
		}
	)
}

#[test]
fn from() {
	vnm_from_test!(mat2x2, 2, 2);
	vnm_from_test!(mat2x3, 2, 3);
	vnm_from_test!(mat2x4, 2, 4);
	vnm_from_test!(mat3x2, 3, 2);
	vnm_from_test!(mat3x3, 3, 3);
	vnm_from_test!(mat3x4, 3, 4);
	vnm_from_test!(mat4x2, 4, 2);
	vnm_from_test!(mat4x3, 4, 3);
	vnm_from_test!(mat4x4, 4, 4);
}

macro_rules! vnm_neg_test {
	($V:ident, $N:expr, $M:expr) => (
		let m: $V<i32> = vnm_arr![i, j; (i + j + 1) as i32; $N, $M].into();
		let nm = -m;
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(-m[(i, j)], nm[(i, j)]);
			}
		}
	)
}

#[test]
fn neg() {
	vnm_neg_test!(mat2x2, 2, 2);
	vnm_neg_test!(mat2x3, 2, 3);
	vnm_neg_test!(mat2x4, 2, 4);
	vnm_neg_test!(mat3x2, 3, 2);
	vnm_neg_test!(mat3x3, 3, 3);
	vnm_neg_test!(mat3x4, 3, 4);
	vnm_neg_test!(mat4x2, 4, 2);
	vnm_neg_test!(mat4x3, 4, 3);
	vnm_neg_test!(mat4x4, 4, 4);
}