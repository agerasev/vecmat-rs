use crate::{
    matrix::Matrix2x2,
    traits::{Dot, Conj, NormL1, NormL2},
    vector::{Vector2},
};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use num_complex::Complex as NumComplex;
use num_traits::{Float, Num, One, Zero};

/// Complex number.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Complex<T> {
    vec: Vector2<T>,
}

impl<T> Complex<T> {
    pub fn new(w: T, x: T) -> Self {
        Self {
            vec: [w, x].into(),
        }
    }
    pub fn from_vector(vec: Vector2<T>) -> Self {
        Self { vec }
    }
    pub fn from_array(arr: [T; 2]) -> Self {
        Self { vec: arr.into() }
    }
    pub fn from_tuple(tup: (T, T)) -> Self {
        Self { vec: tup.into() }
    }
    pub fn into_vector(self) -> Vector2<T> {
        self.vec
    }
    pub fn into_array(self) -> [T; 2] {
        self.vec.into()
    }
    pub fn into_tuple(self) -> (T, T) {
        self.vec.into()
    }
}

impl<T> From<Vector2<T>> for Complex<T> {
    fn from(vec: Vector2<T>) -> Self {
        Self::from_vector(vec)
    }
}
impl<T> From<Complex<T>> for Vector2<T> {
    fn from(comp: Complex<T>) -> Self {
        comp.into_vector()
    }
}
impl<T> From<[T; 2]> for Complex<T> {
    fn from(arr: [T; 2]) -> Self {
        Self::from_array(arr)
    }
}
impl<T> From<Complex<T>> for [T; 2] {
    fn from(comp: Complex<T>) -> Self {
        comp.into_array()
    }
}
impl<T> From<(T, T)> for Complex<T> {
    fn from(tup: (T, T)) -> Self {
        Self::from_tuple(tup)
    }
}
impl<T> From<Complex<T>> for (T, T) {
    fn from(comp: Complex<T>) -> Self {
        comp.into_tuple()
    }
}

impl<T: Copy> Complex<T> {
    pub fn re(&self) -> T {
        self.vec.x()
    }
    pub fn im(&self) -> T {
        self.vec.y()
    }
}

impl<T> Complex<T> {
    pub fn re_ref(&self) -> &T {
        self.vec.x_ref()
    }
    pub fn im_ref(&self) -> &T {
        self.vec.y_ref()
    }
    pub fn re_mut(&mut self) -> &mut T {
        self.vec.x_mut()
    }
    pub fn im_mut(&mut self) -> &mut T {
        self.vec.y_mut()
    }
}

impl<T> Complex<T>
where
    T: Neg<Output = T> + Copy,
{
    pub fn into_matrix(self) -> Matrix2x2<T> {
        let (re, im) = self.into();
        Matrix2x2::from([[re, -im], [im, re]])
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        (-self.vec).into()
    }
}

impl<T> Complex<T>
where
    T: Neg<Output = T>,
{
    pub fn conj(self) -> Self {
        let (w, x) = self.into();
        Self::new(w, -x)
    }
}

impl<T> Conj for Complex<T> where T: Neg<Output=T> {
    fn conj(self) -> Self {
        Complex::conj(self)
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        (self.vec + other.vec).into()
    }
}
impl<T> Add<T> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        let (w, x) = self.into();
        Self::new(w + other, x)
    }
}
impl<T> Sub for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        (self.vec - other.vec).into()
    }
}
impl<T> Sub<T> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let (w, x) = self.into();
        Self::new(w - other, x)
    }
}

macro_rules! reverse_add_sub {
    ($T:ident) => {
        /// Workaround for reverse addition.
        impl Add<Complex<$T>> for $T {
            type Output = Complex<$T>;
            fn add(self, other: Complex<$T>) -> Self::Output {
                other + self
            }
        }
        /// Workaround for reverse subtraction.
        impl Sub<Complex<$T>> for $T {
            type Output = Complex<$T>;
            fn sub(self, other: Complex<$T>) -> Self::Output {
                -other + self
            }
        }
    };
}

reverse_add_sub!(f32);
reverse_add_sub!(f64);

impl<T> AddAssign for Complex<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.vec += other.vec;
    }
}
impl<T> AddAssign<T> for Complex<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: T) {
        *self.re_mut() += other;
    }
}
impl<T> SubAssign for Complex<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.vec -= other.vec;
    }
}
impl<T> SubAssign<T> for Complex<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: T) {
        *self.re_mut() -= other;
    }
}

impl<T> Zero for Complex<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
    fn is_zero(&self) -> bool {
        self.vec.is_zero()
    }
}

impl<T> Mul for Complex<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(
            self.re() * other.re() - self.im() * other.im(),
            self.re() * other.im() + self.im() * other.re(),
        )
    }
}
impl<T> Mul<T> for Complex<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, other: T) -> Self {
        (self.vec * other).into()
    }
}

impl<T> MulAssign for Complex<T>
where
    Self: Mul<Output = Self> + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
impl<T> MulAssign<T> for Complex<T>
where
    Self: Mul<T, Output = Self> + Copy,
{
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T> One for Complex<T>
where
    T: Zero + One + Sub<Output = T> + Copy,
{
    fn one() -> Self {
        Self::new(T::one(), T::zero())
    }
}

impl<T> Complex<T>
where
    T: Zero + One,
{
    pub fn i() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T> Complex<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    pub fn norm_sqr(self) -> T {
        self.vec.square_length()
    }
}
impl<T: Float> Complex<T> {
    pub fn norm(self) -> T {
        self.vec.length()
    }
    pub fn arg(self) -> T {
        self.im().atan2(self.re())
    }
    pub fn to_polar(self) -> (T, T) {
        (self.norm(), self.arg())
    }
    pub fn from_polar(r: T, theta: T) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }
}
impl<T> NormL1 for Complex<T> where Vector2<T>: NormL1<Output=T> {
    type Output = T;
    fn norm_l1(self) -> T {
        self.vec.norm_l1()
    }
}
impl<T: Float> NormL2 for Complex<T> {
    type Output = T;
    fn norm_l2(self) -> T {
        self.norm()
    }
    fn norm_l2_sqr(self) -> T {
        self.norm_sqr()
    }
}

impl<T> Div<T> for Complex<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, other: T) -> Self {
        (self.vec / other).into()
    }
}

impl<T> Complex<T>
where
    T: Float,
{
    pub fn normalize(self) -> Self {
        self / self.norm()
    }
}
impl<T> Complex<T>
where
    T: Neg<Output = T> + Num + Copy,
{
    pub fn inv(self) -> Self {
        self.conj() / self.norm_sqr()
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T> Div for Complex<T>
where
    T: Neg<Output = T> + Num + Copy,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl<T> DivAssign for Complex<T>
where
    Self: Div<Output = Self> + Copy,
{
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
impl<T> DivAssign<T> for Complex<T>
where
    Self: Div<T, Output = Self> + Copy,
{
    fn div_assign(&mut self, other: T) {
        *self = *self / other;
    }
}

macro_rules! reverse_mul_div {
    ($T:ident) => {
        /// Workaround for reverse multiplication.
        impl Mul<Complex<$T>> for $T {
            type Output = Complex<$T>;
            fn mul(self, other: Complex<$T>) -> Self::Output {
                other * self
            }
        }
        /// Workaround for reverse division.
        #[allow(clippy::suspicious_arithmetic_impl)]
        impl Div<Complex<$T>> for $T {
            type Output = Complex<$T>;
            fn div(self, other: Complex<$T>) -> Self::Output {
                self * other.inv()
            }
        }
    };
}

reverse_mul_div!(f32);
reverse_mul_div!(f64);

impl<T> Dot for Complex<T>
where
    T: Add<Output = T> + Mul<Output = T>,
{
    type Output = T;
    fn dot(self, other: Self) -> T {
        self.vec.dot(other.vec)
    }
}

impl<T> Complex<T> {
    pub fn from_num(nc: NumComplex<T>) -> Self {
        Self::new(nc.re, nc.im)
    }
    pub fn into_num(self) -> NumComplex<T> {
        let (re, im) = self.into();
        NumComplex { re, im }
    }
}
impl<T> From<NumComplex<T>> for Complex<T> {
    fn from(nc: NumComplex<T>) -> Self {
        Self::from_num(nc)
    }
}
impl<T> From<Complex<T>> for NumComplex<T> {
    fn from(comp: Complex<T>) -> Self {
        comp.into_num()
    }
}

impl<T: Num + Copy> Complex<T> {
    pub fn powu(&self, exp: u32) -> Self {
        self.into_num().powu(exp).into()
    }
}
impl<T: Neg<Output=T> + Num + Copy> Complex<T> {
    pub fn powi(&self, exp: i32) -> Self {
        self.into_num().powi(exp).into()
    }
}

impl<T: Float> Complex<T> {
    pub fn exp(self) -> Self {
        self.into_num().exp().into()
    }
    pub fn ln(self) -> Self {
        self.into_num().ln().into()
    }
    pub fn sqrt(self) -> Self {
        self.into_num().sqrt().into()
    }
    pub fn cbrt(self) -> Self {
        self.into_num().cbrt().into()
    }
    pub fn powf(self, exp: T) -> Self {
        self.into_num().powf(exp).into()
    }
    pub fn log(self, base: T) -> Self {
        self.into_num().log(base).into()
    }
    pub fn powc(self, exp: Self) -> Self {
        self.into_num().powc(exp.into_num()).into()
    }
    pub fn expf(self, base: T) -> Self {
        self.into_num().expf(base).into()
    }
    pub fn sin(self) -> Self {
        self.into_num().sin().into()
    }
    pub fn cos(self) -> Self {
        self.into_num().cos().into()
    }
    pub fn tan(self) -> Self {
        self.into_num().tan().into()
    }
    pub fn asin(self) -> Self {
        self.into_num().asin().into()
    }
    pub fn acos(self) -> Self {
        self.into_num().acos().into()
    }
    pub fn atan(self) -> Self {
        self.into_num().atan().into()
    }
    pub fn sinh(self) -> Self {
        self.into_num().sinh().into()
    }
    pub fn cosh(self) -> Self {
        self.into_num().cosh().into()
    }
    pub fn tanh(self) -> Self {
        self.into_num().tanh().into()
    }
    pub fn asinh(self) -> Self {
        self.into_num().asinh().into()
    }
    pub fn acosh(self) -> Self {
        self.into_num().acosh().into()
    }
    pub fn atanh(self) -> Self {
        self.into_num().atanh().into()
    }
    pub fn finv(self) -> Self {
        self.into_num().finv().into()
    }
    pub fn fdiv(self, other: Self) -> Self {
        self.into_num().fdiv(other.into_num()).into()
    }
}