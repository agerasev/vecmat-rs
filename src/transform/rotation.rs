#[cfg(feature = "rand")]
use crate::distr::{Uniform, Unit};
use crate::{
    transform::{Linear, Reorder, Shift},
    Complex, Matrix, Quaternion, Transform, Vector,
    traits::Dot,
};
#[cfg(feature = "approx")]
use approx::{abs_diff_eq, AbsDiffEq};
use core::ops::Neg;
#[cfg(feature = "rand")]
use num_traits::FloatConst;
use num_traits::{Float, Num, NumCast, One};
#[cfg(feature = "rand")]
use rand_::{
    distributions::{uniform::SampleUniform, Distribution, Uniform as RangedUniform},
    Rng,
};

// TODO: Use partial specialization when it will be possible.
/// Two-dimensional rotation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rotation2<T> {
    comp: Complex<T>,
}

impl<T> Rotation2<T> {
    pub fn from_complex(comp: Complex<T>) -> Self {
        Self { comp }
    }
    pub fn into_complex(self) -> Complex<T> {
        self.comp
    }
}

impl<T> From<Complex<T>> for Rotation2<T> {
    fn from(comp: Complex<T>) -> Self {
        Self::from_complex(comp)
    }
}
impl<T> From<Rotation2<T>> for Complex<T> {
    fn from(rot: Rotation2<T>) -> Self {
        rot.into_complex()
    }
}

impl<T> Rotation2<T>
where
    T: Float,
{
    pub fn new(angle: T) -> Self {
        Self {
            comp: Complex::new(angle.clone().cos(), angle.sin()),
        }
    }
    pub fn angle(&self) -> T {
        self.comp.im().atan2(self.comp.re())
    }
}

impl<T> Transform<T, 2> for Rotation2<T>
where
    T: Neg<Output = T> + Num + Copy,
{
    fn identity() -> Self {
        Self {
            comp: Complex::one(),
        }
    }
    fn inv(self) -> Self {
        Self {
            comp: self.comp.conj(),
        }
    }
    fn apply(&self, pos: Vector<T, 2>) -> Vector<T, 2> {
        (<Vector<T, 2> as Into<Complex<T>>>::into(pos) * self.into_complex()).into()
    }
    fn deriv(&self, _pos: Vector<T, 2>, dir: Vector<T, 2>) -> Vector<T, 2> {
        self.apply(dir)
    }
    fn chain(self, other: Self) -> Self {
        Self {
            comp: self.comp * other.comp,
        }
    }
}

impl<T> Rotation2<T>
where
    T: Neg<Output = T> + Copy,
{
    pub fn to_linear(self) -> Linear<T, 2> {
        Linear::from(Matrix::from([
            [self.comp.re(), -self.comp.im()],
            [self.comp.im(), self.comp.re()],
        ]))
    }
}

#[cfg(feature = "rand")]
impl<T> Distribution<Rotation2<T>> for Uniform
where
    RangedUniform<T>: Distribution<T>,
    T: SampleUniform + Float + FloatConst + NumCast,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation2<T> {
        Rotation2::new(rng.sample(&RangedUniform::new(
            T::zero(),
            T::from(2.0).unwrap() * T::PI(),
        )))
    }
}
#[cfg(feature = "approx")]
impl<T> AbsDiffEq for Rotation2<T>
where
    T: AbsDiffEq<Epsilon = T> + Copy,
{
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.comp.re(), other.comp.re(), epsilon = epsilon)
            && abs_diff_eq!(self.comp.im(), other.comp.im(), epsilon = epsilon)
    }
}

/// Three-dimensional rotation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rotation3<T> {
    quat: Quaternion<T>,
}

impl<T> Rotation3<T> {
    pub fn from_quaternion(quat: Quaternion<T>) -> Self {
        Self { quat }
    }
    pub fn into_quaternion(self) -> Quaternion<T> {
        self.quat
    }
}

impl<T> From<Quaternion<T>> for Rotation3<T> {
    fn from(quat: Quaternion<T>) -> Self {
        Self::from_quaternion(quat)
    }
}
impl<T> From<Rotation3<T>> for Quaternion<T> {
    fn from(rot: Rotation3<T>) -> Self {
        rot.into_quaternion()
    }
}

impl<T> Rotation3<T>
where
    T: Float + NumCast,
{
    pub fn new(axis: Vector<T, 3>, angle: T) -> Self {
        let half = angle / T::from(2.0).unwrap();
        Self {
            quat: Quaternion::from_scalar_and_vector3(half.cos(), axis * half.sin()),
        }
    }
    pub fn axis(&self) -> Vector<T, 3> {
        let (_, ax) = self.quat.into();
        ax.normalize()
    }
    pub fn angle(&self) -> T {
        let (w, ax) = self.quat.into();
        T::from(2.0).unwrap() * ax.length().atan2(w)
    }
}

impl<T> Transform<T, 3> for Rotation3<T>
where
    T: Neg<Output = T> + Num + Copy,
{
    fn identity() -> Self {
        Self {
            quat: Quaternion::one(),
        }
    }
    fn inv(self) -> Self {
        Self {
            quat: self.quat.conj(),
        }
    }
    fn apply(&self, pos: Vector<T, 3>) -> Vector<T, 3> {
        let qpos = Quaternion::from_scalar_and_vector3(T::zero(), pos);
        let qres = self.quat * qpos * self.quat.conj();
        let (_, res) = qres.into();
        res
    }
    fn deriv(&self, _pos: Vector<T, 3>, dir: Vector<T, 3>) -> Vector<T, 3> {
        self.apply(dir)
    }
    fn chain(self, other: Self) -> Self {
        Self {
            quat: self.quat * other.quat,
        }
    }
}

impl<T> Rotation3<T>
where
    T: Float + NumCast,
{
    pub fn to_linear(self) -> Linear<T, 3> {
        let t1 = T::one();
        let t2 = T::from(2.0).unwrap();
        let q = self.quat;
        Linear::from(Matrix::from([
            [
                t1 - t2 * q.y() * q.y() - t2 * q.z() * q.z(),
                t2 * q.x() * q.y() - t2 * q.z() * q.w(),
                t2 * q.x() * q.z() + t2 * q.y() * q.w(),
            ],
            [
                t2 * q.x() * q.y() + t2 * q.z() * q.w(),
                t1 - t2 * q.x() * q.x() - t2 * q.z() * q.z(),
                t2 * q.y() * q.z() - t2 * q.x() * q.w(),
            ],
            [
                t2 * q.x() * q.z() - t2 * q.y() * q.w(),
                t2 * q.y() * q.z() + t2 * q.x() * q.w(),
                t1 - t2 * q.x() * q.x() - t2 * q.y() * q.y(),
            ],
        ]))
    }
}

impl<T> Rotation3<T>
where
    T: Float + FloatConst,
{
    /// Returns any of transformations that rotate `dir` to `-z`-axis.
    pub fn look_at_any(dir: Vector<T, 3>) -> Self {
        let z = Vector::from((T::zero(), T::zero(), -T::one()));
        let cross = dir.cross(z);
        let cross_len = cross.length();
        if cross_len > T::epsilon() {
            let dot = dir.dot(z);
            Self::new(cross / cross_len, cross_len.atan2(dot))
        } else {
            let y = Vector::from((T::zero(), T::one(), T::zero()));
            Self::new(y, T::from(T::PI()).unwrap())
        }
    }
}

impl<T> Reorder<Rotation2<T>, T, 2> for Shift<T, 2>
where
    Rotation2<T>: Transform<T, 2> + Copy,
    Self: Transform<T, 2>,
{
    fn reorder(self, other: Rotation2<T>) -> (Rotation2<T>, Shift<T, 2>) {
        (other, other.inv().apply(self.into_vector()).into())
    }
}
impl<T> Reorder<Shift<T, 2>, T, 2> for Rotation2<T>
where
    Self: Transform<T, 2>,
    Shift<T, 2>: Transform<T, 2>,
{
    fn reorder(self, other: Shift<T, 2>) -> (Shift<T, 2>, Rotation2<T>) {
        (self.apply(other.into_vector()).into(), self)
    }
}

impl<T> Reorder<Rotation3<T>, T, 3> for Shift<T, 3>
where
    Rotation3<T>: Transform<T, 3> + Copy,
    Self: Transform<T, 3>,
{
    fn reorder(self, other: Rotation3<T>) -> (Rotation3<T>, Shift<T, 3>) {
        (other, other.inv().apply(self.into_vector()).into())
    }
}
impl<T> Reorder<Shift<T, 3>, T, 3> for Rotation3<T>
where
    Self: Transform<T, 3>,
    Shift<T, 3>: Transform<T, 3>,
{
    fn reorder(self, other: Shift<T, 3>) -> (Shift<T, 3>, Rotation3<T>) {
        (self.apply(other.into_vector()).into(), self)
    }
}

#[cfg(feature = "rand")]
impl<T> Distribution<Rotation3<T>> for Uniform
where
    Unit: Distribution<Vector<T, 3>>,
    RangedUniform<T>: Distribution<T>,
    T: SampleUniform + Float + FloatConst + NumCast,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation3<T> {
        Rotation3::new(
            rng.sample(&Unit),
            rng.sample(&RangedUniform::new(
                T::zero(),
                T::from(2.0).unwrap() * T::PI(),
            )),
        )
    }
}
#[cfg(feature = "approx")]
impl<T> AbsDiffEq for Rotation3<T>
where
    T: AbsDiffEq<Epsilon = T> + Copy,
{
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.quat, other.quat, epsilon = epsilon)
    }
}

#[cfg(all(test, feature = "approx", feature = "rand"))]
mod tests {
    use super::*;
    use crate::{distr::Normal, prelude::*, vector::*};
    use approx::assert_abs_diff_eq;
    use rand_::SeedableRng;
    use rand_xorshift::XorShiftRng;

    const SAMPLE_ATTEMPTS: usize = 256;
    const EPS: f64 = 1e-14;

    mod r2d {
        use super::*;

        #[test]
        fn mapping() {
            let mut rng = XorShiftRng::seed_from_u64(0x2DA);
            for _ in 0..SAMPLE_ATTEMPTS {
                let r: Rotation2<f64> = rng.sample(&Uniform);
                let a: Vector2<f64> = rng.sample(&Normal);
                let b = r.apply(a);
                assert_abs_diff_eq!(a.length(), b.length(), epsilon = EPS);
                assert_abs_diff_eq!(a.dot(b) / a.square_length(), r.angle().cos(), epsilon = EPS);
            }
        }

        #[test]
        fn chaining() {
            let mut rng = XorShiftRng::seed_from_u64(0x2DB);
            for _ in 0..SAMPLE_ATTEMPTS {
                let a: Rotation2<f64> = rng.sample(&Uniform);
                let b: Rotation2<f64> = rng.sample(&Uniform);
                let v: Vector2<f64> = rng.sample(&Normal);
                assert_abs_diff_eq!(a.chain(b).apply(v), a.apply(b.apply(v)), epsilon = EPS);
            }
        }

        #[test]
        fn inversion() {
            let mut rng = XorShiftRng::seed_from_u64(0x2DC);
            for _ in 0..SAMPLE_ATTEMPTS {
                let r: Rotation2<f64> = rng.sample(&Uniform);
                assert_abs_diff_eq!(r.chain(r.inv()), Rotation2::identity(), epsilon = EPS);
            }
        }

        #[test]
        fn to_linear() {
            let mut rng = XorShiftRng::seed_from_u64(0x2DD);
            for _ in 0..SAMPLE_ATTEMPTS {
                let a: Rotation2<f64> = rng.sample(&Uniform);
                let b: Rotation2<f64> = rng.sample(&Uniform);
                let x: Vector2<f64> = rng.sample(&Normal);
                assert_abs_diff_eq!(a.to_linear().apply(x), a.apply(x), epsilon = EPS);
                assert_abs_diff_eq!(
                    a.to_linear().chain(b.to_linear()),
                    a.chain(b).to_linear(),
                    epsilon = EPS,
                );
            }
        }
    }

    mod r3d {
        use super::*;

        #[test]
        fn mapping() {
            let mut rng = XorShiftRng::seed_from_u64(0x3DA);
            for _ in 0..SAMPLE_ATTEMPTS {
                let r: Rotation3<f64> = rng.sample(&Uniform);
                let mut a: Vector3<f64> = rng.sample(&Normal);
                let mut b = r.apply(a);
                let (axis, angle) = (r.axis(), r.angle());
                assert_abs_diff_eq!(a.length(), b.length(), epsilon = EPS);
                a -= axis * a.dot(axis);
                b -= axis * b.dot(axis);
                let aa = a.square_length();
                assert_abs_diff_eq!(a.dot(b) / aa, angle.cos(), epsilon = EPS);
                assert_abs_diff_eq!(a.cross(b) / aa, axis * angle.sin(), epsilon = EPS);
            }
        }

        #[test]
        fn chaining() {
            let mut rng = XorShiftRng::seed_from_u64(0x2DB);
            for _ in 0..SAMPLE_ATTEMPTS {
                let a: Rotation3<f64> = rng.sample(&Uniform);
                let b: Rotation3<f64> = rng.sample(&Uniform);
                let v: Vector3<f64> = rng.sample(&Normal);
                assert_abs_diff_eq!(a.chain(b).apply(v), a.apply(b.apply(v)), epsilon = EPS);
            }
        }

        #[test]
        fn inversion() {
            let mut rng = XorShiftRng::seed_from_u64(0x2DC);
            for _ in 0..SAMPLE_ATTEMPTS {
                let r: Rotation3<f64> = rng.sample(&Uniform);
                assert_abs_diff_eq!(r.chain(r.inv()), Rotation3::identity(), epsilon = EPS);
            }
        }

        #[test]
        fn to_linear() {
            let mut rng = XorShiftRng::seed_from_u64(0x2DD);
            for _ in 0..SAMPLE_ATTEMPTS {
                let a: Rotation3<f64> = rng.sample(&Uniform);
                let b: Rotation3<f64> = rng.sample(&Uniform);
                let x: Vector3<f64> = rng.sample(&Normal);
                assert_abs_diff_eq!(a.to_linear().apply(x), a.apply(x), epsilon = EPS);
                assert_abs_diff_eq!(
                    a.to_linear().chain(b.to_linear()),
                    a.chain(b).to_linear(),
                    epsilon = EPS,
                );
            }
        }


        #[test]
        fn look_to_the_direction() {
            const EPS: f64 = 1e-14;
            let mut rng = XorShiftRng::seed_from_u64(0xBEC);

            for _ in 0..SAMPLE_ATTEMPTS {
                let d: Vector<f64, 3> = rng.sample(&Unit);
                let m = Rotation3::look_at_any(d);

                assert_abs_diff_eq!(m.apply(d), Vector::from([0.0, 0.0, -1.0]), epsilon = EPS);
            }
        }
    }
}
