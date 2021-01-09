use core::{
    ops::{
        Index, IndexMut,
        Neg, Add, Sub, Mul, Div, Rem,
        AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    },
    cmp::{max, min},
};
use num_traits::{Num, Float, Zero, One};
use num_complex::Complex;
use crate::{Vector, traits::IntoComplex};


/// Polynomial up to order of `N - 1`.
/// 
/// The coefficients are stored reverese manner, e.g.
/// 
/// ```
/// a[0] + a[1]*x + a[2]*x^2 + a[3]*x^3 + ...
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Polynomial<T, const N: usize> {
    coeffs: Vector<T, N>,
}

impl<T, const N: usize> Polynomial<T, N> {
    pub fn from_array(array: [T; N]) -> Self {
        Self { coeffs: array.into() }
    }
    pub fn into_array(self) -> [T; N] {
        self.coeffs.into_array()
    }
    pub fn as_array(&self) -> &[T; N] {
        self.coeffs.as_array()
    }
    pub fn as_mut_array(&mut self) -> &mut [T; N] {
        self.coeffs.as_mut_array()
    }
    pub fn from_vector(coeffs: Vector<T, N>) -> Self {
        Self { coeffs }
    }
    pub fn into_vector(self) -> Vector<T, N> {
        self.coeffs
    }
    pub fn as_vector(&self) -> &Vector<T, N> {
        &self.coeffs
    }
    pub fn as_mut_vector(&mut self) -> &mut Vector<T, N> {
        &mut self.coeffs
    }
}

impl<T, const N: usize> From<[T; N]> for Polynomial<T, N> {
    fn from(coeffs: [T; N]) -> Self {
        Self::from_array(coeffs)
    }
}
impl<T, const N: usize> Into<[T; N]> for Polynomial<T, N> {
    fn into(self) -> [T; N] {
        self.into_array()
    }
}
impl<T, const N: usize> AsRef<[T; N]> for Polynomial<T, N> {
    fn as_ref(&self) -> &[T; N] {
        self.as_array()
    }
}
impl<T, const N: usize> AsMut<[T; N]> for Polynomial<T, N> {
    fn as_mut(&mut self) -> &mut [T; N] {
        self.as_mut_array()
    }
}

impl<T, const N: usize> From<Vector<T, N>> for Polynomial<T, N> {
    fn from(coeffs: Vector<T, N>) -> Self {
        Self::from_vector(coeffs)
    }
}
impl<T, const N: usize> Into<Vector<T, N>> for Polynomial<T, N> {
    fn into(self) -> Vector<T, N> {
        self.into_vector()
    }
}
impl<T, const N: usize> AsRef<Vector<T, N>> for Polynomial<T, N> {
    fn as_ref(&self) -> &Vector<T, N> {
        self.as_vector()
    }
}
impl<T, const N: usize> AsMut<Vector<T, N>> for Polynomial<T, N> {
    fn as_mut(&mut self) -> &mut Vector<T, N> {
        self.as_mut_vector()
    }
}

impl<T, const N: usize> Index<usize> for Polynomial<T, N> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        &self.coeffs[idx]
    }
}
impl<T, const N: usize> IndexMut<usize> for Polynomial<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.coeffs[idx]
    }
}
impl<T, const N: usize> IntoIterator for Polynomial<T, N> {
    type Item = T;
    type IntoIter = <Vector<T, N> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.coeffs.into_iter()
    }
}
impl<T, const N: usize> Polynomial<T, N> {
    fn iter(&self) -> impl Iterator<Item=&T> {
        self.coeffs.iter()
    }
    fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.coeffs.iter_mut()
    }
}

impl<T: Zero, const N: usize> Zero for Polynomial<T, N> {
    fn zero() -> Self {
        Self::from(Vector::zero())
    }
    fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|x| x.is_zero())
    }
}
impl<T: Default, const N: usize> Default for Polynomial<T, N> {
    fn default() -> Self {
        Self::from(Vector::default())
    }
}

impl<T: Neg<Output=T>, const N: usize> Neg for Polynomial<T, N> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::from(-self.coeffs)
    }
}
impl<T: Add<Output=T>, const N: usize> Add for Polynomial<T, N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from(self.coeffs + other.coeffs)
    }
}
impl<T: Sub<Output=T>, const N: usize> Sub for Polynomial<T, N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::from(self.coeffs - other.coeffs)
    }
}
impl<T: AddAssign, const N: usize> AddAssign for Polynomial<T, N> {
    fn add_assign(&mut self, other: Self) {
        self.coeffs += other.coeffs;
    }
}
impl<T: SubAssign, const N: usize> SubAssign for Polynomial<T, N> {
    fn sub_assign(&mut self, other: Self) {
        self.coeffs -= other.coeffs;
    }
}

impl<T: Zero, const N: usize> Polynomial<T, N> {
    pub fn order(&self) -> usize {
        let mut o = 0;
        for (i, c) in self.coeffs.iter().enumerate().rev() {
            if !c.is_zero() {
                o = i;
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
