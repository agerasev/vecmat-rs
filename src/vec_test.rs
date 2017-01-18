#[allow(unused_imports)]
use vec::*;

macro_rules! n_arr {
	[$i:ident; $v:expr; $N:expr] => ({
		let $i = 0;
		let mut arr = [$v; $N];
		for $i in 1..$N {
			arr[$i] = $v;
		}
		arr
	})
}

macro_rules! vn_content_test {
	($V:ident, $N:expr) => (
		let mut v = $V { d: n_arr![i; i + 1; $N] };
		for i in 0..$N {
			assert_eq!(v.d[i], i + 1);
		}

		let z = $V { d: [0; $N] };
		for i in 0..$N {
			assert_eq!(z.d[i], 0);
		}

		for i in 0..$N {
			v.d[i] = i + 2;
		}
		for i in 0..$N {
			assert_eq!(v.d[i], i + 2);
		}
	)
}

#[test]
fn content() {
	vn_content_test!(v2, 2);
	vn_content_test!(v3, 3);
	vn_content_test!(v4, 4);
}

macro_rules! vn_copy_test {
	($V:ident, $N:expr) => (
		let v = $V::<usize> { d: n_arr![i; i + 1; $N] };
		let cv = v;

		for i in 0..$N {
			assert_eq!(cv.d[i], i + 1);
		}
	)
}

#[test]
fn copy() {
	vn_copy_test!(v2, 2);
	vn_copy_test!(v3, 3);
	vn_copy_test!(v4, 4);
}

macro_rules! vn_index_test {
	($V:ident, $N:expr) => (
		let mut v = $V::<usize> { d: n_arr![i; i + 1; $N] };
		for i in 0..$N {
			assert_eq!(v[i], i + 1);
		}
		for i in 0..$N {
			v[i] = i + 2;
		}
		for i in 0..$N {
			assert_eq!(v[i], i + 2);
		}
	)
}

#[test]
fn index() {
	vn_index_test!(v2, 2);
	vn_index_test!(v3, 3);
	vn_index_test!(v4, 4);
}

macro_rules! vn_new_test {
	($V:ident, $N:expr) => (
		let v = $V::<i32>::new();
		for i in 0..$N {
			assert_eq!(v.d[i], 0);
		}
	)
}

#[test]
fn new() {
	vn_new_test!(v2, 2);
	vn_new_test!(v3, 3);
	vn_new_test!(v4, 4);
}

macro_rules! vn_from_test {
	($V:ident, $N:expr) => (
		let vf: $V<usize> = $V::from(n_arr![i; i + 1; $N]);
		for i in 0..$N {
			assert_eq!(vf.d[i], i + 1);
		}

		let vi: $V<usize> = n_arr![i; i + 2; $N].into();
		for i in 0..$N {
			assert_eq!(vi.d[i], i + 2);
		}
	)
}

#[test]
fn from() {
	vn_from_test!(v2, 2);
	vn_from_test!(v3, 3);
	vn_from_test!(v4, 4);
}

macro_rules! vn_neg_test {
	($V:ident, $N:expr) => (
		let v: $V<i32> = n_arr![i; i as i32; $N].into();
		let nv = -v;
		for i in 0..$N {
			assert_eq!(-v.d[i], nv.d[i]);
		}
	)
}

#[test]
fn neg() {
	vn_neg_test!(v2, 2);
	vn_neg_test!(v3, 3);
	vn_neg_test!(v4, 4);
}

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a*$b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a/$b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a%$b }) }

macro_rules! vn_vec_op_test {
	($V:ident, $N:expr, $op:ident) => (
		let va: $V<i32> = n_arr![i; (2*i + 2) as i32; $N].into();
		let vb: $V<i32> = n_arr![i; (i + 1) as i32; $N].into();
		let vc = $op!(va, vb);
		for i in 0..$N {
			assert_eq!(vc.d[i], $op!(va.d[i], vb.d[i]));
		}
	)
}

#[test]
fn vec_add() {
	vn_vec_op_test!(v2, 2, op_add);
	vn_vec_op_test!(v3, 3, op_add);
	vn_vec_op_test!(v4, 4, op_add);
}

#[test]
fn vec_sub() {
	vn_vec_op_test!(v2, 2, op_sub);
	vn_vec_op_test!(v3, 3, op_sub);
	vn_vec_op_test!(v4, 4, op_sub);
}

#[test]
fn vec_mul() {
	vn_vec_op_test!(v2, 2, op_mul);
	vn_vec_op_test!(v3, 3, op_mul);
	vn_vec_op_test!(v4, 4, op_mul);
}

#[test]
fn vec_div() {
	vn_vec_op_test!(v2, 2, op_div);
	vn_vec_op_test!(v3, 3, op_div);
	vn_vec_op_test!(v4, 4, op_div);
}

#[test]
fn vec_rem() {
	vn_vec_op_test!(v2, 2, op_rem);
	vn_vec_op_test!(v3, 3, op_rem);
	vn_vec_op_test!(v4, 4, op_rem);
}

macro_rules! vn_scal_op_test {
	($V:ident, $N:expr, $op:ident) => (
		let v: $V<i32> = n_arr![i; (2*i + 1) as i32; $N].into();
		let a: i32 = 3;
		let va = $op!(v, a);
		for i in 0..$N {
			assert_eq!(va.d[i], $op!(v.d[i], a));
		}
	)
}

#[test]
fn scal_mul() {
	vn_scal_op_test!(v2, 2, op_mul);
	vn_scal_op_test!(v3, 3, op_mul);
	vn_scal_op_test!(v4, 4, op_mul);
}

#[test]
fn scal_div() {
	vn_scal_op_test!(v2, 2, op_div);
	vn_scal_op_test!(v3, 3, op_div);
	vn_scal_op_test!(v4, 4, op_div);
}

#[test]
fn scal_rem() {
	vn_scal_op_test!(v2, 2, op_rem);
	vn_scal_op_test!(v3, 3, op_rem);
	vn_scal_op_test!(v4, 4, op_rem);
}

macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! vn_vec_op_assign_test {
	($V:ident, $N:expr, $op_assign:ident, $op:ident) => (
		let va: $V<i32> = n_arr![i; (2*i + 2) as i32; $N].into();
		let vb: $V<i32> = n_arr![i; (i + 1) as i32; $N].into();
		let mut vc = va;
		$op_assign!(vc, vb);
		for i in 0..$N {
			assert_eq!(vc.d[i], $op!(va.d[i], vb.d[i]));
		}
	)
}

#[test]
fn vec_add_assign() {
	vn_vec_op_assign_test!(v2, 2, op_add_assign, op_add);
	vn_vec_op_assign_test!(v3, 3, op_add_assign, op_add);
	vn_vec_op_assign_test!(v4, 4, op_add_assign, op_add);
}

#[test]
fn vec_sub_assign() {
	vn_vec_op_assign_test!(v2, 2, op_sub_assign, op_sub);
	vn_vec_op_assign_test!(v3, 3, op_sub_assign, op_sub);
	vn_vec_op_assign_test!(v4, 4, op_sub_assign, op_sub);
}

#[test]
fn vec_mul_assign() {
	vn_vec_op_assign_test!(v2, 2, op_mul_assign, op_mul);
	vn_vec_op_assign_test!(v3, 3, op_mul_assign, op_mul);
	vn_vec_op_assign_test!(v4, 4, op_mul_assign, op_mul);
}

#[test]
fn vec_div_assign() {
	vn_vec_op_assign_test!(v2, 2, op_div_assign, op_div);
	vn_vec_op_assign_test!(v3, 3, op_div_assign, op_div);
	vn_vec_op_assign_test!(v4, 4, op_div_assign, op_div);
}

#[test]
fn vec_rem_assign() {
	vn_vec_op_assign_test!(v2, 2, op_rem_assign, op_rem);
	vn_vec_op_assign_test!(v3, 3, op_rem_assign, op_rem);
	vn_vec_op_assign_test!(v4, 4, op_rem_assign, op_rem);
}

macro_rules! vn_scal_op_assign_test {
	($V:ident, $N:expr, $op_assign:ident, $op:ident) => (
		let v: $V<i32> = n_arr![i; (2*i + 1) as i32; $N].into();
		let a: i32 = 3;
		let mut va = v;
		$op_assign!(va, a);
		for i in 0..$N {
			assert_eq!(va.d[i], $op!(v.d[i], a));
		}
	)
}

#[test]
fn scal_mul_assign() {
	vn_scal_op_assign_test!(v2, 2, op_mul_assign, op_mul);
	vn_scal_op_assign_test!(v3, 3, op_mul_assign, op_mul);
	vn_scal_op_assign_test!(v4, 4, op_mul_assign, op_mul);
}

#[test]
fn scal_div_assign() {
	vn_scal_op_assign_test!(v2, 2, op_div_assign, op_div);
	vn_scal_op_assign_test!(v3, 3, op_div_assign, op_div);
	vn_scal_op_assign_test!(v4, 4, op_div_assign, op_div);
}

#[test]
fn scal_rem_assign() {
	vn_scal_op_assign_test!(v2, 2, op_rem_assign, op_rem);
	vn_scal_op_assign_test!(v3, 3, op_rem_assign, op_rem);
	vn_scal_op_assign_test!(v4, 4, op_rem_assign, op_rem);
}

macro_rules! vn_dot_test {
	($V:ident, $N:expr) => (
		let va: $V<usize> = [1; $N].into();
		let vb: $V<usize> = n_arr![i; i + 1; $N].into();
		let c = va.dot(vb);
		assert_eq!(c, ($N*($N + 1))/2);
	)
}

#[test]
fn dot() {
	vn_dot_test!(v2, 2);
	vn_dot_test!(v3, 3);
	vn_dot_test!(v4, 4);
}

#[test]
fn cross() {
	let va: v3<i32> = [1, 0, 0].into();
	let vb: v3<i32> = [0, 1, 0].into();
	let vc = va.cross(vb);
	assert_eq!(vc.d[0], 0);
	assert_eq!(vc.d[1], 0);
	assert_eq!(vc.d[2], 1);
}
