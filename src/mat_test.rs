use mat::*;
use vec::*;

macro_rules! mat_new_test {
	($V:ident, $N:expr, $M:expr) => (
		$V::<i32>::new();
	)
}

#[test]
fn new() {
	mat_new_test!(Mat2x2, 2, 2);
	mat_new_test!(Mat2x3, 2, 3);
	mat_new_test!(Mat2x4, 2, 4);
	mat_new_test!(Mat3x2, 3, 2);
	mat_new_test!(Mat3x3, 3, 3);
	mat_new_test!(Mat3x4, 3, 4);
	mat_new_test!(Mat4x2, 4, 2);
	mat_new_test!(Mat4x3, 4, 3);
	mat_new_test!(Mat4x4, 4, 4);
}

#[test]
fn new_no_gen() {
	let v = Mat2i32::from(1, 2, 3, 4);
	for i in 0..4 {
		assert_eq!(v.d[i], i as i32 + 1);
	}

	let v = Mat2i32::from_array([1, 2, 3, 4]);
	for i in 0..4 {
		assert_eq!(v.d[i], i as i32 + 1);
	}

	let v = Mat2i32::from_array_ref(&[1, 2, 3, 4]);
	for i in 0..4 {
		assert_eq!(v.d[i], i as i32 + 1);
	}

	let a = [1, 2, 3, 4, 5];

	let o = Mat2i32::from_slice(&a[..4]);
	assert!(o.is_some());
	let v = o.unwrap();
	for i in 0..4 {
		assert_eq!(v.d[i], i as i32 + 1);
	}

	let o = Mat2i32::from_slice(&a[..3]);
	assert!(o.is_none());

	let o = Mat2i32::from_slice(&a[..]);
	assert!(o.is_none());
}


macro_rules! mat_content_test {
	($V:ident, $N:expr, $M:expr) => (
		let mut m = $V::from_map(|i, j| i + j);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m.d[i + $N*j], i + j);
			}
		}

		let z = $V::from_scal(0);
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
	mat_content_test!(Mat2x2, 2, 2);
	mat_content_test!(Mat2x3, 2, 3);
	mat_content_test!(Mat2x4, 2, 4);
	mat_content_test!(Mat3x2, 3, 2);
	mat_content_test!(Mat3x3, 3, 3);
	mat_content_test!(Mat3x4, 3, 4);
	mat_content_test!(Mat4x2, 4, 2);
	mat_content_test!(Mat4x3, 4, 3);
	mat_content_test!(Mat4x4, 4, 4);
}

macro_rules! mat_data_test {
	($V:ident, $N:expr, $M:expr) => (
		let v = $V::from_map(|i, j| i + j + 1);

		let a = &v.d;
		let b = v.data(); 
		for i in 0..($N*$M) {
			assert_eq!(a[i], b[i]);
		}
	)
}

#[test]
fn data() {
	mat_data_test!(Mat2x2, 2, 2);
	mat_data_test!(Mat2x3, 2, 3);
	mat_data_test!(Mat2x4, 2, 4);
	mat_data_test!(Mat3x2, 3, 2);
	mat_data_test!(Mat3x3, 3, 3);
	mat_data_test!(Mat3x4, 3, 4);
	mat_data_test!(Mat4x2, 4, 2);
	mat_data_test!(Mat4x3, 4, 3);
	mat_data_test!(Mat4x4, 4, 4);
}

macro_rules! mat_eq_test {
	($V:ident, $N:expr, $M:expr) => (
		let va = $V::from_map(|i, j| i + j);
		let vb = $V::from_map(|i, j| i + j);
		assert_eq!(va, vb);
	)
}

#[test]
fn eq() {
	mat_eq_test!(Mat2x2, 2, 2);
	mat_eq_test!(Mat2x3, 2, 3);
	mat_eq_test!(Mat2x4, 2, 4);
	mat_eq_test!(Mat3x2, 3, 2);
	mat_eq_test!(Mat3x3, 3, 3);
	mat_eq_test!(Mat3x4, 3, 4);
	mat_eq_test!(Mat4x2, 4, 2);
	mat_eq_test!(Mat4x3, 4, 3);
	mat_eq_test!(Mat4x4, 4, 4);
}

macro_rules! mat_copy_test {
	($V:ident, $N:expr, $M:expr) => (
		let m = $V::from_map(|i, j| i + j);
		let cm = m;
		assert_eq!(m, cm);
	)
}

#[test]
fn copy() {
	mat_copy_test!(Mat2x2, 2, 2);
	mat_copy_test!(Mat2x3, 2, 3);
	mat_copy_test!(Mat2x4, 2, 4);
	mat_copy_test!(Mat3x2, 3, 2);
	mat_copy_test!(Mat3x3, 3, 3);
	mat_copy_test!(Mat3x4, 3, 4);
	mat_copy_test!(Mat4x2, 4, 2);
	mat_copy_test!(Mat4x3, 4, 3);
	mat_copy_test!(Mat4x4, 4, 4);
}

macro_rules! mat_index_test {
	($V:ident, $N:expr, $M:expr) => (
		let mut m = $V::from_map(|i, j| i + j);
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
	mat_index_test!(Mat2x2, 2, 2);
	mat_index_test!(Mat2x3, 2, 3);
	mat_index_test!(Mat2x4, 2, 4);
	mat_index_test!(Mat3x2, 3, 2);
	mat_index_test!(Mat3x3, 3, 3);
	mat_index_test!(Mat3x4, 3, 4);
	mat_index_test!(Mat4x2, 4, 2);
	mat_index_test!(Mat4x3, 4, 3);
	mat_index_test!(Mat4x4, 4, 4);
}

macro_rules! mat_iter_test {
	($V:ident, $N:expr, $M:expr) => (
		let mut m = $V::from_map(|i, j| i + j*$N + 1);
		for (i, c) in m.iter().enumerate() {
			assert_eq!(m.d[i], *c);
		}
		for (i, c) in m.iter_mut().enumerate() {
			*c = i + 2;
		}
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m[(i, j)], i + j*$N + 2);
			}
		}

		let mut m = $V::from_scal(0);
		for c in &m {
			assert_eq!(*c, 0);
		}
		for c in &mut m {
			*c = 1;
		}
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(m[(i, j)], 1);
			}
		}
	)
}

#[test]
fn iter() {
	mat_iter_test!(Mat2x2, 2, 2);
	mat_iter_test!(Mat2x3, 2, 3);
	mat_iter_test!(Mat2x4, 2, 4);
	mat_iter_test!(Mat3x2, 3, 2);
	mat_iter_test!(Mat3x3, 3, 3);
	mat_iter_test!(Mat3x4, 3, 4);
	mat_iter_test!(Mat4x2, 4, 2);
	mat_iter_test!(Mat4x3, 4, 3);
	mat_iter_test!(Mat4x4, 4, 4);
}

macro_rules! mat_neg_test {
	($V:ident, $N:expr, $M:expr) => (
		let m = $V::from_map(|i, j| (i + j + 1) as i32);
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
	mat_neg_test!(Mat2x2, 2, 2);
	mat_neg_test!(Mat2x3, 2, 3);
	mat_neg_test!(Mat2x4, 2, 4);
	mat_neg_test!(Mat3x2, 3, 2);
	mat_neg_test!(Mat3x3, 3, 3);
	mat_neg_test!(Mat3x4, 3, 4);
	mat_neg_test!(Mat4x2, 4, 2);
	mat_neg_test!(Mat4x3, 4, 3);
	mat_neg_test!(Mat4x4, 4, 4);
}

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a*$b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a/$b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a%$b }) }

macro_rules! mat_op_mat_test {
	($V:ident, $N:expr, $M:expr, $op:ident) => (
		let va = $V::from_map(|i, j| (2*(i + j) + 2) as i32);
		let vb = $V::from_map(|i, j| (i + j + 1) as i32);
		let vc = $op!(va, vb);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(vc[(i, j)], $op!(va[(i, j)], vb[(i, j)]));
			}
		}
	)
}

#[test]
fn mat_add() {
	mat_op_mat_test!(Mat2x2, 2, 2, op_add);
	mat_op_mat_test!(Mat2x3, 2, 3, op_add);
	mat_op_mat_test!(Mat2x4, 2, 4, op_add);
	mat_op_mat_test!(Mat3x2, 3, 2, op_add);
	mat_op_mat_test!(Mat3x3, 3, 3, op_add);
	mat_op_mat_test!(Mat3x4, 3, 4, op_add);
	mat_op_mat_test!(Mat4x2, 4, 2, op_add);
	mat_op_mat_test!(Mat4x3, 4, 3, op_add);
	mat_op_mat_test!(Mat4x4, 4, 4, op_add);
}

#[test]
fn mat_sub() {
	mat_op_mat_test!(Mat2x2, 2, 2, op_sub);
	mat_op_mat_test!(Mat2x3, 2, 3, op_sub);
	mat_op_mat_test!(Mat2x4, 2, 4, op_sub);
	mat_op_mat_test!(Mat3x2, 3, 2, op_sub);
	mat_op_mat_test!(Mat3x3, 3, 3, op_sub);
	mat_op_mat_test!(Mat3x4, 3, 4, op_sub);
	mat_op_mat_test!(Mat4x2, 4, 2, op_sub);
	mat_op_mat_test!(Mat4x3, 4, 3, op_sub);
	mat_op_mat_test!(Mat4x4, 4, 4, op_sub);
}

#[test]
fn mat_mul() {
	mat_op_mat_test!(Mat2x2, 2, 2, op_mul);
	mat_op_mat_test!(Mat2x3, 2, 3, op_mul);
	mat_op_mat_test!(Mat2x4, 2, 4, op_mul);
	mat_op_mat_test!(Mat3x2, 3, 2, op_mul);
	mat_op_mat_test!(Mat3x3, 3, 3, op_mul);
	mat_op_mat_test!(Mat3x4, 3, 4, op_mul);
	mat_op_mat_test!(Mat4x2, 4, 2, op_mul);
	mat_op_mat_test!(Mat4x3, 4, 3, op_mul);
	mat_op_mat_test!(Mat4x4, 4, 4, op_mul);
}

#[test]
fn mat_div() {
	mat_op_mat_test!(Mat2x2, 2, 2, op_div);
	mat_op_mat_test!(Mat2x3, 2, 3, op_div);
	mat_op_mat_test!(Mat2x4, 2, 4, op_div);
	mat_op_mat_test!(Mat3x2, 3, 2, op_div);
	mat_op_mat_test!(Mat3x3, 3, 3, op_div);
	mat_op_mat_test!(Mat3x4, 3, 4, op_div);
	mat_op_mat_test!(Mat4x2, 4, 2, op_div);
	mat_op_mat_test!(Mat4x3, 4, 3, op_div);
	mat_op_mat_test!(Mat4x4, 4, 4, op_div);
}

#[test]
fn mat_rem() {
	mat_op_mat_test!(Mat2x2, 2, 2, op_rem);
	mat_op_mat_test!(Mat2x3, 2, 3, op_rem);
	mat_op_mat_test!(Mat2x4, 2, 4, op_rem);
	mat_op_mat_test!(Mat3x2, 3, 2, op_rem);
	mat_op_mat_test!(Mat3x3, 3, 3, op_rem);
	mat_op_mat_test!(Mat3x4, 3, 4, op_rem);
	mat_op_mat_test!(Mat4x2, 4, 2, op_rem);
	mat_op_mat_test!(Mat4x3, 4, 3, op_rem);
	mat_op_mat_test!(Mat4x4, 4, 4, op_rem);
}

macro_rules! mat_op_scal_test {
	($V:ident, $N:expr, $M:expr, $op:ident) => (
		let v = $V::from_map(|i, j| (2*(i + j) + 2) as i32);
		let a = 3;
		let va = $op!(v, a);
		for j in 0..$M {
			for i in 0..$N {
				assert_eq!(va[(i, j)], $op!(v[(i, j)], a));
			}
		}
	)
}

#[test]
fn scal_mul() {
	mat_op_scal_test!(Mat2x2, 2, 2, op_mul);
	mat_op_scal_test!(Mat2x3, 2, 3, op_mul);
	mat_op_scal_test!(Mat2x4, 2, 4, op_mul);
	mat_op_scal_test!(Mat3x2, 3, 2, op_mul);
	mat_op_scal_test!(Mat3x3, 3, 3, op_mul);
	mat_op_scal_test!(Mat3x4, 3, 4, op_mul);
	mat_op_scal_test!(Mat4x2, 4, 2, op_mul);
	mat_op_scal_test!(Mat4x3, 4, 3, op_mul);
	mat_op_scal_test!(Mat4x4, 4, 4, op_mul);
}

#[test]
fn scal_div() {
	mat_op_scal_test!(Mat2x2, 2, 2, op_div);
	mat_op_scal_test!(Mat2x3, 2, 3, op_div);
	mat_op_scal_test!(Mat2x4, 2, 4, op_div);
	mat_op_scal_test!(Mat3x2, 3, 2, op_div);
	mat_op_scal_test!(Mat3x3, 3, 3, op_div);
	mat_op_scal_test!(Mat3x4, 3, 4, op_div);
	mat_op_scal_test!(Mat4x2, 4, 2, op_div);
	mat_op_scal_test!(Mat4x3, 4, 3, op_div);
	mat_op_scal_test!(Mat4x4, 4, 4, op_div);
}

#[test]
fn scal_rem() {
	mat_op_scal_test!(Mat2x2, 2, 2, op_rem);
	mat_op_scal_test!(Mat2x3, 2, 3, op_rem);
	mat_op_scal_test!(Mat2x4, 2, 4, op_rem);
	mat_op_scal_test!(Mat3x2, 3, 2, op_rem);
	mat_op_scal_test!(Mat3x3, 3, 3, op_rem);
	mat_op_scal_test!(Mat3x4, 3, 4, op_rem);
	mat_op_scal_test!(Mat4x2, 4, 2, op_rem);
	mat_op_scal_test!(Mat4x3, 4, 3, op_rem);
	mat_op_scal_test!(Mat4x4, 4, 4, op_rem);
}

macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! mat_op_mat_assign_test {
	($V:ident, $N:expr, $M:expr, $op_assign:ident, $op:ident) => (
		let va = $V::from_map(|i, j| (2*(i + 2*j) + 2) as i32);
		let vb = $V::from_map(|i, j| (i + j + 1) as i32);
		let mut vc = va;
		$op_assign!(vc, vb);
		assert_eq!(vc, $op!(va, vb));
	)
}

#[test]
fn mat_add_assign() {
	mat_op_mat_assign_test!(Mat2x2, 2, 2, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat2x3, 2, 3, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat2x4, 2, 4, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat3x2, 3, 2, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat3x3, 3, 3, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat3x4, 3, 4, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat4x2, 4, 2, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat4x3, 4, 3, op_add_assign, op_add);
	mat_op_mat_assign_test!(Mat4x4, 4, 4, op_add_assign, op_add);
}

#[test]
fn mat_sub_assign() {
	mat_op_mat_assign_test!(Mat2x2, 2, 2, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat2x3, 2, 3, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat2x4, 2, 4, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat3x2, 3, 2, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat3x3, 3, 3, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat3x4, 3, 4, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat4x2, 4, 2, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat4x3, 4, 3, op_sub_assign, op_sub);
	mat_op_mat_assign_test!(Mat4x4, 4, 4, op_sub_assign, op_sub);
}

#[test]
fn mat_mul_assign() {
	mat_op_mat_assign_test!(Mat2x2, 2, 2, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(Mat2x3, 2, 3, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(Mat2x4, 2, 4, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(Mat3x2, 3, 2, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(Mat3x3, 3, 3, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(Mat3x4, 3, 4, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(Mat4x2, 4, 2, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(Mat4x3, 4, 3, op_mul_assign, op_mul);
	mat_op_mat_assign_test!(Mat4x4, 4, 4, op_mul_assign, op_mul);
}

#[test]
fn mat_div_assign() {
	mat_op_mat_assign_test!(Mat2x2, 2, 2, op_div_assign, op_div);
	mat_op_mat_assign_test!(Mat2x3, 2, 3, op_div_assign, op_div);
	mat_op_mat_assign_test!(Mat2x4, 2, 4, op_div_assign, op_div);
	mat_op_mat_assign_test!(Mat3x2, 3, 2, op_div_assign, op_div);
	mat_op_mat_assign_test!(Mat3x3, 3, 3, op_div_assign, op_div);
	mat_op_mat_assign_test!(Mat3x4, 3, 4, op_div_assign, op_div);
	mat_op_mat_assign_test!(Mat4x2, 4, 2, op_div_assign, op_div);
	mat_op_mat_assign_test!(Mat4x3, 4, 3, op_div_assign, op_div);
	mat_op_mat_assign_test!(Mat4x4, 4, 4, op_div_assign, op_div);
}

#[test]
fn mat_rem_assign() {
	mat_op_mat_assign_test!(Mat2x2, 2, 2, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(Mat2x3, 2, 3, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(Mat2x4, 2, 4, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(Mat3x2, 3, 2, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(Mat3x3, 3, 3, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(Mat3x4, 3, 4, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(Mat4x2, 4, 2, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(Mat4x3, 4, 3, op_rem_assign, op_rem);
	mat_op_mat_assign_test!(Mat4x4, 4, 4, op_rem_assign, op_rem);
}

macro_rules! mat_op_scal_assign_test {
	($V:ident, $N:expr, $M:expr, $op_assign:ident, $op:ident) => (
		let v = $V::from_map(|i, j| (2*(i + 2*j) + 2) as i32);
		let a = 3;
		let mut va = v;
		$op_assign!(va, a);
		assert_eq!(va, $op!(v, a));
	)
}

#[test]
fn scal_mul_assign() {
	mat_op_scal_assign_test!(Mat2x2, 2, 2, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat2x3, 2, 3, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat2x4, 2, 4, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat3x2, 3, 2, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat3x3, 3, 3, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat3x4, 3, 4, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat4x2, 4, 2, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat4x3, 4, 3, op_mul_assign, op_mul);
	mat_op_scal_assign_test!(Mat4x4, 4, 4, op_mul_assign, op_mul);
}

#[test]
fn scal_div_assign() {
	mat_op_scal_assign_test!(Mat2x2, 2, 2, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat2x3, 2, 3, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat2x4, 2, 4, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat3x2, 3, 2, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat3x3, 3, 3, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat3x4, 3, 4, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat4x2, 4, 2, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat4x3, 4, 3, op_div_assign, op_div);
	mat_op_scal_assign_test!(Mat4x4, 4, 4, op_div_assign, op_div);
}

#[test]
fn scal_rem_assign() {
	mat_op_scal_assign_test!(Mat2x2, 2, 2, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat2x3, 2, 3, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat2x4, 2, 4, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat3x2, 3, 2, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat3x3, 3, 3, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat3x4, 3, 4, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat4x2, 4, 2, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat4x3, 4, 3, op_rem_assign, op_rem);
	mat_op_scal_assign_test!(Mat4x4, 4, 4, op_rem_assign, op_rem);
}

macro_rules! mat_zero_test {
	($V:ident, $N:expr, $M:expr) => (
		let z = $V::<i32>::zero();
		for i in 0..($N*$M) {
			assert_eq!(z.d[i], 0);
		}
		assert!(z.is_zero());
		
		let nz = $V::<i32> { d: [1; ($N*$M)] };
		assert!(!nz.is_zero());
	)
}

#[test]
fn zero() {
	mat_zero_test!(Mat2x2, 2, 2);
	mat_zero_test!(Mat2x3, 2, 3);
	mat_zero_test!(Mat2x4, 2, 4);
	mat_zero_test!(Mat3x2, 3, 2);
	mat_zero_test!(Mat3x3, 3, 3);
	mat_zero_test!(Mat3x4, 3, 4);
	mat_zero_test!(Mat4x2, 4, 2);
	mat_zero_test!(Mat4x3, 4, 3);
	mat_zero_test!(Mat4x4, 4, 4);
}

macro_rules! mat_transpose_test {
	($Vnm:ident, $Vmn:ident, $N:expr, $M:expr) => (
		let vnm = $Vnm::from_map(|i, j| 2*i + 3*j);
		let vmn = $Vmn::from_map(|i, j| 3*i + 2*j);
		assert_eq!(vnm.transpose(), vmn);
	)
}

#[test]
fn transpose() {
	mat_transpose_test!(Mat2x2, Mat2x2, 2, 2);
	mat_transpose_test!(Mat2x3, Mat3x2, 2, 3);
	mat_transpose_test!(Mat2x4, Mat4x2, 2, 4);
	mat_transpose_test!(Mat3x2, Mat2x3, 3, 2);
	mat_transpose_test!(Mat3x3, Mat3x3, 3, 3);
	mat_transpose_test!(Mat3x4, Mat4x3, 3, 4);
	mat_transpose_test!(Mat4x2, Mat2x4, 4, 2);
	mat_transpose_test!(Mat4x3, Mat3x4, 4, 3);
	mat_transpose_test!(Mat4x4, Mat4x4, 4, 4);
}

macro_rules! mat_outer_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let vn = $Vn::from_map(|i| i as i32);
		let vm = $Vm::from_map(|i| 2*i as i32);
		let mat = $Vnm::from_map(|i, j| (2*i*j) as i32);
		let res = vm.outer(vn);
		assert_eq!(res, mat);
	)
}

#[test]
fn outer() {
	mat_outer_test!(Mat2x2, Vec2, Vec2, 2, 2);
	mat_outer_test!(Mat2x3, Vec2, Vec3, 2, 3);
	mat_outer_test!(Mat2x4, Vec2, Vec4, 2, 4);
	mat_outer_test!(Mat3x2, Vec3, Vec2, 3, 2);
	mat_outer_test!(Mat3x3, Vec3, Vec3, 3, 3);
	mat_outer_test!(Mat3x4, Vec3, Vec4, 3, 4);
	mat_outer_test!(Mat4x2, Vec4, Vec2, 4, 2);
	mat_outer_test!(Mat4x3, Vec4, Vec3, 4, 3);
	mat_outer_test!(Mat4x4, Vec4, Vec4, 4, 4);
}

macro_rules! mat_row_col_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let mat = $Vnm::from_map(|i, j| 2*i + 3*j);
		for i in 0..$N {
			assert_eq!(mat.col(i), $Vm::from_map(|j| 2*i + 3*j));
		}
		for j in 0..$M {
			assert_eq!(mat.row(j), $Vn::from_map(|i| 2*i + 3*j));
		}
		
	)
}

#[test]
fn row_col() {
	mat_row_col_test!(Mat2x2, Vec2, Vec2, 2, 2);
	mat_row_col_test!(Mat2x3, Vec2, Vec3, 2, 3);
	mat_row_col_test!(Mat2x4, Vec2, Vec4, 2, 4);
	mat_row_col_test!(Mat3x2, Vec3, Vec2, 3, 2);
	mat_row_col_test!(Mat3x3, Vec3, Vec3, 3, 3);
	mat_row_col_test!(Mat3x4, Vec3, Vec4, 3, 4);
	mat_row_col_test!(Mat4x2, Vec4, Vec2, 4, 2);
	mat_row_col_test!(Mat4x3, Vec4, Vec3, 4, 3);
	mat_row_col_test!(Mat4x4, Vec4, Vec4, 4, 4);
}

macro_rules! mat_mul_vec_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let m = $Vnm::from_scal(1 as i32);
		let v = $Vn::from_scal(1 as i32);
		assert_eq!(m.dot(v), $Vm::from_scal($N));
	)
}

#[test]
fn mul_vec() {
	mat_mul_vec_test!(Mat2x2, Vec2, Vec2, 2, 2);
	mat_mul_vec_test!(Mat2x3, Vec2, Vec3, 2, 3);
	mat_mul_vec_test!(Mat2x4, Vec2, Vec4, 2, 4);
	mat_mul_vec_test!(Mat3x2, Vec3, Vec2, 3, 2);
	mat_mul_vec_test!(Mat3x3, Vec3, Vec3, 3, 3);
	mat_mul_vec_test!(Mat3x4, Vec3, Vec4, 3, 4);
	mat_mul_vec_test!(Mat4x2, Vec4, Vec2, 4, 2);
	mat_mul_vec_test!(Mat4x3, Vec4, Vec3, 4, 3);
	mat_mul_vec_test!(Mat4x4, Vec4, Vec4, 4, 4);
}

macro_rules! mat_mul_vec_mat_test {
	($Vnm:ident, $Vn:ident, $Vm:ident, $N:expr, $M:expr) => (
		let m = $Vnm::from_scal(1 as i32);
		let v = $Vm::from_scal(1 as i32);
		assert_eq!(v.dot(m), $Vn::from_scal($M));
	)
}

#[test]
fn mul_vec_mat() {
	mat_mul_vec_mat_test!(Mat2x2, Vec2, Vec2, 2, 2);
	mat_mul_vec_mat_test!(Mat2x3, Vec2, Vec3, 2, 3);
	mat_mul_vec_mat_test!(Mat2x4, Vec2, Vec4, 2, 4);
	mat_mul_vec_mat_test!(Mat3x2, Vec3, Vec2, 3, 2);
	mat_mul_vec_mat_test!(Mat3x3, Vec3, Vec3, 3, 3);
	mat_mul_vec_mat_test!(Mat3x4, Vec3, Vec4, 3, 4);
	mat_mul_vec_mat_test!(Mat4x2, Vec4, Vec2, 4, 2);
	mat_mul_vec_mat_test!(Mat4x3, Vec4, Vec3, 4, 3);
	mat_mul_vec_mat_test!(Mat4x4, Vec4, Vec4, 4, 4);
}

macro_rules! mat_mul_mat_test {
	($Vnm:ident, $Vln:ident, $Vlm:ident, $N:expr, $M:expr, $L:expr) => (
		let vnm = $Vnm::from_scal(1 as i32);
		let vln = $Vln::from_scal(1 as i32);
		assert_eq!(vnm.dot(vln), $Vlm::from_scal($N as i32));
	)
}

#[test]
fn mul_mat() {
	mat_mul_mat_test!(Mat2x2, Mat2x2, Mat2x2, 2, 2, 2);
	mat_mul_mat_test!(Mat2x2, Mat3x2, Mat3x2, 2, 2, 3);
	mat_mul_mat_test!(Mat2x2, Mat4x2, Mat4x2, 2, 2, 4);
	mat_mul_mat_test!(Mat2x3, Mat2x2, Mat2x3, 2, 3, 2);
	mat_mul_mat_test!(Mat2x3, Mat3x2, Mat3x3, 2, 3, 3);
	mat_mul_mat_test!(Mat2x3, Mat4x2, Mat4x3, 2, 3, 4);
	mat_mul_mat_test!(Mat2x4, Mat2x2, Mat2x4, 2, 4, 2);
	mat_mul_mat_test!(Mat2x4, Mat3x2, Mat3x4, 2, 4, 3);
	mat_mul_mat_test!(Mat2x4, Mat4x2, Mat4x4, 2, 4, 4);
	mat_mul_mat_test!(Mat3x2, Mat2x3, Mat2x2, 3, 2, 2);
	mat_mul_mat_test!(Mat3x2, Mat3x3, Mat3x2, 3, 2, 3);
	mat_mul_mat_test!(Mat3x2, Mat4x3, Mat4x2, 3, 2, 4);
	mat_mul_mat_test!(Mat3x3, Mat2x3, Mat2x3, 3, 3, 2);
	mat_mul_mat_test!(Mat3x3, Mat3x3, Mat3x3, 3, 3, 3);
	mat_mul_mat_test!(Mat3x3, Mat4x3, Mat4x3, 3, 3, 4);
	mat_mul_mat_test!(Mat3x4, Mat2x3, Mat2x4, 3, 4, 2);
	mat_mul_mat_test!(Mat3x4, Mat3x3, Mat3x4, 3, 4, 3);
	mat_mul_mat_test!(Mat3x4, Mat4x3, Mat4x4, 3, 4, 4);
	mat_mul_mat_test!(Mat4x2, Mat2x4, Mat2x2, 4, 2, 2);
	mat_mul_mat_test!(Mat4x2, Mat3x4, Mat3x2, 4, 2, 3);
	mat_mul_mat_test!(Mat4x2, Mat4x4, Mat4x2, 4, 2, 4);
	mat_mul_mat_test!(Mat4x3, Mat2x4, Mat2x3, 4, 3, 2);
	mat_mul_mat_test!(Mat4x3, Mat3x4, Mat3x3, 4, 3, 3);
	mat_mul_mat_test!(Mat4x3, Mat4x4, Mat4x3, 4, 3, 4);
	mat_mul_mat_test!(Mat4x4, Mat2x4, Mat2x4, 4, 4, 2);
	mat_mul_mat_test!(Mat4x4, Mat3x4, Mat3x4, 4, 4, 3);
	mat_mul_mat_test!(Mat4x4, Mat4x4, Mat4x4, 4, 4, 4);
}

macro_rules! mat_one_test {
	($V:ident, $N:expr) => (
		let o = $V::<i32>::one();
		for j in 0..$N {
			for i in 0..$N {
				assert_eq!(o[(i, j)], if i == j { 1 } else { 0 });
			}
		}
	)
}

#[test]
fn one() {
	mat_one_test!(Mat2x2, 2);
	mat_one_test!(Mat3x3, 3);
	mat_one_test!(Mat4x4, 4);
}

#[test]
fn det() {
	let m = Mat2::<i32>::from_arr([11, 12, 21, 22]);
	assert_eq!(m.det(), 11*22 - 12*21);
	
	let m = Mat3::<i32>::from_arr([11, 12, 13, 21, 22, 23, 31, 32, 33]);
	assert_eq!(m.det(), 11*(22*33 - 23*32) + 12*(23*31 - 21*33) + 13*(21*32 - 22*31));
}

#[test]
fn inverse() {
	let m = Mat2::<f64>::from_arr([11.0, 12.0, 21.0, 22.0]).inverse();
	let im = Mat2::<f64>::from_arr([22.0, -12.0, -21.0, 11.0])/(11.0*22.0 - 12.0*21.0);
	let dm = m - im;
	assert!(dm[(0, 0)].abs() + dm[(0, 1)].abs() + dm[(1, 0)].abs() + dm[(1, 1)].abs() < 1e-8);
}
