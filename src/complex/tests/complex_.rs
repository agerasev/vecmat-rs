use crate::*;
use ::approx::*;
use num_traits::{One, Zero};

#[test]
fn new() {
    let q = Complex::<f32>::new(1.0, 2.0);
    assert_abs_diff_eq!(q.re(), 1.0);
    assert_abs_diff_eq!(q.im(), 2.0);
}

#[test]
fn conj() {
    let c = Complex::<f32>::new(1.0, 2.0).conj();
    assert_abs_diff_eq!(c, Complex::new(1.0, -2.0));
}

#[test]
fn add() {
    let a = Complex::<f32>::new(1.0, 2.0);
    let b = Complex::<f32>::new(5.0, 6.0);
    let c = a + b;
    assert_abs_diff_eq!(c, Complex::new(6.0, 8.0));
}

#[test]
fn sub() {
    let a = Complex::<f32>::new(4.0, 3.0);
    let b = Complex::<f32>::new(5.0, 6.0);
    let c = a - b;
    assert_abs_diff_eq!(c, Complex::new(-1.0, -3.0));
}

#[test]
fn abs() {
    let q = Complex::<f32>::new(1.0, 2.0);
    assert_abs_diff_eq!(q.norm_sqr(), 5.0);
    assert_abs_diff_eq!(q.norm(), 5.0f32.sqrt());
}

#[test]
fn mul() {
    let a = Complex::<f32>::new(1.0, 2.0);
    let b = Complex::<f32>::new(5.0, 6.0);
    let c = Complex::new(-7.0, 16.0);
    assert_abs_diff_eq!(a * b, c);
    assert_abs_diff_eq!(b * a, c);
}

#[test]
fn mul_scal() {
    let a = Complex::<f32>::new(1.0, 2.0);
    assert_abs_diff_eq!(2.0 * a, Complex::new(2.0, 4.0));
}

#[test]
fn inv() {
    let a = 1.0 / Complex::<f32>::new(1.0, 2.0);
    assert_abs_diff_eq!(a, Complex::new(1.0, -2.0) / 5.0);
}

#[test]
fn div() {
    let a = Complex::<f32>::new(4.0, 3.0);
    let b = Complex::<f32>::new(1.0, 2.0);
    assert_abs_diff_eq!(a / b, (a * b.conj()) / b.norm_sqr());
}

#[test]
fn zero() {
    let a = Complex::<f32>::zero();
    assert_abs_diff_eq!(a, Complex::new(0.0, 0.0));
}

#[test]
fn one() {
    let a = Complex::<f32>::one();
    assert_abs_diff_eq!(a, Complex::new(1.0, 0.0));
}
