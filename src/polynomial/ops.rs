use core::{
    ops::{
        Neg, Add, Sub, Mul, Div, Rem,
        AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    },
    cmp::{max, min},
    iter,
};
use num_traits::{Zero, One};
use super::Polynomial;
use crate::Vector;

impl<T: Neg<Output=T>, const N: usize> Neg for Polynomial<T, N> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.constant, -self.coeffs)
    }
}
impl<T: Add<Output=T>, const N: usize> Add for Polynomial<T, N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.constant + other.constant, self.coeffs + other.coeffs)
    }
}
impl<T: Sub<Output=T>, const N: usize> Sub for Polynomial<T, N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.constant - other.constant, self.coeffs - other.coeffs)
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

impl<T: Copy + Zero + Add<Output=T> + Mul<Output=T>, const N: usize> Mul for Polynomial<T, N> {
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
impl<T: Copy + Zero + One, const N: usize> One for Polynomial<T, N> {
    fn one() -> Self {
        Self::new(T::one(), Vector::zero())
    }
}
