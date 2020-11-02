use std::convert::TryFrom;
use num_traits::{Zero, One};
use crate::{matrix::*, vector::*};


macro_rules! mat_new_test {
	($M:expr, $N:expr, $W:ident) => (
		$W::<i32>::new();
	)
}
#[test]
fn new() {
	mat_new_test!(2, 2, Matrix2x2);
	mat_new_test!(2, 3, Matrix2x3);
	mat_new_test!(2, 4, Matrix2x4);
	mat_new_test!(3, 2, Matrix3x2);
	mat_new_test!(3, 3, Matrix3x3);
	mat_new_test!(3, 4, Matrix3x4);
	mat_new_test!(4, 2, Matrix4x2);
	mat_new_test!(4, 3, Matrix4x3);
	mat_new_test!(4, 4, Matrix4x4);
}
#[test]
fn new_no_gen() {
	let v = Matrix2x2::<i32>::from([[1, 2], [3, 4]]);
	for i in 0..2 {
		for j in 0..2 {
			assert_eq!(v.as_ref()[i][j], (2*i + j + 1) as i32);
		}
	}

	let v = Matrix2x2::<i32>::from(&[[1, 2], [3, 4]]);
	for i in 0..2 {
		for j in 0..2 {
			assert_eq!(v.as_ref()[i][j], (2*i + j + 1) as i32);
		}
	}

	let a = [1, 2, 3, 4, 5];

	let o = Matrix2x2::<i32>::try_from(&a[..4]);
	assert!(o.is_ok());
	let v = o.unwrap();
	for i in 0..2 {
		for j in 0..2 {
			assert_eq!(v.as_ref()[i][j], (2*i + j + 1) as i32);
		}
	}

	let o = Matrix2x2::<i32>::try_from(&a[..3]);
	assert!(o.is_err());

	let o = Matrix2x2::<i32>::try_from(&a[..]);
	assert!(o.is_err());
}


macro_rules! mat_content_test {
	($M:expr, $N:expr, $W:ident) => (
		let mut m = $W::indices().map(|(i, j)| i + j);
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(m.as_ref()[i][j], i + j);
			}
		}

		let z = $W::fill(0);
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(z.as_ref()[i][j], 0);
			}
		}

		for i in 0..$M {
			for j in 0..$N {
				m.as_mut()[i][j] = i + 2;
			}
		}
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(m.as_ref()[i][j], i + 2);
			}
		}
	)
}
#[test]
fn content() {
	mat_content_test!(2, 2, Matrix2x2);
	mat_content_test!(2, 3, Matrix2x3);
	mat_content_test!(2, 4, Matrix2x4);
	mat_content_test!(3, 2, Matrix3x2);
	mat_content_test!(3, 3, Matrix3x3);
	mat_content_test!(3, 4, Matrix3x4);
	mat_content_test!(4, 2, Matrix4x2);
	mat_content_test!(4, 3, Matrix4x3);
	mat_content_test!(4, 4, Matrix4x4);
}

macro_rules! mat_data_test {
	($M:expr, $N:expr, $W:ident) => (
		let v = $W::indices().map(|(i, j)| i + j*$N + 1);

		let a = v.as_ref();
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(a.as_ref()[i][j], v[(i, j)]);
			}
		}
	)
}
#[test]
fn data() {
	mat_data_test!(2, 2, Matrix2x2);
	mat_data_test!(2, 3, Matrix2x3);
	mat_data_test!(2, 4, Matrix2x4);
	mat_data_test!(3, 2, Matrix3x2);
	mat_data_test!(3, 3, Matrix3x3);
	mat_data_test!(3, 4, Matrix3x4);
	mat_data_test!(4, 2, Matrix4x2);
	mat_data_test!(4, 3, Matrix4x3);
	mat_data_test!(4, 4, Matrix4x4);
}

macro_rules! mat_eq_test {
	($M:expr, $N:expr, $W:ident) => (
		let va = $W::indices().map(|(i, j)| i + j);
		let vb = $W::indices().map(|(i, j)| i + j);
		assert_eq!(va, vb);
	)
}
#[test]
fn eq() {
	mat_eq_test!(2, 2, Matrix2x2);
	mat_eq_test!(2, 3, Matrix2x3);
	mat_eq_test!(2, 4, Matrix2x4);
	mat_eq_test!(3, 2, Matrix3x2);
	mat_eq_test!(3, 3, Matrix3x3);
	mat_eq_test!(3, 4, Matrix3x4);
	mat_eq_test!(4, 2, Matrix4x2);
	mat_eq_test!(4, 3, Matrix4x3);
	mat_eq_test!(4, 4, Matrix4x4);
}

macro_rules! mat_copy_test {
	($M:expr, $N:expr, $W:ident) => (
		let m = $W::indices().map(|(i, j)| i + j);
		let cm = m;
		assert_eq!(m, cm);
	)
}
#[test]
fn copy() {
	mat_copy_test!(2, 2, Matrix2x2);
	mat_copy_test!(2, 3, Matrix2x3);
	mat_copy_test!(2, 4, Matrix2x4);
	mat_copy_test!(3, 2, Matrix3x2);
	mat_copy_test!(3, 3, Matrix3x3);
	mat_copy_test!(3, 4, Matrix3x4);
	mat_copy_test!(4, 2, Matrix4x2);
	mat_copy_test!(4, 3, Matrix4x3);
	mat_copy_test!(4, 4, Matrix4x4);
}

macro_rules! mat_index_test {
	($M:expr, $N:expr, $W:ident) => (
		let mut m = $W::indices().map(|(i, j)| i + j);
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(m[(i, j)], i + j);
			}
		}
		for i in 0..$M {
			for j in 0..$N {
				m[(i, j)] = i*j;
			}
		}
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(m[(i, j)], i*j);
			}
		}
	)
}
#[test]
fn index() {
	mat_index_test!(2, 2, Matrix2x2);
	mat_index_test!(2, 3, Matrix2x3);
	mat_index_test!(2, 4, Matrix2x4);
	mat_index_test!(3, 2, Matrix3x2);
	mat_index_test!(3, 3, Matrix3x3);
	mat_index_test!(3, 4, Matrix3x4);
	mat_index_test!(4, 2, Matrix4x2);
	mat_index_test!(4, 3, Matrix4x3);
	mat_index_test!(4, 4, Matrix4x4);
}

macro_rules! mat_iter_test {
	($M:expr, $N:expr, $W:ident) => (
		let mut m = $W::indices().map(|(i, j)| i*$N + j + 1);
		for (i, c) in m.iter().enumerate() {
			assert_eq!(m.as_ref()[i / $N][i % $N], *c);
		}
		for (i, c) in m.iter_mut().enumerate() {
			*c = i + 2;
		}
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(m[(i, j)], i*$N + j + 2);
			}
		}

		let mut m = $W::fill(0);
		for c in &m {
			assert_eq!(*c, 0);
		}
		for c in &mut m {
			*c = 1;
		}
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(m[(i, j)], 1);
			}
		}
	)
}
#[test]
fn iter() {
	mat_iter_test!(2, 2, Matrix2x2);
	mat_iter_test!(2, 3, Matrix2x3);
	mat_iter_test!(2, 4, Matrix2x4);
	mat_iter_test!(3, 2, Matrix3x2);
	mat_iter_test!(3, 3, Matrix3x3);
	mat_iter_test!(3, 4, Matrix3x4);
	mat_iter_test!(4, 2, Matrix4x2);
	mat_iter_test!(4, 3, Matrix4x3);
	mat_iter_test!(4, 4, Matrix4x4);
}

macro_rules! mat_neg_test {
	($M:expr, $N:expr, $W:ident) => (
		let m = $W::indices().map(|(i, j)| (i + j + 1) as i32);
		let nm = -m;
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(-m[(i, j)], nm[(i, j)]);
			}
		}
	)
}
#[test]
fn neg() {
	mat_neg_test!(2, 2, Matrix2x2);
	mat_neg_test!(2, 3, Matrix2x3);
	mat_neg_test!(2, 4, Matrix2x4);
	mat_neg_test!(3, 2, Matrix3x2);
	mat_neg_test!(3, 3, Matrix3x3);
	mat_neg_test!(3, 4, Matrix3x4);
	mat_neg_test!(4, 2, Matrix4x2);
	mat_neg_test!(4, 3, Matrix4x3);
	mat_neg_test!(4, 4, Matrix4x4);
}

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a*$b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a/$b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a%$b }) }

macro_rules! mat_op_mat_test {
	($M:expr, $N:expr, $W:ident, $op:ident) => (
		let va = $W::indices().map(|(i, j)| (2*(i + j) + 2) as i32);
		let vb = $W::indices().map(|(i, j)| (i + j + 1) as i32);
		let vc = $op!(va, vb);
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(vc[(i, j)], $op!(va[(i, j)], vb[(i, j)]));
			}
		}
	)
}
#[test]
fn mat_add() {
	mat_op_mat_test!(2, 2, Matrix2x2, op_add);
	mat_op_mat_test!(2, 3, Matrix2x3, op_add);
	mat_op_mat_test!(2, 4, Matrix2x4, op_add);
	mat_op_mat_test!(3, 2, Matrix3x2, op_add);
	mat_op_mat_test!(3, 3, Matrix3x3, op_add);
	mat_op_mat_test!(3, 4, Matrix3x4, op_add);
	mat_op_mat_test!(4, 2, Matrix4x2, op_add);
	mat_op_mat_test!(4, 3, Matrix4x3, op_add);
	mat_op_mat_test!(4, 4, Matrix4x4, op_add);
}
#[test]
fn mat_sub() {
	mat_op_mat_test!(2, 2, Matrix2x2, op_sub);
	mat_op_mat_test!(2, 3, Matrix2x3, op_sub);
	mat_op_mat_test!(2, 4, Matrix2x4, op_sub);
	mat_op_mat_test!(3, 2, Matrix3x2, op_sub);
	mat_op_mat_test!(3, 3, Matrix3x3, op_sub);
	mat_op_mat_test!(3, 4, Matrix3x4, op_sub);
	mat_op_mat_test!(4, 2, Matrix4x2, op_sub);
	mat_op_mat_test!(4, 3, Matrix4x3, op_sub);
	mat_op_mat_test!(4, 4, Matrix4x4, op_sub);
}
#[test]
fn mat_mul() {
	mat_op_mat_test!(2, 2, Matrix2x2, op_mul);
	mat_op_mat_test!(2, 3, Matrix2x3, op_mul);
	mat_op_mat_test!(2, 4, Matrix2x4, op_mul);
	mat_op_mat_test!(3, 2, Matrix3x2, op_mul);
	mat_op_mat_test!(3, 3, Matrix3x3, op_mul);
	mat_op_mat_test!(3, 4, Matrix3x4, op_mul);
	mat_op_mat_test!(4, 2, Matrix4x2, op_mul);
	mat_op_mat_test!(4, 3, Matrix4x3, op_mul);
	mat_op_mat_test!(4, 4, Matrix4x4, op_mul);
}
#[test]
fn mat_div() {
	mat_op_mat_test!(2, 2, Matrix2x2, op_div);
	mat_op_mat_test!(2, 3, Matrix2x3, op_div);
	mat_op_mat_test!(2, 4, Matrix2x4, op_div);
	mat_op_mat_test!(3, 2, Matrix3x2, op_div);
	mat_op_mat_test!(3, 3, Matrix3x3, op_div);
	mat_op_mat_test!(3, 4, Matrix3x4, op_div);
	mat_op_mat_test!(4, 2, Matrix4x2, op_div);
	mat_op_mat_test!(4, 3, Matrix4x3, op_div);
	mat_op_mat_test!(4, 4, Matrix4x4, op_div);
}
#[test]
fn mat_rem() {
	mat_op_mat_test!(2, 2, Matrix2x2, op_rem);
	mat_op_mat_test!(2, 3, Matrix2x3, op_rem);
	mat_op_mat_test!(2, 4, Matrix2x4, op_rem);
	mat_op_mat_test!(3, 2, Matrix3x2, op_rem);
	mat_op_mat_test!(3, 3, Matrix3x3, op_rem);
	mat_op_mat_test!(3, 4, Matrix3x4, op_rem);
	mat_op_mat_test!(4, 2, Matrix4x2, op_rem);
	mat_op_mat_test!(4, 3, Matrix4x3, op_rem);
	mat_op_mat_test!(4, 4, Matrix4x4, op_rem);
}

macro_rules! mat_op_scal_test {
	($M:expr, $N:expr, $W:ident, $op:ident) => (
		let v = $W::indices().map(|(i, j)| (2*(i + j) + 2) as i32);
		let a = 3;
		let va = $op!(v, a);
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(va[(i, j)], $op!(v[(i, j)], a));
			}
		}
	)
}
#[test]
fn scal_mul() {
	mat_op_scal_test!(2, 2, Matrix2x2, op_mul);
	mat_op_scal_test!(2, 3, Matrix2x3, op_mul);
	mat_op_scal_test!(2, 4, Matrix2x4, op_mul);
	mat_op_scal_test!(3, 2, Matrix3x2, op_mul);
	mat_op_scal_test!(3, 3, Matrix3x3, op_mul);
	mat_op_scal_test!(3, 4, Matrix3x4, op_mul);
	mat_op_scal_test!(4, 2, Matrix4x2, op_mul);
	mat_op_scal_test!(4, 3, Matrix4x3, op_mul);
	mat_op_scal_test!(4, 4, Matrix4x4, op_mul);
}
#[test]
fn scal_div() {
	mat_op_scal_test!(2, 2, Matrix2x2, op_div);
	mat_op_scal_test!(2, 3, Matrix2x3, op_div);
	mat_op_scal_test!(2, 4, Matrix2x4, op_div);
	mat_op_scal_test!(3, 2, Matrix3x2, op_div);
	mat_op_scal_test!(3, 3, Matrix3x3, op_div);
	mat_op_scal_test!(3, 4, Matrix3x4, op_div);
	mat_op_scal_test!(4, 2, Matrix4x2, op_div);
	mat_op_scal_test!(4, 3, Matrix4x3, op_div);
	mat_op_scal_test!(4, 4, Matrix4x4, op_div);
}
#[test]
fn scal_rem() {
	mat_op_scal_test!(2, 2, Matrix2x2, op_rem);
	mat_op_scal_test!(2, 3, Matrix2x3, op_rem);
	mat_op_scal_test!(2, 4, Matrix2x4, op_rem);
	mat_op_scal_test!(3, 2, Matrix3x2, op_rem);
	mat_op_scal_test!(3, 3, Matrix3x3, op_rem);
	mat_op_scal_test!(3, 4, Matrix3x4, op_rem);
	mat_op_scal_test!(4, 2, Matrix4x2, op_rem);
	mat_op_scal_test!(4, 3, Matrix4x3, op_rem);
	mat_op_scal_test!(4, 4, Matrix4x4, op_rem);
}

macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! mat_op_mat_assign_test {
	($M:expr, $N:expr, $W:ident, $op_assign:ident, $op:ident) => (
		let va = $W::indices().map(|(i, j)| (2*(i + 2*j) + 2) as i32);
		let vb = $W::indices().map(|(i, j)| (i + j + 1) as i32);
		let mut vc = va;
		$op_assign!(vc, vb);
		assert_eq!(vc, $op!(va, vb));
	)
}
#[test]
fn mat_add_assign() {
	mat_op_mat_assign_test!(2, 2, Matrix2x2, op_add_assign, op_add);
	mat_op_mat_assign_test!(2, 3, Matrix2x3, op_add_assign, op_add);
	mat_op_mat_assign_test!(2, 4, Matrix2x4, op_add_assign, op_add);
	mat_op_mat_assign_test!(3, 2, Matrix3x2, op_add_assign, op_add);
	mat_op_mat_assign_test!(3, 3, Matrix3x3, op_add_assign, op_add);
	mat_op_mat_assign_test!(3, 4, Matrix3x4, op_add_assign, op_add);
	mat_op_mat_assign_test!(4, 2, Matrix4x2, op_add_assign, op_add);
	mat_op_mat_assign_test!(4, 3, Matrix4x3, op_add_assign, op_add);
	mat_op_mat_assign_test!(4, 4, Matrix4x4, op_add_assign, op_add);
}
#[test]
fn mat_sub_assign() {
	mat_op_mat_assign_test!(2, 2, Matrix2x2, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(2, 3, Matrix2x3, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(2, 4, Matrix2x4, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(3, 2, Matrix3x2, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(3, 3, Matrix3x3, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(3, 4, Matrix3x4, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(4, 2, Matrix4x2, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(4, 3, Matrix4x3, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(4, 4, Matrix4x4, op_sub_assign, op_sub);
}
#[test]
fn mat_mul_assign() {
	mat_op_mat_assign_test!(2, 2, Matrix2x2, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(2, 3, Matrix2x3, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(2, 4, Matrix2x4, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(3, 2, Matrix3x2, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(3, 3, Matrix3x3, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(3, 4, Matrix3x4, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(4, 2, Matrix4x2, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(4, 3, Matrix4x3, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(4, 4, Matrix4x4, op_mul_assign, op_mul);
}
#[test]
fn mat_div_assign() {
	mat_op_mat_assign_test!(2, 2, Matrix2x2, op_div_assign, op_div);
	mat_op_mat_assign_test!(2, 3, Matrix2x3, op_div_assign, op_div);
	mat_op_mat_assign_test!(2, 4, Matrix2x4, op_div_assign, op_div);
	mat_op_mat_assign_test!(3, 2, Matrix3x2, op_div_assign, op_div);
	mat_op_mat_assign_test!(3, 3, Matrix3x3, op_div_assign, op_div);
	mat_op_mat_assign_test!(3, 4, Matrix3x4, op_div_assign, op_div);
	mat_op_mat_assign_test!(4, 2, Matrix4x2, op_div_assign, op_div);
	mat_op_mat_assign_test!(4, 3, Matrix4x3, op_div_assign, op_div);
	mat_op_mat_assign_test!(4, 4, Matrix4x4, op_div_assign, op_div);
}
#[test]
fn mat_rem_assign() {
	mat_op_mat_assign_test!(2, 2, Matrix2x2, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(2, 3, Matrix2x3, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(2, 4, Matrix2x4, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(3, 2, Matrix3x2, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(3, 3, Matrix3x3, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(3, 4, Matrix3x4, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(4, 2, Matrix4x2, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(4, 3, Matrix4x3, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(4, 4, Matrix4x4, op_rem_assign, op_rem);
}

macro_rules! mat_op_scal_assign_test {
	($M:expr, $N:expr, $W:ident, $op_assign:ident, $op:ident) => (
		let v = $W::indices().map(|(i, j)| (2*(i + 2*j) + 2) as i32);
		let a = 3;
		let mut va = v;
		$op_assign!(va, a);
		assert_eq!(va, $op!(v, a));
	)
}
#[test]
fn scal_mul_assign() {
	mat_op_scal_assign_test!(2, 2, Matrix2x2, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(2, 3, Matrix2x3, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(2, 4, Matrix2x4, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(3, 2, Matrix3x2, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(3, 3, Matrix3x3, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(3, 4, Matrix3x4, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(4, 2, Matrix4x2, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(4, 3, Matrix4x3, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(4, 4, Matrix4x4, op_mul_assign, op_mul);
}
#[test]
fn scal_div_assign() {
	mat_op_scal_assign_test!(2, 2, Matrix2x2, op_div_assign, op_div);
	mat_op_scal_assign_test!(2, 3, Matrix2x3, op_div_assign, op_div);
	mat_op_scal_assign_test!(2, 4, Matrix2x4, op_div_assign, op_div);
	mat_op_scal_assign_test!(3, 2, Matrix3x2, op_div_assign, op_div);
	mat_op_scal_assign_test!(3, 3, Matrix3x3, op_div_assign, op_div);
	mat_op_scal_assign_test!(3, 4, Matrix3x4, op_div_assign, op_div);
	mat_op_scal_assign_test!(4, 2, Matrix4x2, op_div_assign, op_div);
	mat_op_scal_assign_test!(4, 3, Matrix4x3, op_div_assign, op_div);
	mat_op_scal_assign_test!(4, 4, Matrix4x4, op_div_assign, op_div);
}
#[test]
fn scal_rem_assign() {
	mat_op_scal_assign_test!(2, 2, Matrix2x2, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(2, 3, Matrix2x3, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(2, 4, Matrix2x4, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(3, 2, Matrix3x2, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(3, 3, Matrix3x3, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(3, 4, Matrix3x4, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(4, 2, Matrix4x2, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(4, 3, Matrix4x3, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(4, 4, Matrix4x4, op_rem_assign, op_rem);
}

macro_rules! mat_zero_test {
	($M:expr, $N:expr, $W:ident) => (
		let z = $W::<i32>::zero();
		for i in 0..$M {
			for j in 0..$N {
				assert_eq!(z.as_ref()[i][j], 0);
			}
		}
		assert!(z.is_zero());

		let nz = $W::<i32>::fill(1);
		assert!(!nz.is_zero());
	)
}
#[test]
fn zero() {
	mat_zero_test!(2, 2, Matrix2x2);
	mat_zero_test!(2, 3, Matrix2x3);
	mat_zero_test!(2, 4, Matrix2x4);
	mat_zero_test!(3, 2, Matrix3x2);
	mat_zero_test!(3, 3, Matrix3x3);
	mat_zero_test!(3, 4, Matrix3x4);
	mat_zero_test!(4, 2, Matrix4x2);
	mat_zero_test!(4, 3, Matrix4x3);
	mat_zero_test!(4, 4, Matrix4x4);
}

macro_rules! mat_transpose_test {
	($M:expr, $N:expr, $W:ident, $V:ident) => (
		let w = $W::indices().map(|(i, j)| 2*i + 3*j);
		let v = $V::indices().map(|(i, j)| 3*i + 2*j);
		assert_eq!(w.transpose(), v);
	)
}
#[test]
fn transpose() {
	mat_transpose_test!(2, 2, Matrix2x2, Matrix2x2);
	mat_transpose_test!(2, 3, Matrix2x3, Matrix3x2);
	mat_transpose_test!(2, 4, Matrix2x4, Matrix4x2);
	mat_transpose_test!(3, 2, Matrix3x2, Matrix2x3);
	mat_transpose_test!(3, 3, Matrix3x3, Matrix3x3);
	mat_transpose_test!(3, 4, Matrix3x4, Matrix4x3);
	mat_transpose_test!(4, 2, Matrix4x2, Matrix2x4);
	mat_transpose_test!(4, 3, Matrix4x3, Matrix3x4);
	mat_transpose_test!(4, 4, Matrix4x4, Matrix4x4);
}

macro_rules! mat_outer_test {
	($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
		let v = $V::indices().map(|i| i as i32);
		let u = $U::indices().map(|i| 2*i as i32);
		let mat = $W::indices().map(|(i, j)| (2*i*j) as i32);
		let res = v.outer(u);
		assert_eq!(res, mat);
	)
}
#[test]
fn outer() {
	mat_outer_test!(2, 2, Matrix2x2, Vector2, Vector2);
	mat_outer_test!(2, 3, Matrix2x3, Vector2, Vector3);
	mat_outer_test!(2, 4, Matrix2x4, Vector2, Vector4);
	mat_outer_test!(3, 2, Matrix3x2, Vector3, Vector2);
	mat_outer_test!(3, 3, Matrix3x3, Vector3, Vector3);
	mat_outer_test!(3, 4, Matrix3x4, Vector3, Vector4);
	mat_outer_test!(4, 2, Matrix4x2, Vector4, Vector2);
	mat_outer_test!(4, 3, Matrix4x3, Vector4, Vector3);
	mat_outer_test!(4, 4, Matrix4x4, Vector4, Vector4);
}

macro_rules! mat_row_col_test {
	($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
		let mat = $W::indices().map(|(i, j)| 2*i + 3*j);
		for i in 0..$M {
			assert_eq!(mat.row(i), $U::indices().map(|j| 2*i + 3*j));
		}
		for j in 0..$N {
			assert_eq!(mat.col(j), $V::indices().map(|i| 2*i + 3*j));
		}

	)
}
#[test]
fn row_col() {
	mat_row_col_test!(2, 2, Matrix2x2, Vector2, Vector2);
	mat_row_col_test!(2, 3, Matrix2x3, Vector2, Vector3);
	mat_row_col_test!(2, 4, Matrix2x4, Vector2, Vector4);
	mat_row_col_test!(3, 2, Matrix3x2, Vector3, Vector2);
	mat_row_col_test!(3, 3, Matrix3x3, Vector3, Vector3);
	mat_row_col_test!(3, 4, Matrix3x4, Vector3, Vector4);
	mat_row_col_test!(4, 2, Matrix4x2, Vector4, Vector2);
	mat_row_col_test!(4, 3, Matrix4x3, Vector4, Vector3);
	mat_row_col_test!(4, 4, Matrix4x4, Vector4, Vector4);
}

macro_rules! mat_mul_mv_test {
	($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
		let m = $W::fill(1 as i32);
		let u = $U::fill(1 as i32);
		assert_eq!(m.dot(u), $V::fill($N));
	)
}
#[test]
fn mul_mv() {
	mat_mul_mv_test!(2, 2, Matrix2x2, Vector2, Vector2);
	mat_mul_mv_test!(2, 3, Matrix2x3, Vector2, Vector3);
	mat_mul_mv_test!(2, 4, Matrix2x4, Vector2, Vector4);
	mat_mul_mv_test!(3, 2, Matrix3x2, Vector3, Vector2);
	mat_mul_mv_test!(3, 3, Matrix3x3, Vector3, Vector3);
	mat_mul_mv_test!(3, 4, Matrix3x4, Vector3, Vector4);
	mat_mul_mv_test!(4, 2, Matrix4x2, Vector4, Vector2);
	mat_mul_mv_test!(4, 3, Matrix4x3, Vector4, Vector3);
	mat_mul_mv_test!(4, 4, Matrix4x4, Vector4, Vector4);
}

macro_rules! mat_mul_vm_test {
	($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
		let m = $W::fill(1 as i32);
		let v = $V::fill(1 as i32);
		assert_eq!(v.dot(m), $U::fill($M));
	)
}
#[test]
fn mul_vm() {
	mat_mul_vm_test!(2, 2, Matrix2x2, Vector2, Vector2);
	mat_mul_vm_test!(2, 3, Matrix2x3, Vector2, Vector3);
	mat_mul_vm_test!(2, 4, Matrix2x4, Vector2, Vector4);
	mat_mul_vm_test!(3, 2, Matrix3x2, Vector3, Vector2);
	mat_mul_vm_test!(3, 3, Matrix3x3, Vector3, Vector3);
	mat_mul_vm_test!(3, 4, Matrix3x4, Vector3, Vector4);
	mat_mul_vm_test!(4, 2, Matrix4x2, Vector4, Vector2);
	mat_mul_vm_test!(4, 3, Matrix4x3, Vector4, Vector3);
	mat_mul_vm_test!(4, 4, Matrix4x4, Vector4, Vector4);
}

macro_rules! mat_mul_mm_test {
	($L:expr, $M:expr, $N:expr, $Wlm:ident, $Wmn:ident, $Wln:ident) => (
		let wlm = $Wlm::fill(1 as i32);
		let wmn = $Wmn::fill(1 as i32);
		assert_eq!(wlm.dot(wmn), $Wln::fill($M as i32));
	)
}
#[test]
fn mul_mm() {
	mat_mul_mm_test!(2, 2, 2, Matrix2x2, Matrix2x2, Matrix2x2);
	mat_mul_mm_test!(2, 2, 3, Matrix2x2, Matrix2x3, Matrix2x3);
	mat_mul_mm_test!(2, 2, 4, Matrix2x2, Matrix2x4, Matrix2x4);
	mat_mul_mm_test!(2, 3, 2, Matrix2x3, Matrix3x2, Matrix2x2);
	mat_mul_mm_test!(2, 3, 3, Matrix2x3, Matrix3x3, Matrix2x3);
	mat_mul_mm_test!(2, 3, 4, Matrix2x3, Matrix3x4, Matrix2x4);
	mat_mul_mm_test!(2, 4, 2, Matrix2x4, Matrix4x2, Matrix2x2);
	mat_mul_mm_test!(2, 4, 3, Matrix2x4, Matrix4x3, Matrix2x3);
	mat_mul_mm_test!(2, 4, 4, Matrix2x4, Matrix4x4, Matrix2x4);
	mat_mul_mm_test!(3, 2, 2, Matrix3x2, Matrix2x2, Matrix3x2);
	mat_mul_mm_test!(3, 2, 3, Matrix3x2, Matrix2x3, Matrix3x3);
	mat_mul_mm_test!(3, 2, 4, Matrix3x2, Matrix2x4, Matrix3x4);
	mat_mul_mm_test!(3, 3, 2, Matrix3x3, Matrix3x2, Matrix3x2);
	mat_mul_mm_test!(3, 3, 3, Matrix3x3, Matrix3x3, Matrix3x3);
	mat_mul_mm_test!(3, 3, 4, Matrix3x3, Matrix3x4, Matrix3x4);
	mat_mul_mm_test!(3, 4, 2, Matrix3x4, Matrix4x2, Matrix3x2);
	mat_mul_mm_test!(3, 4, 3, Matrix3x4, Matrix4x3, Matrix3x3);
	mat_mul_mm_test!(3, 4, 4, Matrix3x4, Matrix4x4, Matrix3x4);
	mat_mul_mm_test!(4, 2, 2, Matrix4x2, Matrix2x2, Matrix4x2);
	mat_mul_mm_test!(4, 2, 3, Matrix4x2, Matrix2x3, Matrix4x3);
	mat_mul_mm_test!(4, 2, 4, Matrix4x2, Matrix2x4, Matrix4x4);
	mat_mul_mm_test!(4, 3, 2, Matrix4x3, Matrix3x2, Matrix4x2);
	mat_mul_mm_test!(4, 3, 3, Matrix4x3, Matrix3x3, Matrix4x3);
	mat_mul_mm_test!(4, 3, 4, Matrix4x3, Matrix3x4, Matrix4x4);
	mat_mul_mm_test!(4, 4, 2, Matrix4x4, Matrix4x2, Matrix4x2);
	mat_mul_mm_test!(4, 4, 3, Matrix4x4, Matrix4x3, Matrix4x3);
	mat_mul_mm_test!(4, 4, 4, Matrix4x4, Matrix4x4, Matrix4x4);
}

macro_rules! mat_one_test {
	($W:ident, $N:expr) => (
		let o = $W::<i32>::one();
		for j in 0..$N {
			for i in 0..$N {
				assert_eq!(o[(i, j)], if i == j { 1 } else { 0 });
			}
		}
	)
}
#[test]
fn one() {
	mat_one_test!(Matrix2x2, 2);
	mat_one_test!(Matrix3x3, 3);
	mat_one_test!(Matrix4x4, 4);
}
#[test]
fn det() {
	let m = Matrix2x2::<i32>::from([[11, 12], [21, 22]]);
	assert_eq!(m.det(), 11*22 - 12*21);

	let m = Matrix3x3::<i32>::from([[11, 12, 13], [21, 22, 23], [31, 32, 33]]);
	assert_eq!(m.det(), 11*(22*33 - 23*32) + 12*(23*31 - 21*33) + 13*(21*32 - 22*31));
}
#[test]
fn inverse() {
	let m = Matrix2x2::<f64>::from([[11.0, 12.0], [21.0, 22.0]]).inverse();
	let im = Matrix2x2::<f64>::from([[22.0, -12.0], [-21.0, 11.0]])/(11.0*22.0 - 12.0*21.0);
	let dm = m - im;
	assert!(dm[(0, 0)].abs() + dm[(0, 1)].abs() + dm[(1, 0)].abs() + dm[(1, 1)].abs() < 1e-8);
}
