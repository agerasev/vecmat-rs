use map::*;
use vec::*;
use mat::*;

macro_rules! affine_new_test {
	($Map:ident, $Mat:ident, $Vec:ident) => (
		let m = $Map::<f64>::new();
		assert_eq!(m.linear, $Mat::one());
		assert_eq!(m.shift, $Vec::zero());

		let m = $Mat::from_scalar(1.0);
		let v = $Vec::from_scalar(1.0);
		for v in m.iter().chain(v.iter()) {
			assert_eq!(*v, 1.0);
		}
	)
}

#[test]
fn affine_new() {
	affine_new_test!(Affine2, Mat2, Vec2);
	affine_new_test!(Affine3, Mat3, Vec3);
	affine_new_test!(Affine4, Mat4, Vec4);
}

macro_rules! affine_inverse_test {
	($Map:ident, $Mat:ident, $Vec:ident) => (
		let m = $Map::from($Mat::from_scalar(1.0) + $Mat::one(), $Vec::from_scalar(1.0));
		let v = $Vec::from_scalar(1.0);
		assert!((v - m.inverse().map_vec(m.map_vec(v))).sqrlen() < 1e-8);
		assert!((v - m.map_vec(m.inverse().map_vec(v))).sqrlen() < 1e-8);
	)
}

#[test]
fn affine_inverse() {
	affine_inverse_test!(Affine2, Mat2, Vec2);
	affine_inverse_test!(Affine3, Mat3, Vec3);
	affine_inverse_test!(Affine4, Mat4, Vec4);
}

macro_rules! affine_map_vec_test {
	($Map:ident, $Mat:ident, $Vec:ident) => (
		let m = $Map::new();
		let v = $Vec::from_scalar(1.0);
		assert_eq!(v, m.map_vec(v));
	)
}

#[test]
fn affine_map_vec() {
	affine_map_vec_test!(Affine2, Mat2, Vec2);
	affine_map_vec_test!(Affine3, Mat3, Vec3);
	affine_map_vec_test!(Affine4, Mat4, Vec4);
}

macro_rules! affine_map_mat_test {
	($Map:ident, $Mat:ident, $Vec:ident) => (
		let m = $Map::new();
		let v = $Mat::from_scalar(1.0);
		assert_eq!(v, m.map_mat(v));
	)
}

#[test]
fn affine_map_mat() {
	affine_map_mat_test!(Affine2, Mat2, Vec2);
	affine_map_mat_test!(Affine3, Mat3, Vec3);
	affine_map_mat_test!(Affine4, Mat4, Vec4);
}

macro_rules! affine_chain_test {
	($Map:ident, $Mat:ident, $Vec:ident) => (
		let m0 = $Map::from($Mat::from_scalar(1.0) + $Mat::one(), $Vec::from_scalar(1.0));
		let m1 = $Map::from($Mat::from_scalar(1.0) - $Mat::one(), $Vec::from_map(|i| i as f64));
		let v = $Vec::from_scalar(1.0);
		assert!((m0.map_vec(m1.map_vec(v)) - m0.chain(&m1).map_vec(v)).sqrlen() < 1e-8);
		assert!((m1.map_vec(m0.map_vec(v)) - m1.chain(&m0).map_vec(v)).sqrlen() < 1e-8);
	)
}

#[test]
fn affine_chain() {
	affine_chain_test!(Affine2, Mat2, Vec2);
	affine_chain_test!(Affine3, Mat3, Vec3);
	affine_chain_test!(Affine4, Mat4, Vec4);
}
