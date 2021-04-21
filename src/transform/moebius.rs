pub use crate::complex::Moebius;
use crate::{complex::{Complex, Quaternion}, Transform};
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
