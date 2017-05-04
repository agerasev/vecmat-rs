#[allow(unused_imports)]
use std::mem::size_of;
#[allow(unused_imports)]
use vec::*;

macro_rules! vec_content_test {
	($V:ident, $N:expr) => (
		let mut v = $V { d: [0; $N] };
		for i in 0..$N {
			v.d[i] = i + 2;
		}
		for i in 0..$N {
			assert_eq!(v.d[i], i + 2);
		}

		assert_eq!($N*size_of::<usize>(), size_of::<$V<usize>>());
	)
}

#[test]
fn content() {
	vec_content_test!(Vec2, 2);
	vec_content_test!(Vec3, 3);
	vec_content_test!(Vec4, 4);
}

macro_rules! vec_new_test {
	($V:ident, $N:expr) => (
		let v = $V::<i32>::new();
		for i in 0..$N {
			assert_eq!(v.d[i], i32::default());
		}

		let v = $V::new_map(|i| i + 1);
		for i in 0..$N {
			assert_eq!(v.d[i], i + 1);
		}

		let z = $V::new_scal(5);
		for i in 0..$N {
			assert_eq!(z.d[i], 5);
		}
	)
}

#[test]
fn new() {
	vec_new_test!(Vec2, 2);
	vec_new_test!(Vec3, 3);
	vec_new_test!(Vec4, 4);
}

macro_rules! vec_data_test {
	($V:ident, $N:expr) => (
		let mut v = $V::new_map(|i| i + 1);

		{
			let mut b = v.data_mut(); 
			for i in 0..$N {
				b[i] = i + 3;
			}
		}

		let a = &v.d;
		{
			let b = v.data(); 
			for i in 0..$N {
				assert_eq!(a[i], b[i]);
			}
		}
	)
}

#[test]
fn data() {
	vec_data_test!(Vec2, 2);
	vec_data_test!(Vec3, 3);
	vec_data_test!(Vec4, 4);
}

macro_rules! vec_eq_test {
	($V:ident, $N:expr) => (
		let va = $V::new_map(|i| i + 1);
		let vb = $V::new_map(|i| i + 1);
		assert_eq!(va, vb);
	)
}

#[test]
fn eq() {
	vec_eq_test!(Vec2, 2);
	vec_eq_test!(Vec3, 3);
	vec_eq_test!(Vec4, 4);
}

macro_rules! vec_copy_test {
	($V:ident, $N:expr) => (
		let v = $V::new_map(|i| i + 1);
		let cv = v;
		assert_eq!(cv, v);
	)
}

#[test]
fn copy() {
	vec_copy_test!(Vec2, 2);
	vec_copy_test!(Vec3, 3);
	vec_copy_test!(Vec4, 4);
}

macro_rules! vec_index_test {
	($V:ident, $N:expr) => (
		let mut v = $V::new_map(|i| i + 1);
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
	vec_index_test!(Vec2, 2);
	vec_index_test!(Vec3, 3);
	vec_index_test!(Vec4, 4);
}

#[test]
fn fmt() {
	assert_eq!(format!("{}", Vec3::new_map(|i| i + 1)), "Vec3[1, 2, 3]");
}

macro_rules! vec_neg_test {
	($V:ident, $N:expr) => (
		let v = $V::new_map(|i| i as i32);
		let nv = -v;
		for i in 0..$N {
			assert_eq!(-v[i], nv[i]);
		}
	)
}

#[test]
fn neg() {
	vec_neg_test!(Vec2, 2);
	vec_neg_test!(Vec3, 3);
	vec_neg_test!(Vec4, 4);
}

macro_rules! op_add { ($a:expr, $b:expr) => ({ $a + $b }) }
macro_rules! op_sub { ($a:expr, $b:expr) => ({ $a - $b }) }
macro_rules! op_mul { ($a:expr, $b:expr) => ({ $a*$b }) }
macro_rules! op_div { ($a:expr, $b:expr) => ({ $a/$b }) }
macro_rules! op_rem { ($a:expr, $b:expr) => ({ $a%$b }) }

macro_rules! vec_op_vec_test {
	($V:ident, $N:expr, $op:ident) => (
		let va = $V::new_map(|i| (2*i + 2) as i32);
		let vb = $V::new_map(|i| (i + 1) as i32);
		let vc = $op!(va, vb);
		for i in 0..$N {
			assert_eq!(vc[i], $op!(va[i], vb[i]));
		}
	)
}

#[test]
fn vec_add() {
	vec_op_vec_test!(Vec2, 2, op_add);
	vec_op_vec_test!(Vec3, 3, op_add);
	vec_op_vec_test!(Vec4, 4, op_add);
}

#[test]
fn vec_sub() {
	vec_op_vec_test!(Vec2, 2, op_sub);
	vec_op_vec_test!(Vec3, 3, op_sub);
	vec_op_vec_test!(Vec4, 4, op_sub);
}

#[test]
fn vec_mul() {
	vec_op_vec_test!(Vec2, 2, op_mul);
	vec_op_vec_test!(Vec3, 3, op_mul);
	vec_op_vec_test!(Vec4, 4, op_mul);
}

#[test]
fn vec_div() {
	vec_op_vec_test!(Vec2, 2, op_div);
	vec_op_vec_test!(Vec3, 3, op_div);
	vec_op_vec_test!(Vec4, 4, op_div);
}

#[test]
fn vec_rem() {
	vec_op_vec_test!(Vec2, 2, op_rem);
	vec_op_vec_test!(Vec3, 3, op_rem);
	vec_op_vec_test!(Vec4, 4, op_rem);
}

macro_rules! vec_op_scal_test {
	($V:ident, $N:expr, $op:ident) => (
		let v = $V::new_map(|i| (2*i + 1) as i32);
		let a: i32 = 3;
		let va = $op!(v, a);
		for i in 0..$N {
			assert_eq!(va[i], $op!(v[i], a));
		}
	)
}

#[test]
fn scal_mul() {
	vec_op_scal_test!(Vec2, 2, op_mul);
	vec_op_scal_test!(Vec3, 3, op_mul);
	vec_op_scal_test!(Vec4, 4, op_mul);
}

#[test]
fn scal_div() {
	vec_op_scal_test!(Vec2, 2, op_div);
	vec_op_scal_test!(Vec3, 3, op_div);
	vec_op_scal_test!(Vec4, 4, op_div);
}

#[test]
fn scal_rem() {
	vec_op_scal_test!(Vec2, 2, op_rem);
	vec_op_scal_test!(Vec3, 3, op_rem);
	vec_op_scal_test!(Vec4, 4, op_rem);
}

macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! vec_op_vec_assign_test {
	($V:ident, $N:expr, $op_assign:ident, $op:ident) => (
		let va = $V::new_map(|i| (2*i + 2) as i32);
		let vb = $V::new_map(|i| (i + 1) as i32);
		let mut vc = va;
		$op_assign!(vc, vb);
		assert_eq!(vc, $op!(va, vb));
	)
}

#[test]
fn vec_add_assign() {
	vec_op_vec_assign_test!(Vec2, 2, op_add_assign, op_add);
	vec_op_vec_assign_test!(Vec3, 3, op_add_assign, op_add);
	vec_op_vec_assign_test!(Vec4, 4, op_add_assign, op_add);
}

#[test]
fn vec_sub_assign() {
	vec_op_vec_assign_test!(Vec2, 2, op_sub_assign, op_sub);
	vec_op_vec_assign_test!(Vec3, 3, op_sub_assign, op_sub);
	vec_op_vec_assign_test!(Vec4, 4, op_sub_assign, op_sub);
}

#[test]
fn vec_mul_assign() {
	vec_op_vec_assign_test!(Vec2, 2, op_mul_assign, op_mul);
	vec_op_vec_assign_test!(Vec3, 3, op_mul_assign, op_mul);
	vec_op_vec_assign_test!(Vec4, 4, op_mul_assign, op_mul);
}

#[test]
fn vec_div_assign() {
	vec_op_vec_assign_test!(Vec2, 2, op_div_assign, op_div);
	vec_op_vec_assign_test!(Vec3, 3, op_div_assign, op_div);
	vec_op_vec_assign_test!(Vec4, 4, op_div_assign, op_div);
}

#[test]
fn vec_rem_assign() {
	vec_op_vec_assign_test!(Vec2, 2, op_rem_assign, op_rem);
	vec_op_vec_assign_test!(Vec3, 3, op_rem_assign, op_rem);
	vec_op_vec_assign_test!(Vec4, 4, op_rem_assign, op_rem);
}

macro_rules! vec_op_scal_assign_test {
	($V:ident, $N:expr, $op_assign:ident, $op:ident) => (
		let v = $V::new_map(|i| (2*i + 1) as i32);
		let a = 3;
		let mut va = v;
		$op_assign!(va, a);
		assert_eq!(va, $op!(v, a));
	)
}

#[test]
fn scal_mul_assign() {
	vec_op_scal_assign_test!(Vec2, 2, op_mul_assign, op_mul);
	vec_op_scal_assign_test!(Vec3, 3, op_mul_assign, op_mul);
	vec_op_scal_assign_test!(Vec4, 4, op_mul_assign, op_mul);
}

#[test]
fn scal_div_assign() {
	vec_op_scal_assign_test!(Vec2, 2, op_div_assign, op_div);
	vec_op_scal_assign_test!(Vec3, 3, op_div_assign, op_div);
	vec_op_scal_assign_test!(Vec4, 4, op_div_assign, op_div);
}

#[test]
fn scal_rem_assign() {
	vec_op_scal_assign_test!(Vec2, 2, op_rem_assign, op_rem);
	vec_op_scal_assign_test!(Vec3, 3, op_rem_assign, op_rem);
	vec_op_scal_assign_test!(Vec4, 4, op_rem_assign, op_rem);
}

#[test]
fn div_mod_floor() {
	assert_eq!(Vec4::new_data(&[-2, -3, -4, -5]).div_mod_floor(Vec4::new_data(&[4, 3, 2, 1])), (Vec4::new_data(&[-1, -1, -2, -5]),Vec4::new_data(&[2, 0, 0, 0])));
}

/*
macro_rules! vec_dot_test {
	($V:ident, $N:expr) => (
		let va: $V<usize> = [1; $N].into();
		let vb: $V<usize> = vec_map![i; i + 1; $V, $N];
		let c = va.dot(vb);
		assert_eq!(c, ($N*($N + 1))/2);
	)
}

#[test]
fn dot() {
	vec_dot_test!(Vec2, 2);
	vec_dot_test!(Vec3, 3);
	vec_dot_test!(Vec4, 4);
}

macro_rules! vec_zero_test {
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
	vec_zero_test!(Vec2, 2);
	vec_zero_test!(Vec3, 3);
	vec_zero_test!(Vec4, 4);
}

macro_rules! vec_bool_not_test {
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
	vec_bool_not_test!(Vec2, 2);
	vec_bool_not_test!(Vec3, 3);
	vec_bool_not_test!(Vec4, 4);
}

macro_rules! vec_bool_any_test {
	($V:ident, $N:expr) => (
		let mut v: $V<bool> = [false; $N].into();
		assert!(!v.any());
		v[0] = true;
		assert!(v.any());
	)
}

#[test]
fn bool_any() {
	vec_bool_any_test!(Vec2, 2);
	vec_bool_any_test!(Vec3, 3);
	vec_bool_any_test!(Vec4, 4);
}

macro_rules! vec_bool_all_test {
	($V:ident, $N:expr) => (
		let mut v: $V<bool> = [true; $N].into();
		assert!(v.all());
		v[0] = false;
		assert!(!v.all());
	)
}

#[test]
fn bool_all() {
	vec_bool_all_test!(Vec2, 2);
	vec_bool_all_test!(Vec3, 3);
	vec_bool_all_test!(Vec4, 4);
}

macro_rules! vec_vec_eq_test {
	($V:ident, $N:expr) => (
		let va = vec_map![i; ($N - i) as i32; $V, $N];
		let vb = vec_map![i; i as i32; $V, $N];
		
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
	vec_vec_eq_test!(Vec2, 2);
	vec_vec_eq_test!(Vec3, 3);
	vec_vec_eq_test!(Vec4, 4);
}

macro_rules! vec_vec_cmp_test {
	($V:ident, $N:expr) => (
		let va = vec_map![i; ($N - i) as i32; $V, $N];
		let vb = vec_map![i; i as i32; $V, $N];
		
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
	vec_vec_cmp_test!(Vec2, 2);
	vec_vec_cmp_test!(Vec3, 3);
	vec_vec_cmp_test!(Vec4, 4);
}

#[test]
fn cross() {
	let va: Vec3<i32> = [1, 0, 0].into();
	let vb: Vec3<i32> = [0, 1, 0].into();
	let vc = va.cross(vb);
	assert_eq!(vc[0], 0);
	assert_eq!(vc[1], 0);
	assert_eq!(vc[2], 1);
}
*/
