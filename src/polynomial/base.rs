use core::{
    ops::{Index, IndexMut},
    iter::{self, FromIterator},
};
use num_traits::Zero;
use crate::Vector;
use super::iter::*;


/// Polynomial up to degree of `N`.
/// 
/// The coefficients are stored reverese manner, e.g.
/// `a[0] + a[1]*x + a[2]*x^2 + a[3]*x^3 + ...`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Polynomial<T, const N: usize> {
    // Ugly workaround because we cannot write `Vector<T, N + 1>` for now.
    pub(super) constant: T,
    pub(super) coeffs: Vector<T, N>,
}

impl<T, const N: usize> Polynomial<T, N> {
    pub(super) fn new(constant: T, coeffs: Vector<T, N>) -> Self {
        Self { constant, coeffs }
    }
    pub fn from_iter_exact(iter: &mut impl Iterator<Item=T>) -> Option<Self> {
        Some(Self {
            constant: iter.next()?,
            coeffs: Vector::try_from_iter(iter)?,
        })
    }
}
impl<T: Zero, const N: usize> FromIterator<T> for Polynomial<T, N> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        Self::from_iter_exact(&mut iter.into_iter().chain(iter::repeat_with(T::zero))).unwrap()
    }
}

impl<T, const N: usize> IntoIterator for Polynomial<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.constant, self.coeffs)
    }
}
impl<T, const N: usize> Polynomial<T, N> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(&self.constant, &self.coeffs)
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(&mut self.constant, &mut self.coeffs)
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

impl<T: Default, const N: usize> Default for Polynomial<T, N> {
    fn default() -> Self {
        Self::new(T::default(), Vector::default())
    }
}

impl<T: Zero, const N: usize> Zero for Polynomial<T, N> {
    fn zero() -> Self {
        Self::new(T::zero(), Vector::zero())
    }
    fn is_zero(&self) -> bool {
        self.constant.is_zero() && self.coeffs.iter().all(|x| x.is_zero())
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
    
    pub fn degree(&self) -> usize {
        match self.order() {
            0 => 0,
            x => x - 1,
        }
    }
}
