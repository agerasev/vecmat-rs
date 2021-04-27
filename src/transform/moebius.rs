pub use crate::complex::Moebius;
use crate::{complex::{Complex, Quaternion}, Transform, transform::Directional, traits::Normalize};
use core::ops::Neg;
use num_traits::{Num, NumCast};


impl<T> Transform<Complex<T>> for Moebius<Complex<T>>
where
    T: Neg<Output = T> + Num + Copy
{
    fn identity() -> Self {
        Moebius::identity()
    }
    fn inv(self) -> Self {
        Moebius::inv(self)
    }

    fn apply(&self, pos: Complex<T>) -> Complex<T> {
        Moebius::apply(self, pos)
    }
    fn deriv(&self, pos: Complex<T>, dir: Complex<T>) -> Complex<T> {
        Moebius::deriv(self, pos) * dir
    }

    fn chain(self, other: Self) -> Self {
        Moebius::chain(self, other)
    }
}

impl<T> Transform<Quaternion<T>> for Moebius<Complex<T>>
where
    T: Neg<Output = T> + Num + NumCast + Copy
{
    fn identity() -> Self {
        Moebius::identity()
    }
    fn inv(self) -> Self {
        Moebius::inv(self)
    }

    fn apply(&self, pos: Quaternion<T>) -> Quaternion<T> {
        Moebius::apply(self, pos)
    }
    fn deriv(&self, pos: Quaternion<T>, dir: Quaternion<T>) -> Quaternion<T> {
        Moebius::deriv_dir(self, pos, dir)
    }

    fn chain(self, other: Self) -> Self {
        Moebius::chain(self, other)
    }
}

impl<T> Directional<Complex<T>> for Moebius<Complex<T>>
where
    Self: Transform<Complex<T>>,
    Complex<T>: Normalize,
{
    fn apply_dir(&self, pos: Complex<T>, dir: Complex<T>) -> Complex<T> {
        self.deriv(pos, dir).normalize()
    }
    fn apply_normal(&self, pos: Complex<T>, normal: Complex<T>) -> Complex<T> {
        self.apply_dir(pos, normal)
    }
}

impl<T> Directional<Quaternion<T>> for Moebius<Complex<T>>
where
    Self: Transform<Quaternion<T>>,
    Quaternion<T>: Normalize,
{
    fn apply_dir(&self, pos: Quaternion<T>, dir: Quaternion<T>) -> Quaternion<T> {
        self.deriv(pos, dir).normalize()
    }
    fn apply_normal(&self, pos: Quaternion<T>, normal: Quaternion<T>) -> Quaternion<T> {
        self.apply_dir(pos, normal)
    }
}
