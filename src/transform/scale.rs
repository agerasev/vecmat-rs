#[cfg(feature = "rand")]
use crate::distr::Normal;
use crate::{
    transform::{Reorder, Shift, Directional},
    Transform, Vector,
};
#[cfg(feature = "approx")]
use approx::{abs_diff_eq, AbsDiffEq};
use num_traits::{Num, Inv};
#[cfg(feature = "rand")]
use rand_::{distributions::Distribution, Rng};

/// Scale transformation.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Scale<T> {
    mag: T,
}

impl<T> Scale<T> {
    pub fn from_mag(mag: T) -> Self {
        Self { mag }
    }
    pub fn into_mag(self) -> T {
        self.mag
    }
}
impl<T> From<T> for Scale<T> {
    fn from(mag: T) -> Self {
        Self::from_mag(mag)
    }
}
/*
impl<T> From<Scale<T>> for T {
    fn from(scale: Scale<T>) -> T {
        scale.into_mag()
    }
}
*/

impl<T, const N: usize> Transform<Vector<T, N>> for Scale<T>
where
    T: Num + Inv<Output = T> + Copy,
{
    fn identity() -> Self {
        Self { mag: T::one() }
    }
    fn inv(self) -> Self {
        Self { mag: self.mag.inv() }
    }
    fn apply(&self, pos: Vector<T, N>) -> Vector<T, N> {
        pos * self.mag
    }
    fn deriv(&self, _pos: Vector<T, N>, dir: Vector<T, N>) -> Vector<T, N> {
        self.apply(dir)
    }
    fn chain(self, other: Self) -> Self {
        Self { mag: self.mag * other.mag }
    }
}

impl<T, const N: usize> Directional<Vector<T, N>> for Scale<T>
where
    Self: Transform<Vector<T, N>>,
{
    fn apply_dir(&self, _: Vector<T, N>, dir: Vector<T, N>) -> Vector<T, N> {
        dir
    }
    fn apply_normal(&self, _: Vector<T, N>, normal: Vector<T, N>) -> Vector<T, N> {
        normal
    }
}

#[cfg(feature = "rand")]
impl<T> Distribution<Scale<T>> for Normal
where
    Normal: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Scale<T> {
        Scale::from_mag(self.sample(rng))
    }
}

#[cfg(feature = "approx")]
impl<T> AbsDiffEq for Scale<T>
where
    T: AbsDiffEq<Epsilon = T> + Copy,
{
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.mag, other.mag, epsilon = epsilon)
    }
}

impl<T, const N: usize> Reorder<Scale<T>, Vector<T, N>> for Shift<T, N>
where
    Scale<T>: Transform<Vector<T, N>> + Copy,
    Self: Transform<Vector<T, N>>,
{
    fn reorder(self, other: Scale<T>) -> (Scale<T>, Shift<T, N>) {
        (other, other.inv().apply(self.into_vector()).into())
    }
}

impl<T, const N: usize> Reorder<Shift<T, N>, Vector<T, N>> for Scale<T>
where
    Self: Transform<Vector<T, N>>,
    Shift<T, N>: Transform<Vector<T, N>>,
{
    fn reorder(self, other: Shift<T, N>) -> (Shift<T, N>, Scale<T>) {
        (self.apply(other.into_vector()).into(), self)
    }
}
