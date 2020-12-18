use crate::{matrix::*, traits::*};
use core::{
    cmp::PartialOrd,
    iter::IntoIterator,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};
use num_traits::Zero;

impl<T, const M: usize, const N: usize> Neg for Matrix<T, N, M>
where
    T: Neg<Output = T>,
{
    type Output = Matrix<T, N, M>;
    fn neg(self) -> Self::Output {
        self.map(|v| -v)
    }
}

impl<T, const M: usize, const N: usize> Add for Matrix<T, N, M>
where
    T: Add<Output = T>,
{
    type Output = Matrix<T, N, M>;
    fn add(self, vec: Matrix<T, N, M>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x + y)
    }
}
impl<T, const M: usize, const N: usize> Sub for Matrix<T, N, M>
where
    T: Sub<Output = T>,
{
    type Output = Matrix<T, N, M>;
    fn sub(self, vec: Matrix<T, N, M>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x - y)
    }
}
impl<T, const M: usize, const N: usize> Mul for Matrix<T, N, M>
where
    T: Mul<Output = T>,
{
    type Output = Matrix<T, N, M>;
    fn mul(self, vec: Matrix<T, N, M>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x * y)
    }
}
impl<T, const M: usize, const N: usize> Div for Matrix<T, N, M>
where
    T: Div<Output = T>,
{
    type Output = Matrix<T, N, M>;
    fn div(self, vec: Matrix<T, N, M>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x / y)
    }
}
impl<T, const M: usize, const N: usize> Rem for Matrix<T, N, M>
where
    T: Rem<Output = T>,
{
    type Output = Matrix<T, N, M>;
    fn rem(self, vec: Matrix<T, N, M>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x % y)
    }
}

impl<T, const M: usize, const N: usize> Mul<T> for Matrix<T, N, M>
where
    T: Mul<Output = T> + ImplicitClone,
{
    type Output = Matrix<T, N, M>;
    fn mul(self, a: T) -> Self::Output {
        self.map(|v| v * a.clone())
    }
}
impl<T, const M: usize, const N: usize> Div<T> for Matrix<T, N, M>
where
    T: Div<Output = T> + ImplicitClone,
{
    type Output = Matrix<T, N, M>;
    fn div(self, a: T) -> Self::Output {
        self.map(|v| v / a.clone())
    }
}
impl<T, const M: usize, const N: usize> Rem<T> for Matrix<T, N, M>
where
    T: Rem<Output = T> + ImplicitClone,
{
    type Output = Matrix<T, N, M>;
    fn rem(self, a: T) -> Self::Output {
        self.map(|v| v % a.clone())
    }
}

impl<T, const M: usize, const N: usize> AddAssign for Matrix<T, N, M>
where
    T: AddAssign,
{
    fn add_assign(&mut self, vec: Matrix<T, N, M>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s += x;
        })
    }
}
impl<T, const M: usize, const N: usize> SubAssign for Matrix<T, N, M>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, vec: Matrix<T, N, M>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s -= x;
        })
    }
}
impl<T, const M: usize, const N: usize> MulAssign for Matrix<T, N, M>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, vec: Matrix<T, N, M>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s *= x;
        })
    }
}
impl<T, const M: usize, const N: usize> DivAssign for Matrix<T, N, M>
where
    T: DivAssign,
{
    fn div_assign(&mut self, vec: Matrix<T, N, M>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s /= x;
        })
    }
}
impl<T, const M: usize, const N: usize> RemAssign for Matrix<T, N, M>
where
    T: RemAssign,
{
    fn rem_assign(&mut self, vec: Matrix<T, N, M>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s %= x;
        })
    }
}

impl<T, const M: usize, const N: usize> MulAssign<T> for Matrix<T, N, M>
where
    T: MulAssign + ImplicitClone,
{
    fn mul_assign(&mut self, a: T) {
        self.iter_mut().for_each(|s| {
            *s *= a.clone();
        })
    }
}
impl<T, const M: usize, const N: usize> DivAssign<T> for Matrix<T, N, M>
where
    T: DivAssign + ImplicitClone,
{
    fn div_assign(&mut self, a: T) {
        self.iter_mut().for_each(|s| {
            *s /= a.clone();
        })
    }
}
impl<T, const M: usize, const N: usize> RemAssign<T> for Matrix<T, N, M>
where
    T: RemAssign + ImplicitClone,
{
    fn rem_assign(&mut self, a: T) {
        self.iter_mut().for_each(|s| {
            *s %= a.clone();
        })
    }
}

impl<T, const M: usize, const N: usize> Zero for Matrix<T, N, M>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::init(T::zero)
    }
    fn is_zero(&self) -> bool {
        self.iter().all(|x| x.is_zero())
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, N, M> {
    pub fn sum(self) -> T
    where
        T: Add<Output = T>,
    {
        self.fold_first(|x, y| x + y)
    }
    pub fn max(self) -> T
    where
        T: PartialOrd,
    {
        self.fold_first(|x, y| if x < y { y } else { x })
    }
    pub fn min(self) -> T
    where
        T: PartialOrd,
    {
        self.fold_first(|x, y| if x < y { x } else { y })
    }
}

impl<T, const M: usize, const N: usize> NormL1 for Matrix<T, N, M>
where
    T: NormL1<Output = T> + Add<Output = T>,
{
    type Output = T;
    fn norm_l1(self) -> T {
        self.map(|x| x.norm_l1()).sum()
    }
}
impl<T, const M: usize, const N: usize> NormL2 for Matrix<T, N, M>
where
    T: Mul<Output = T> + Add<Output = T> + GenericFloat + ImplicitClone,
{
    type Output = T;
    fn norm_l2(self) -> T {
        self.map(|x| x.clone() * x).sum().sqrt()
    }
}
impl<T, const M: usize, const N: usize> NormLInf for Matrix<T, N, M>
where
    T: NormLInf<Output = T> + PartialOrd,
{
    type Output = T;
    fn norm_l_inf(self) -> T {
        self.map(|x| x.norm_l_inf()).max()
    }
}
