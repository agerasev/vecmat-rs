use std::mem::size_of;
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

		let v = $V::from_map(|i| i + 1);
		for i in 0..$N {
			assert_eq!(v.d[i], i + 1);
		}

		let z = $V::from_scalar(5);
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

#[test]
fn new_no_gen() {
	let v = Vec4::<i32>::from(1, 2, 3, 4);
	for i in 0..4 {
		assert_eq!(v.d[i], i as i32 + 1);
	}

	let v = Vec4::<i32>::fromay([1, 2, 3, 4]);
	for i in 0..4 {
		assert_eq!(v.d[i], i as i32 + 1);
	}

	let v = Vec4::<i32>::fromay_ref(&[1, 2, 3, 4]);
	for i in 0..4 {
		assert_eq!(v.d[i], i as i32 + 1);
	}

	let a = [1, 2, 3, 4, 5];

	let o = Vec4::<i32>::from_slice(&a[..4]);
	assert!(o.is_some());
	let v = o.unwrap();
	for i in 0..4 {
		assert_eq!(v.d[i], i as i32 + 1);
	}

	let o = Vec4::<i32>::from_slice(&a[..3]);
	assert!(o.is_none());

	let o = Vec4::<i32>::from_slice(&a[..]);
	assert!(o.is_none());
}

macro_rules! vec_data_test {
	($V:ident, $N:expr) => (
		let mut v = $V::from_map(|i| i + 1);

		{
			let b = v.data_mut();
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
		let va = $V::from_map(|i| i + 1);
		let vb = $V::from_map(|i| i + 1);
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
		let v = $V::from_map(|i| i + 1);
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
		let mut v = $V::from_map(|i| i + 1);
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

macro_rules! vec_iter_test {
	($V:ident, $N:expr) => (
		let mut v = $V::from_map(|i| i + 1);
		for (i, c) in v.iter().enumerate() {
			assert_eq!(v[i], *c);
		}
		for (i, c) in v.iter_mut().enumerate() {
			*c = i + 2;
		}
		for i in 0..$N {
			assert_eq!(v[i], i + 2);
		}

		let mut v = $V::from_scalar(0);
		for c in &v {
			assert_eq!(*c, 0);
		}
		for c in &mut v {
			*c = 1;
		}
		for i in 0..$N {
			assert_eq!(v[i], 1);
		}
	)
}

#[test]
fn iter() {
	vec_iter_test!(Vec2, 2);
	vec_iter_test!(Vec3, 3);
	vec_iter_test!(Vec4, 4);
}

#[test]
fn fmt() {
	assert_eq!(format!("{}", Vec3::from_map(|i| i + 1)), "Vec3[1, 2, 3]");
}

macro_rules! vec_neg_test {
	($V:ident, $N:expr) => (
		let v = $V::from_map(|i| i as i32);
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
		let va = $V::from_map(|i| (2*i + 2) as i32);
		let vb = $V::from_map(|i| (i + 1) as i32);
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
		let v = $V::from_map(|i| (2*i + 1) as i32);
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

macro_rules! vec_mul_scal_rev_test {
	($V:ident, $N:expr) => (
		let v = $V::from_map(|i| (2*i + 1) as i32);
		let a: i32 = 3;
		let va = a*v;
		for i in 0..$N {
			assert_eq!(va[i], a*v[i]);
		}
	)
}

#[test]
fn scal_mul_rev() {
	vec_mul_scal_rev_test!(Vec2, 2);
	vec_mul_scal_rev_test!(Vec3, 3);
	vec_mul_scal_rev_test!(Vec4, 4);
}

macro_rules! op_add_assign { ($a:expr, $b:expr) => ({ $a += $b }) }
macro_rules! op_sub_assign { ($a:expr, $b:expr) => ({ $a -= $b }) }
macro_rules! op_mul_assign { ($a:expr, $b:expr) => ({ $a *= $b }) }
macro_rules! op_div_assign { ($a:expr, $b:expr) => ({ $a /= $b }) }
macro_rules! op_rem_assign { ($a:expr, $b:expr) => ({ $a %= $b }) }

macro_rules! vec_op_vec_assign_test {
	($V:ident, $N:expr, $op_assign:ident, $op:ident) => (
		let va = $V::from_map(|i| (2*i + 2) as i32);
		let vb = $V::from_map(|i| (i + 1) as i32);
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
		let v = $V::from_map(|i| (2*i + 1) as i32);
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
	assert_eq!(
		Vec4::from(-2, -3, -4, -5).div_mod_floor(Vec4::from(4, 3, 2, 1)), 
		(Vec4::from(-1, -1, -2, -5), Vec4::from(2, 0, 0, 0))
		);
}

macro_rules! vec_dot_test {
	($V:ident, $N:expr) => (
		let va = $V::<usize>::from_scalar(1);
		let vb = $V::<usize>::from_map(|i| i + 1);
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

macro_rules! vec_norm_test {
	($V:ident, $N:expr) => (
		assert_eq!($V::from_scalar(2).abs2(), $N*4);
	)
}

#[test]
fn norm() {
	vec_norm_test!(Vec2, 2);
	vec_norm_test!(Vec3, 3);
	vec_norm_test!(Vec4, 4);
}

macro_rules! vec_zero_test {
	($V:ident, $N:expr) => (
		let z = $V::<i32>::zero();
		assert_eq!(z, $V::from_scalar(0));
		assert!(z.is_zero());
		
		let nz: $V<i32> = $V::from_scalar(1);
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
		let z = $V::from_scalar(false);
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
		let mut v = $V::from_scalar(false);
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
		let mut v = $V::from_scalar(true);
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

macro_rules! vec_veq_test {
	($V:ident, $N:expr) => (
		let va = $V::from_map(|i| ($N - i) as i32);
		let vb = $V::from_map(|i| i as i32);
		
		let eq = va.veq(vb);
		for i in 0..$N {
			assert_eq!(eq[i], $N - i == i);
		}

		let ne = va.vne(vb);
		for i in 0..$N {
			assert_eq!(ne[i], $N - i != i);
		}
	)
}

#[test]
fn vec_eq() {
	vec_veq_test!(Vec2, 2);
	vec_veq_test!(Vec3, 3);
	vec_veq_test!(Vec4, 4);
}

macro_rules! vec_vcmp_test {
	($V:ident, $N:expr) => (
		let va = $V::from_map(|i| ($N - i) as i32);
		let vb = $V::from_map(|i| i as i32);
		
		let lt = va.vlt(vb);
		for i in 0..$N {
			assert_eq!(lt[i], $N - i < i);
		}

		let le = va.vle(vb);
		for i in 0..$N {
			assert_eq!(le[i], $N - i <= i);
		}

		let gt = va.vgt(vb);
		for i in 0..$N {
			assert_eq!(gt[i], $N - i > i);
		}

		let ge = va.vge(vb);
		for i in 0..$N {
			assert_eq!(ge[i], $N - i >= i);
		}
	)
}

#[test]
fn vec_vcmp() {
	vec_vcmp_test!(Vec2, 2);
	vec_vcmp_test!(Vec3, 3);
	vec_vcmp_test!(Vec4, 4);
}

#[test]
fn cross() {
	let va = Vec3::<i32>::from(1, 0, 0);
	let vb = Vec3::<i32>::from(0, 1, 0);
	let vc = va.cross(vb);
	assert_eq!(vc[0], 0);
	assert_eq!(vc[1], 0);
	assert_eq!(vc[2], 1);
}
