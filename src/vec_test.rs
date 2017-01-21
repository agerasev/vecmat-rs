#[allow(unused_imports)]
use std::mem::size_of;
#[allow(unused_imports)]
use num::{Zero};
#[allow(unused_imports)]
use vec::*;


macro_rules! vn_arr {
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
		let mut v = $V { d: vn_arr![i; i + 1; $N] };
		for i in 0..$N {
			assert_eq!(v.d[i], i + 1);
		}

		let z = $V { d: [0; $N] };
		for i in 0..$N {
			assert_eq!(z.d[i], 0);
		}

		for i in 0..$N {
			v[i] = i + 2;
		}
		for i in 0..$N {
			assert_eq!(v.d[i], i + 2);
		}

		assert_eq!($N*size_of::<usize>(), size_of::<$V<usize>>());
	)
}

#[test]
fn content() {
	vn_content_test!(vec2, 2);
	vn_content_test!(vec3, 3);
	vn_content_test!(vec4, 4);
}

macro_rules! vn_eq_test {
	($V:ident, $N:expr) => (
		let va = $V::<usize> { d: vn_arr![i; i + 1; $N] };
		let vb = $V::<usize> { d: vn_arr![i; i + 1; $N] };
		assert_eq!(va, vb);
	)
}

#[test]
fn eq() {
	vn_eq_test!(vec2, 2);
	vn_eq_test!(vec3, 3);
	vn_eq_test!(vec4, 4);
}

macro_rules! vn_copy_test {
	($V:ident, $N:expr) => (
		let v = $V::<usize> { d: vn_arr![i; i + 1; $N] };
		let cv = v;
		assert_eq!(cv, v);
	)
}

#[test]
fn copy() {
	vn_copy_test!(vec2, 2);
	vn_copy_test!(vec3, 3);
	vn_copy_test!(vec4, 4);
}

macro_rules! vn_index_test {
	($V:ident, $N:expr) => (
		let mut v = $V::<usize> { d: vn_arr![i; i + 1; $N] };
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
	vn_index_test!(vec2, 2);
	vn_index_test!(vec3, 3);
	vn_index_test!(vec4, 4);
}

macro_rules! vn_new_test {
	($V:ident, $N:expr) => (
		$V::<i32>::new();
	)
}

#[test]
fn new() {
	vn_new_test!(vec2, 2);
	vn_new_test!(vec3, 3);
	vn_new_test!(vec4, 4);
}


macro_rules! vn_from_test {
	($V:ident, $N:expr) => (
		let vf: $V<usize> = $V::from(vn_arr![i; i + 1; $N]);
		for i in 0..$N {
			assert_eq!(vf[i], i + 1);
		}

		let vi: $V<usize> = vn_arr![i; i + 2; $N].into();
		for i in 0..$N {
			assert_eq!(vi[i], i + 2);
		}
	)
}

#[test]
fn from() {
	vn_from_test!(vec2, 2);
	vn_from_test!(vec3, 3);
	vn_from_test!(vec4, 4);
}

macro_rules! vn_neg_test {
	($V:ident, $N:expr) => (
		let v = vn_map![i; i as i32; $V, $N];
		let nv = -v;
		for i in 0..$N {
			assert_eq!(-v[i], nv[i]);
		}
	)
}

#[test]
fn neg() {
	vn_neg_test!(vec2, 2);
	vn_neg_test!(vec3, 3);
	vn_neg_test!(vec4, 4);
}

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a*$b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a/$b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a%$b }) }

macro_rules! vn_op_vec_test {
	($V:ident, $N:expr, $op:ident) => (
		let va = vn_map![i; (2*i + 2) as i32; $V, $N];
		let vb = vn_map![i; (i + 1) as i32; $V, $N];
		let vc = $op!(va, vb);
		for i in 0..$N {
			assert_eq!(vc[i], $op!(va[i], vb[i]));
		}
	)
}

#[test]
fn vec_add() {
	vn_op_vec_test!(vec2, 2, op_add);
	vn_op_vec_test!(vec3, 3, op_add);
	vn_op_vec_test!(vec4, 4, op_add);
}

#[test]
fn vec_sub() {
	vn_op_vec_test!(vec2, 2, op_sub);
	vn_op_vec_test!(vec3, 3, op_sub);
	vn_op_vec_test!(vec4, 4, op_sub);
}

#[test]
fn vec_mul() {
	vn_op_vec_test!(vec2, 2, op_mul);
	vn_op_vec_test!(vec3, 3, op_mul);
	vn_op_vec_test!(vec4, 4, op_mul);
}

#[test]
fn vec_div() {
	vn_op_vec_test!(vec2, 2, op_div);
	vn_op_vec_test!(vec3, 3, op_div);
	vn_op_vec_test!(vec4, 4, op_div);
}

#[test]
fn vec_rem() {
	vn_op_vec_test!(vec2, 2, op_rem);
	vn_op_vec_test!(vec3, 3, op_rem);
	vn_op_vec_test!(vec4, 4, op_rem);
}

macro_rules! vn_op_scal_test {
	($V:ident, $N:expr, $op:ident) => (
		let v = vn_map![i; (2*i + 1) as i32; $V, $N];
		let a: i32 = 3;
		let va = $op!(v, a);
		for i in 0..$N {
			assert_eq!(va[i], $op!(v[i], a));
		}
	)
}

#[test]
fn scal_mul() {
	vn_op_scal_test!(vec2, 2, op_mul);
	vn_op_scal_test!(vec3, 3, op_mul);
	vn_op_scal_test!(vec4, 4, op_mul);
}

#[test]
fn scal_div() {
	vn_op_scal_test!(vec2, 2, op_div);
	vn_op_scal_test!(vec3, 3, op_div);
	vn_op_scal_test!(vec4, 4, op_div);
}

#[test]
fn scal_rem() {
	vn_op_scal_test!(vec2, 2, op_rem);
	vn_op_scal_test!(vec3, 3, op_rem);
	vn_op_scal_test!(vec4, 4, op_rem);
}

macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! vn_op_vec_assign_test {
	($V:ident, $N:expr, $op_assign:ident, $op:ident) => (
		let va = vn_map![i; (2*i + 2) as i32; $V, $N];
		let vb = vn_map![i; (i + 1) as i32; $V, $N];
		let mut vc = va;
		$op_assign!(vc, vb);
		assert_eq!(vc, $op!(va, vb));
	)
}

#[test]
fn vec_add_assign() {
	vn_op_vec_assign_test!(vec2, 2, op_add_assign, op_add);
	vn_op_vec_assign_test!(vec3, 3, op_add_assign, op_add);
	vn_op_vec_assign_test!(vec4, 4, op_add_assign, op_add);
}

#[test]
fn vec_sub_assign() {
	vn_op_vec_assign_test!(vec2, 2, op_sub_assign, op_sub);
	vn_op_vec_assign_test!(vec3, 3, op_sub_assign, op_sub);
	vn_op_vec_assign_test!(vec4, 4, op_sub_assign, op_sub);
}

#[test]
fn vec_mul_assign() {
	vn_op_vec_assign_test!(vec2, 2, op_mul_assign, op_mul);
	vn_op_vec_assign_test!(vec3, 3, op_mul_assign, op_mul);
	vn_op_vec_assign_test!(vec4, 4, op_mul_assign, op_mul);
}

#[test]
fn vec_div_assign() {
	vn_op_vec_assign_test!(vec2, 2, op_div_assign, op_div);
	vn_op_vec_assign_test!(vec3, 3, op_div_assign, op_div);
	vn_op_vec_assign_test!(vec4, 4, op_div_assign, op_div);
}

#[test]
fn vec_rem_assign() {
	vn_op_vec_assign_test!(vec2, 2, op_rem_assign, op_rem);
	vn_op_vec_assign_test!(vec3, 3, op_rem_assign, op_rem);
	vn_op_vec_assign_test!(vec4, 4, op_rem_assign, op_rem);
}

macro_rules! vn_op_scal_assign_test {
	($V:ident, $N:expr, $op_assign:ident, $op:ident) => (
		let v = vn_map![i; (2*i + 1) as i32; $V, $N];
		let a = 3;
		let mut va = v;
		$op_assign!(va, a);
		assert_eq!(va, $op!(v, a));
	)
}

#[test]
fn scal_mul_assign() {
	vn_op_scal_assign_test!(vec2, 2, op_mul_assign, op_mul);
	vn_op_scal_assign_test!(vec3, 3, op_mul_assign, op_mul);
	vn_op_scal_assign_test!(vec4, 4, op_mul_assign, op_mul);
}

#[test]
fn scal_div_assign() {
	vn_op_scal_assign_test!(vec2, 2, op_div_assign, op_div);
	vn_op_scal_assign_test!(vec3, 3, op_div_assign, op_div);
	vn_op_scal_assign_test!(vec4, 4, op_div_assign, op_div);
}

#[test]
fn scal_rem_assign() {
	vn_op_scal_assign_test!(vec2, 2, op_rem_assign, op_rem);
	vn_op_scal_assign_test!(vec3, 3, op_rem_assign, op_rem);
	vn_op_scal_assign_test!(vec4, 4, op_rem_assign, op_rem);
}

macro_rules! vn_dot_test {
	($V:ident, $N:expr) => (
		let va: $V<usize> = [1; $N].into();
		let vb: $V<usize> = vn_map![i; i + 1; $V, $N];
		let c = va.dot(vb);
		assert_eq!(c, ($N*($N + 1))/2);
	)
}

#[test]
fn dot() {
	vn_dot_test!(vec2, 2);
	vn_dot_test!(vec3, 3);
	vn_dot_test!(vec4, 4);
}

macro_rules! vn_zero_test {
	($V:ident, $N:expr) => (
		let z = $V::<i32>::zero();
		assert_eq!(z, [0; $N].into());
		assert!(z.is_zero());
		
		let nz: $V<i32> = [1; $N].into();
		assert!(!nz.is_zero());
	)
}

#[test]
fn zero() {
	vn_zero_test!(vec2, 2);
	vn_zero_test!(vec3, 3);
	vn_zero_test!(vec4, 4);
}

macro_rules! vn_bool_not_test {
	($V:ident, $N:expr) => (
		let z = $V::<bool>::from([false; $N]);
		let nz = !z;
		for i in 0..$N {
			assert_eq!(nz[i], !z[i]);
		}
	)
}

#[test]
fn bool_not() {
	vn_bool_not_test!(vec2, 2);
	vn_bool_not_test!(vec3, 3);
	vn_bool_not_test!(vec4, 4);
}

macro_rules! vn_bool_any_test {
	($V:ident, $N:expr) => (
		let mut v: $V<bool> = [false; $N].into();
		assert!(!v.any());
		v[0] = true;
		assert!(v.any());
	)
}

#[test]
fn bool_any() {
	vn_bool_any_test!(vec2, 2);
	vn_bool_any_test!(vec3, 3);
	vn_bool_any_test!(vec4, 4);
}

macro_rules! vn_bool_all_test {
	($V:ident, $N:expr) => (
		let mut v: $V<bool> = [true; $N].into();
		assert!(v.all());
		v[0] = false;
		assert!(!v.all());
	)
}

#[test]
fn bool_all() {
	vn_bool_all_test!(vec2, 2);
	vn_bool_all_test!(vec3, 3);
	vn_bool_all_test!(vec4, 4);
}

macro_rules! vn_vec_eq_test {
	($V:ident, $N:expr) => (
		let va = vn_map![i; ($N - i) as i32; $V, $N];
		let vb = vn_map![i; i as i32; $V, $N];
		
		let eq = va.eq_(vb);
		for i in 0..$N {
			assert_eq!(eq[i], $N - i == i);
		}

		let ne = va.ne_(vb);
		for i in 0..$N {
			assert_eq!(ne[i], $N - i != i);
		}
	)
}

#[test]
fn vec_eq() {
	vn_vec_eq_test!(vec2, 2);
	vn_vec_eq_test!(vec3, 3);
	vn_vec_eq_test!(vec4, 4);
}

macro_rules! vn_vec_cmp_test {
	($V:ident, $N:expr) => (
		let va = vn_map![i; ($N - i) as i32; $V, $N];
		let vb = vn_map![i; i as i32; $V, $N];
		
		let lt = va.lt_(vb);
		for i in 0..$N {
			assert_eq!(lt[i], $N - i < i);
		}

		let le = va.le_(vb);
		for i in 0..$N {
			assert_eq!(le[i], $N - i <= i);
		}

		let gt = va.gt_(vb);
		for i in 0..$N {
			assert_eq!(gt[i], $N - i > i);
		}

		let ge = va.ge_(vb);
		for i in 0..$N {
			assert_eq!(ge[i], $N - i >= i);
		}
	)
}

#[test]
fn vec_cmp() {
	vn_vec_cmp_test!(vec2, 2);
	vn_vec_cmp_test!(vec3, 3);
	vn_vec_cmp_test!(vec4, 4);
}

#[test]
fn cross() {
	let va: vec3<i32> = [1, 0, 0].into();
	let vb: vec3<i32> = [0, 1, 0].into();
	let vc = va.cross(vb);
	assert_eq!(vc[0], 0);
	assert_eq!(vc[1], 0);
	assert_eq!(vc[2], 1);
}
