use crate::{traits::*, vector::*};
use core::{
    cmp::PartialOrd,
    iter::IntoIterator,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};
use num_traits::{Float, Zero};

impl<T, const N: usize> Neg for Vector<T, N>
where
    T: Neg<Output = T>,
{
    type Output = Vector<T, N>;
    fn neg(self) -> Self::Output {
        self.map(|v| -v)
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Add<Output = T>,
{
    type Output = Vector<T, N>;
    fn add(self, vec: Vector<T, N>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x + y)
    }
}
impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Sub<Output = T>,
{
    type Output = Vector<T, N>;
    fn sub(self, vec: Vector<T, N>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x - y)
    }
}
impl<T, const N: usize> Mul for Vector<T, N>
where
    T: Mul<Output = T>,
{
    type Output = Vector<T, N>;
    fn mul(self, vec: Vector<T, N>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x * y)
    }
}
impl<T, const N: usize> Div for Vector<T, N>
where
    T: Div<Output = T>,
{
    type Output = Vector<T, N>;
    fn div(self, vec: Vector<T, N>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x / y)
    }
}
impl<T, const N: usize> Rem for Vector<T, N>
where
    T: Rem<Output = T>,
{
    type Output = Vector<T, N>;
    fn rem(self, vec: Vector<T, N>) -> Self::Output {
        self.zip(vec).map(|(x, y)| x % y)
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Mul<Output = T> + ImplicitClone,
{
    type Output = Vector<T, N>;
    fn mul(self, a: T) -> Self::Output {
        self.map(|v| v * a.clone())
    }
}
impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Div<Output = T> + ImplicitClone,
{
    type Output = Vector<T, N>;
    fn div(self, a: T) -> Self::Output {
        self.map(|v| v / a.clone())
    }
}
impl<T, const N: usize> Rem<T> for Vector<T, N>
where
    T: Rem<Output = T> + ImplicitClone,
{
    type Output = Vector<T, N>;
    fn rem(self, a: T) -> Self::Output {
        self.map(|v| v % a.clone())
    }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
    T: AddAssign,
{
    fn add_assign(&mut self, vec: Vector<T, N>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s += x;
        })
    }
}
impl<T, const N: usize> SubAssign for Vector<T, N>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, vec: Vector<T, N>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s -= x;
        })
    }
}
impl<T, const N: usize> MulAssign for Vector<T, N>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, vec: Vector<T, N>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s *= x;
        })
    }
}
impl<T, const N: usize> DivAssign for Vector<T, N>
where
    T: DivAssign,
{
    fn div_assign(&mut self, vec: Vector<T, N>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s /= x;
        })
    }
}
impl<T, const N: usize> RemAssign for Vector<T, N>
where
    T: RemAssign,
{
    fn rem_assign(&mut self, vec: Vector<T, N>) {
        self.iter_mut().zip(vec.into_iter()).for_each(|(s, x)| {
            *s %= x;
        })
    }
}

impl<T, const N: usize> MulAssign<T> for Vector<T, N>
where
    T: MulAssign + ImplicitClone,
{
    fn mul_assign(&mut self, a: T) {
        self.iter_mut().for_each(|s| {
            *s *= a.clone();
        })
    }
}
impl<T, const N: usize> DivAssign<T> for Vector<T, N>
where
    T: DivAssign + ImplicitClone,
{
    fn div_assign(&mut self, a: T) {
        self.iter_mut().for_each(|s| {
            *s /= a.clone();
        })
    }
}
impl<T, const N: usize> RemAssign<T> for Vector<T, N>
where
    T: RemAssign + ImplicitClone,
{
    fn rem_assign(&mut self, a: T) {
        self.iter_mut().for_each(|s| {
            *s %= a.clone();
        })
    }
}

macro_rules! reverse_mul {
    ($T:ident) => {
        /// Workaround for reverse multiplication.
        impl<const N: usize> Mul<Vector<$T, N>> for $T {
            type Output = Vector<$T, N>;
            fn mul(self, other: Vector<$T, N>) -> Self::Output {
                other * self
            }
        }
    };
}

reverse_mul!(u8);
reverse_mul!(u16);
reverse_mul!(u32);
reverse_mul!(u64);
reverse_mul!(i8);
reverse_mul!(i16);
reverse_mul!(i32);
reverse_mul!(i64);
reverse_mul!(f32);
reverse_mul!(f64);

impl<T, const N: usize> Zero for Vector<T, N>
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

impl<T, const N: usize> Vector<T, N> {
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

impl<T, const N: usize> NormL1 for Vector<T, N>
where
    T: NormL1<Output = T> + Add<Output = T>,
{
    type Output = T;
    fn norm_l1(self) -> T {
        self.map(|x| x.norm_l1()).sum()
    }
}
impl<T, const N: usize> NormL2 for Vector<T, N>
where
    T: Float,
{
    type Output = T;
    fn norm_l2(self) -> T {
        self.map(|x| x * x).sum().sqrt()
    }
}
impl<T, const N: usize> NormLInf for Vector<T, N>
where
    T: NormLInf<Output = T> + PartialOrd,
{
    type Output = T;
    fn norm_l_inf(self) -> T {
        self.map(|x| x.norm_l_inf()).max()
    }
}
