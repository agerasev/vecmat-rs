use core::{
    ops::{
        Index, IndexMut,
        Neg, Add, Sub, Mul, Div, Rem,
        AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    },
    cmp::{max, min},
    iter,
};
use num_traits::{Num, Float, Zero, One};
use num_complex::Complex;
use crate::{Vector, traits::IntoComplex};


/// Polynomial up to degree of `N`.
/// 
/// The coefficients are stored reverese manner, e.g.
/// 
/// ```
/// a[0] + a[1]*x + a[2]*x^2 + a[3]*x^3 + ...
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Polynomial<T, const N: usize> {
    // Ugly workaround because we cannot write `Vector<T, N + 1>` for now.
    constant: T,
    coeffs: Vector<T, N>,
}

impl<T, const N: usize> Polynomial<T, N> {
    pub fn from_coeffs(constant: T, coeffs: Vector<T, N>) -> Self {
        Self { constant, coeffs }
    }
    pub fn into_coeffs(self) -> (T, Vector<T, N>) {
        (self.constant, self.coeffs)
    }
    pub fn as_coeffs(&self) -> (&T, &Vector<T, N>) {
        (&self.constant, &self.coeffs)
    }
    pub fn as_mut_coeffs(&mut self) -> (&mut T, &mut Vector<T, N>) {
        (&mut self.constant, &mut self.coeffs)
    }
}

impl<T, const N: usize> Index<usize> for Polynomial<T, N> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        match idx {
            0 => &self.constant,
            _ => &self.coeffs[idx + 1],
        }
    }
}
impl<T, const N: usize> IndexMut<usize> for Polynomial<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        match idx {
            0 => &mut self.constant,
            _ => &mut self.coeffs[idx + 1],
        }
    }
}
impl<T, const N: usize> IntoIterator for Polynomial<T, N> {
    type Item = T;
    type IntoIter = iter::Chain<iter::Once<T>, <Vector<T, N> as IntoIterator>::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.constant).chain(self.coeffs.into_iter())
    }
}
impl<T, const N: usize> Polynomial<T, N> {
    fn iter(&self) -> impl Iterator<Item=&T> {
        iter::once(&self.constant).chain(self.coeffs.iter())
    }
    fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        iter::once(&mut self.constant).chain(self.coeffs.iter_mut())
    }
}

impl<T: Zero, const N: usize> Zero for Polynomial<T, N> {
    fn zero() -> Self {
        Self::from_coeffs(T::zero(), Vector::zero())
    }
    fn is_zero(&self) -> bool {
        self.constant.is_zero() && self.coeffs.iter().all(|x| x.is_zero())
    }
}
impl<T: Default, const N: usize> Default for Polynomial<T, N> {
    fn default() -> Self {
        Self::from_coeffs(T::default(), Vector::default())
    }
}

impl<T: Neg<Output=T>, const N: usize> Neg for Polynomial<T, N> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::from_coeffs(-self.constant, -self.coeffs)
    }
}
impl<T: Add<Output=T>, const N: usize> Add for Polynomial<T, N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from_coeffs(self.constant + other.constant, self.coeffs + other.coeffs)
    }
}
impl<T: Sub<Output=T>, const N: usize> Sub for Polynomial<T, N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::from_coeffs(self.constant - other.constant, self.coeffs - other.coeffs)
    }
}
impl<T: AddAssign, const N: usize> AddAssign for Polynomial<T, N> {
    fn add_assign(&mut self, other: Self) {
        self.constant += other.constant;
        self.coeffs += other.coeffs;
    }
}
impl<T: SubAssign, const N: usize> SubAssign for Polynomial<T, N> {
    fn sub_assign(&mut self, other: Self) {
        self.constant -= other.constant;
        self.coeffs -= other.coeffs;
    }
}

impl<T: Zero, const N: usize> Polynomial<T, N> {
    /// The degree of the polynomial **plus one**.
    pub fn order(&self) -> usize {
        let mut o = 0;
        for (i, c) in self.iter().enumerate().rev() {
            if !c.is_zero() {
                o = i + 1;
                break;
            }
        }
        o
    }
}
impl<T: Zero + Add<Output=T> + Mul<Output=T>, const N: usize> Mul for Polynomial<T, N> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let order = self.order() + other.order();
        assert!(order < N, "The order of the product is greater than N - 1");
        let mut output = Self::zero();
        for (i, c) in output.iter_mut().take(order + 1).enumerate() {
            for j in 0..=i {
                *c = *c + self[j] * other[i - j];
            }
        }
        output
    }
}
impl<T: Zero + One, const N: usize> One for Polynomial<T, N> {
    fn one() -> Self {
        Self::from(Vector::indices().map(|i| if i == 0 { T::one() } else { T::zero() }))
    }
}
