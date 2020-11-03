use num_traits::{Zero, One};
use approx::*;
use crate::*;


macro_rules! affine_identity_test { ($X:ident, $W:ident, $V:ident) => (
	let m = $X::<f64>::identity();
	assert_abs_diff_eq!(Into::<$W<_>>::into(m.linear()), $W::one());
	assert_abs_diff_eq!(Into::<$V<_>>::into(m.shift()), $V::zero());
) }
#[test]
fn affine_identity() {
	affine_identity_test!(Affine2, Matrix2x2, Vector2);
	affine_identity_test!(Affine3, Matrix3x3, Vector3);
	affine_identity_test!(Affine4, Matrix4x4, Vector4);
}
/*
macro_rules! affine_inverse_test {
	($X:ident, $W:ident, $V:ident) => (
		let m = $X::from($W::from_scalar(1.0) + $W::one(), $V::from_scalar(1.0));
		let v = $V::from_scalar(1.0);
		assert!((v - m.inverse().map_vec(m.map_vec(v))).sqrlen() < 1e-8);
		assert!((v - m.map_vec(m.inverse().map_vec(v))).sqrlen() < 1e-8);
	)
}
#[test]
fn affine_inverse() {
	affine_inverse_test!(Affine2, Matrix2x2, Vector2);
	affine_inverse_test!(Affine3, Matrix3x3, Vector3);
	affine_inverse_test!(Affine4, Matrix4x4, Vector4);
}

macro_rules! affine_map_vec_test {
	($X:ident, $W:ident, $V:ident) => (
		let m = $X::new();
		let v = $V::from_scalar(1.0);
		assert_eq!(v, m.map_vec(v));
	)
}

#[test]
fn affine_map_vec() {
	affine_map_vec_test!(Affine2, Matrix2x2, Vector2);
	affine_map_vec_test!(Affine3, Matrix3x3, Vector3);
	affine_map_vec_test!(Affine4, Matrix4x4, Vector4);
}

macro_rules! affine_map_mat_test {
	($X:ident, $W:ident, $V:ident) => (
		let m = $X::new();
		let v = $W::from_scalar(1.0);
		assert_eq!(v, m.map_mat(v));
	)
}

#[test]
fn affine_map_mat() {
	affine_map_mat_test!(Affine2, Matrix2x2, Vector2);
	affine_map_mat_test!(Affine3, Matrix3x3, Vector3);
	affine_map_mat_test!(Affine4, Matrix4x4, Vector4);
}

macro_rules! affine_chain_test {
	($X:ident, $W:ident, $V:ident) => (
		let m0 = $X::from($W::from_scalar(1.0) + $W::one(), $V::from_scalar(1.0));
		let m1 = $X::from($W::from_scalar(1.0) - $W::one(), $V::from_map(|i| i as f64));
		let v = $V::from_scalar(1.0);
		assert!((m0.map_vec(m1.map_vec(v)) - m0.chain(&m1).map_vec(v)).sqrlen() < 1e-8);
		assert!((m1.map_vec(m0.map_vec(v)) - m1.chain(&m0).map_vec(v)).sqrlen() < 1e-8);
	)
}

#[test]
fn affine_chain() {
	affine_chain_test!(Affine2, Matrix2x2, Vector2);
	affine_chain_test!(Affine3, Matrix3x3, Vector3);
	affine_chain_test!(Affine4, Matrix4x4, Vector4);
}
*/
