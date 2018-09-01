use map::*;
use vec::*;
use mat::*;

macro_rules! affine_new_test {
	($Map:ident, $Mat:ident, $Vec:ident) => (
		let m = $Map::<f64>::new();
		assert_eq!(m.linear, $Mat::one());
		assert_eq!(m.shift, $Vec::zero());

		let m = $Mat::from_scal(1.0);
		let v = $Vec::from_scal(1.0);
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

macro_rules! affine_map_test {
	($Map:ident, $Mat:ident, $Vec:ident) => (
		let m = $Map::new();
		let v = $Vec::from_scal(1.0);
		assert_eq!(v, m.map(v));
	)
}

#[test]
fn affine_map() {
	affine_map_test!(Affine2, Mat2, Vec2);
	affine_map_test!(Affine3, Mat3, Vec3);
	affine_map_test!(Affine4, Mat4, Vec4);
}

macro_rules! affine_inverse_test {
	($Map:ident, $Mat:ident, $Vec:ident) => (
		let m = $Map::from($Mat::from_scal(1.0) + $Mat::one(), $Vec::from_scal(1.0));
		let v = $Vec::from_scal(1.0);
		assert!((v - m.inverse().map(m.map(v))).abs2() < 1e-4);
		assert!((v - m.map(m.inverse().map(v))).abs2() < 1e-4);
	)
}

#[test]
fn affine_inverse() {
	affine_inverse_test!(Affine2, Mat2, Vec2);
	affine_inverse_test!(Affine3, Mat3, Vec3);
	affine_inverse_test!(Affine4, Mat4, Vec4);
}
