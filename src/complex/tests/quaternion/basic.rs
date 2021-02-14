use crate::*;
use ::approx::*;
use num_traits::{One, Zero};

#[test]
fn new() {
    let q = Quaternion::<f32>::new(1.0, 2.0, 3.0, 4.0);
    assert_abs_diff_eq!(q.w(), 1.0);
    assert_abs_diff_eq!(q.x(), 2.0);
    assert_abs_diff_eq!(q.y(), 3.0);
    assert_abs_diff_eq!(q.z(), 4.0);
}

#[test]
fn conj() {
    let c = Quaternion::<f32>::new(1.0, 2.0, 3.0, 4.0).conj();
    assert_abs_diff_eq!(c, Quaternion::new(1.0, -2.0, -3.0, -4.0));
}

#[test]
fn add() {
    let a = Quaternion::<f32>::new(1.0, 2.0, 3.0, 4.0);
    let b = Quaternion::<f32>::new(5.0, 6.0, 7.0, 8.0);
    let c = a + b;
    assert_abs_diff_eq!(c, Quaternion::new(6.0, 8.0, 10.0, 12.0));
}

#[test]
fn sub() {
    let a = Quaternion::<f32>::new(4.0, 3.0, 2.0, 1.0);
    let b = Quaternion::<f32>::new(5.0, 6.0, 7.0, 8.0);
    let c = a - b;
    assert_abs_diff_eq!(c, Quaternion::new(-1.0, -3.0, -5.0, -7.0));
}

#[test]
fn abs() {
    let q = Quaternion::<f32>::new(1.0, 2.0, 3.0, 4.0);
    assert_abs_diff_eq!(q.norm_sqr(), 30.0);
    assert_abs_diff_eq!(q.norm(), 30.0f32.sqrt());
}

#[test]
fn mul() {
    let a = Quaternion::<f32>::new(1.0, 2.0, 3.0, 4.0);
    let b = Quaternion::<f32>::new(5.0, 6.0, 7.0, 8.0);
    assert_abs_diff_eq!(a * b, Quaternion::new(-60.0, 12.0, 30.0, 24.0));
    assert_abs_diff_eq!(b * a, Quaternion::new(-60.0, 20.0, 14.0, 32.0));
}

#[test]
fn mul_scal() {
    let q = Quaternion::<f32>::new(1.0, 2.0, 3.0, 4.0);
    let test_func = |a: Quaternion<f32>| {
        assert_abs_diff_eq!(a, Quaternion::new(2.0, 4.0, 6.0, 8.0));
    };
    let c = 2.0;
    test_func(q * c);
    test_func(c * q);
    let cq = Quaternion::<f32>::new(c, 0.0, 0.0, 0.0);
    test_func(q * cq);
    test_func(cq * q);
}

#[test]
fn mul_comp() {
    let q = Quaternion::<f32>::new(1.0, 2.0, 3.0, 4.0);
    let right_test_func = |a: Quaternion<f32>| {
        assert_abs_diff_eq!(a, Quaternion::new(-3.0, 4.0, 11.0, -2.0));
    };
    let left_test_func = |a: Quaternion<f32>| {
        assert_abs_diff_eq!(a, Quaternion::new(-3.0, 4.0, -5.0, 10.0));
    };
    let c = Complex::<f32>::new(1.0, 2.0);
    right_test_func(q * c);
    left_test_func(c * q);
    let cq = Quaternion::<f32>::new(c.re(), c.im(), 0.0, 0.0);
    right_test_func(q * cq);
    left_test_func(cq * q);
}

#[test]
fn inv() {
    let a = 1.0 / Quaternion::<f32>::new(1.0, 2.0, -1.0, -2.0);
    assert_abs_diff_eq!(a, Quaternion::new(1.0, -2.0, 1.0, 2.0) / 10.0);
}

#[test]
fn div() {
    let a = Quaternion::<f32>::new(4.0, 3.0, 2.0, 1.0);
    let b = Quaternion::<f32>::new(1.0, 2.0, 3.0, 4.0);
    assert_abs_diff_eq!(a / b, (a * b.conj()) / b.norm_sqr());
}

#[test]
fn div_rev() {
    let q = Quaternion::<f32>::new(1.0, 2.0, -1.0, -2.0);
    let s = 2.0;
    assert_abs_diff_eq!(s / q, q.conj() / 5.0);
    let c = Complex::<f32>::new(1.0, 2.0);
    assert_abs_diff_eq!(c / q, c * q.conj() / 10.0);
}

#[test]
fn zero() {
    let a = Quaternion::<f32>::zero();
    assert_abs_diff_eq!(a, Quaternion::new(0.0, 0.0, 0.0, 0.0));
}

#[test]
fn one() {
    let a = Quaternion::<f32>::one();
    assert_abs_diff_eq!(a, Quaternion::new(1.0, 0.0, 0.0, 0.0));
}
