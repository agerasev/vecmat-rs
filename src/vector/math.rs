use crate::{Vector, traits::Broadcast};
use num_traits::Float;

impl<T, const N: usize> Broadcast<Vector<T, N>> for Vector<T, N> {
    fn broadcast(self) -> Self { self }
}
impl<T: Copy, const N: usize> Broadcast<Vector<T, N>> for T {
    fn broadcast(self) -> Vector<T, N> { Vector::fill(self) }
}

impl<T: Float, const N: usize> Vector<T, N> {
    pub fn abs(self) -> Self { self.map(T::abs) }
    pub fn abs_sub<B: Broadcast<Self>>(self, other: B) -> Self { self.zip(other.broadcast()).map(|(s, o)| T::abs_sub(s, o)) }
    pub fn signum(self) -> Self { self.map(T::signum) }
    pub fn cbrt(self) -> Self { self.map(T::cbrt) }
    pub fn sqrt(self) -> Self { self.map(T::sqrt) }
    pub fn vmin<B: Broadcast<Self>>(self, other: B) -> Self { self.zip(other.broadcast()).map(|(s, o)| T::min(s, o)) }
    pub fn vmax<B: Broadcast<Self>>(self, other: B) -> Self { self.zip(other.broadcast()).map(|(s, o)| T::max(s, o)) }
    pub fn clamp<A: Broadcast<Self>, B: Broadcast<Self>>(self, a: A, b: B) -> Self { self.zip(a.broadcast().zip(b.broadcast())).map(|(s, (a, b))| T::min(T::max(s, a), b)) }
    pub fn acos(self) -> Self { self.map(T::acos) }
    pub fn asin(self) -> Self { self.map(T::asin) }
    pub fn atan(self) -> Self { self.map(T::atan) }
    pub fn atan2<B: Broadcast<Self>>(self, other: B) -> Self { self.zip(other.broadcast()).map(|(s, o)| T::atan2(s, o)) }
    pub fn cosh(self) -> Self { self.map(T::cosh) }
    pub fn hypot<B: Broadcast<Self>>(self, other: B) -> Self { self.zip(other.broadcast()).map(|(s, o)| T::hypot(s, o)) }
    pub fn sinh(self) -> Self { self.map(T::sinh) }
    pub fn tan(self) -> Self { self.map(T::tan) }
    pub fn tanh(self) -> Self { self.map(T::tanh) }
    pub fn log2(self) -> Self { self.map(T::log2) }
    pub fn mul_add<A: Broadcast<Self>, B: Broadcast<Self>>(self, a: A, b: B) -> Self { self.zip(a.broadcast().zip(b.broadcast())).map(|(s, (a, b))| T::mul_add(s, a, b)) }
    pub fn exp(self) -> Self { self.map(T::exp) }
    pub fn powi<B: Broadcast<Vector<i32, N>>>(self, other: B) -> Self { self.zip(other.broadcast()).map(|(s, o)| T::powi(s, o)) }
    pub fn ln(self) -> Self { self.map(T::ln) }
    pub fn powf<B: Broadcast<Self>>(self, other: B) -> Self { self.zip(other.broadcast()).map(|(s, o)| T::powf(s, o)) }
    pub fn log<B: Broadcast<Self>>(self, other: B) -> Self { self.zip(other.broadcast()).map(|(s, o)| T::log(s, o)) }
    pub fn sin(self) -> Self { self.map(T::sin) }
    pub fn cos(self) -> Self { self.map(T::cos) }
    pub fn asinh(self) -> Self { self.map(T::asinh) }
    pub fn acosh(self) -> Self { self.map(T::acosh) }
    pub fn atanh(self) -> Self { self.map(T::atanh) }
    pub fn floor(self) -> Self { self.map(T::floor) }
    pub fn ceil(self) -> Self { self.map(T::ceil) }
    pub fn round(self) -> Self { self.map(T::round) }
    pub fn trunc(self) -> Self { self.map(T::trunc) }
    pub fn fract(self) -> Self { self.map(T::fract) }
    pub fn recip(self) -> Self { self.map(T::recip) }
    pub fn exp2(self) -> Self { self.map(T::exp2) }
    pub fn log10(self) -> Self { self.map(T::log10) }
    pub fn sin_cos(self) -> (Self, Self) { self.map(T::sin_cos).unzip() }
    pub fn exp_m1(self) -> Self { self.map(T::exp_m1) }
    pub fn ln_1p(self) -> Self { self.map(T::ln_1p) }
}
