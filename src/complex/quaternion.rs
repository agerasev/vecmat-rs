pub use core::ops::{
    Neg,
    Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
};
pub use num_traits::{Zero, One, Num, Float};
pub use num_complex::{Complex};
pub use crate::{Vector4, Vector3, Dot};


/// Quaternion.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Quaternion<T> {
    vec: Vector4<T>,
}

impl<T> Quaternion<T> {
    pub fn new(w: T, x: T, y: T, z: T) -> Self {
        Self { vec: [w, x, y, z].into() }
    }
    pub fn from_vector(vec: Vector4<T>) -> Self {
        Self { vec }
    }
    pub fn from_array(arr: [T; 4]) -> Self {
        Self { vec: arr.into() }
    }
    pub fn from_tuple(tup: (T, T, T, T)) -> Self {
        Self { vec: tup.into() }
    }
    pub fn from_scalar_and_vector3(w: T, vec: Vector3<T>) -> Self {
        let (x, y, z) = vec.into();
        Self::new(w, x, y, z)
    }
    pub fn into_vector(self) -> Vector4<T> {
        self.vec
    }
    pub fn into_array(self) -> [T; 4] {
        self.vec.into()
    }
    pub fn into_tuple(self) -> (T, T, T, T) {
        self.vec.into()
    }
    pub fn into_scalar_and_vector3(self) -> (T, Vector3<T>) {
        let (w, x, y, z) = self.vec.into();
        (w, [x, y, z].into())
    }
}

impl<T> From<Vector4<T>> for Quaternion<T> {
    fn from(vec: Vector4<T>) -> Self {
        Self::from_vector(vec)
    }
}
impl<T> Into<Vector4<T>> for Quaternion<T> {
    fn into(self) -> Vector4<T> {
        self.into_vector()
    }
}
impl<T> From<(T, Vector3<T>)> for Quaternion<T> {
    fn from((w, vec): (T, Vector3<T>)) -> Self {
        Self::from_scalar_and_vector3(w, vec)
    }
}
impl<T> Into<(T, Vector3<T>)> for Quaternion<T> {
    fn into(self) -> (T, Vector3<T>) {
        self.into_scalar_and_vector3()
    }
}
impl<T> From<[T; 4]> for Quaternion<T> {
    fn from(arr: [T; 4]) -> Self {
        Self::from_array(arr)
    }
}
impl<T> Into<[T; 4]> for Quaternion<T> {
    fn into(self) -> [T; 4] {
        self.into_array()
    }
}
impl<T> From<(T, T, T, T)> for Quaternion<T> {
    fn from(tup: (T, T, T, T)) -> Self {
        Self::from_tuple(tup)
    }
}
impl<T> Into<(T, T, T, T)> for Quaternion<T> {
    fn into(self) ->(T, T, T, T) {
        self.into_tuple()
    }
}

impl<T> Quaternion<T> where T: Clone {
    pub fn w(&self) -> T {
        self.vec.x()
    }
    pub fn x(&self) -> T {
        self.vec.y()
    }
    pub fn y(&self) -> T {
        self.vec.z()
    }
    pub fn z(&self) -> T {
        self.vec.w()
    }
    pub fn xyz(&self) -> Vector3<T> {
        <Self as Into<(T, Vector3<T>)>>::into(self.clone()).1
    }
}

impl<T> Quaternion<T> {
    pub fn w_ref(&self) -> &T {
        self.vec.x_ref()
    }
    pub fn x_ref(&self) -> &T {
        self.vec.y_ref()
    }
    pub fn y_ref(&self) -> &T {
        self.vec.z_ref()
    }
    pub fn z_ref(&self) -> &T {
        self.vec.w_ref()
    }
    pub fn w_mut(&mut self) -> &mut T {
        self.vec.x_mut()
    }
    pub fn x_mut(&mut self) -> &mut T {
        self.vec.y_mut()
    }
    pub fn y_mut(&mut self) -> &mut T {
        self.vec.z_mut()
    }
    pub fn z_mut(&mut self) -> &mut T {
        self.vec.w_mut()
    }
}

impl<T> Neg for Quaternion<T> where T: Neg<Output=T> {
    type Output = Self;
    fn neg(self) -> Self {
        (-self.vec).into()
    }
}

impl<T> Quaternion<T> where T: Neg<Output=T> {
    pub fn conj(self) -> Self {
        let (w, x, y, z) = self.into();
        Self::new(w, -x, -y, -z)
    }
}

impl<T> Add for Quaternion<T> where T: Add<Output=T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        (self.vec + other.vec).into()
    }
}
impl<T> Add<Complex<T>> for Quaternion<T> where T: Add<Output=T> {
    type Output = Self;
    fn add(self, other: Complex<T>) -> Self {
        let (w, x, y, z) = self.into();
        Self::new(w + other.re, x + other.im, y, z)
    }
}
impl<T> Add<T> for Quaternion<T> where T: Add<Output=T> {
    type Output = Self;
    fn add(self, other: T) -> Self {
        let (w, x, y, z) = self.into();
        Self::new(w + other, x, y, z)
    }
}
impl<T> Sub for Quaternion<T> where T: Sub<Output=T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        (self.vec - other.vec).into()
    }
}
impl<T> Sub<Complex<T>> for Quaternion<T> where T: Sub<Output=T> {
    type Output = Self;
    fn sub(self, other: Complex<T>) -> Self {
        let (w, x, y, z) = self.into();
        Self::new(w - other.re, x - other.im, y, z)
    }
}
impl<T> Sub<T> for Quaternion<T> where T: Sub<Output=T> {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let (w, x, y, z) = self.into();
        Self::new(w - other, x, y, z)
    }
}

macro_rules! reverse_add_sub { ($T:ident) => (
    /// Workaround for reverse addition.
    impl Add<Quaternion<$T>> for $T {
        type Output = Quaternion<$T>;
        fn add(self, other: Quaternion<$T>) -> Self::Output {
            other + self
        }
    }
    /// Workaround for reverse addition.
    impl Add<Quaternion<$T>> for Complex<$T> {
        type Output = Quaternion<$T>;
        fn add(self, other: Quaternion<$T>) -> Self::Output {
            other + self
        }
    }
    /// Workaround for reverse subtraction.
    impl Sub<Quaternion<$T>> for $T {
        type Output = Quaternion<$T>;
        fn sub(self, other: Quaternion<$T>) -> Self::Output {
            -other + self
        }
    }
    /// Workaround for reverse subtraction.
    impl Sub<Quaternion<$T>> for Complex<$T> {
        type Output = Quaternion<$T>;
        fn sub(self, other: Quaternion<$T>) -> Self::Output {
            -other + self
        }
    }
) }

reverse_add_sub!(f32);
reverse_add_sub!(f64);


impl<T> AddAssign for Quaternion<T> where T: AddAssign {
    fn add_assign(&mut self, other: Self) {
        self.vec += other.vec;
    }
}
impl<T> AddAssign<Complex<T>> for Quaternion<T> where T: AddAssign {
    fn add_assign(&mut self, other: Complex<T>) {
        *self.w_mut() += other.re;
        *self.x_mut() += other.im;
    }
}
impl<T> AddAssign<T> for Quaternion<T> where T: AddAssign {
    fn add_assign(&mut self, other: T) {
        *self.w_mut() += other;
    }
}
impl<T> SubAssign for Quaternion<T> where T: SubAssign {
    fn sub_assign(&mut self, other: Self) {
        self.vec -= other.vec;
    }
}
impl<T> SubAssign<Complex<T>> for Quaternion<T> where T: SubAssign {
    fn sub_assign(&mut self, other: Complex<T>) {
        *self.w_mut() -= other.re;
        *self.x_mut() -= other.im;
    }
}
impl<T> SubAssign<T> for Quaternion<T> where T: SubAssign {
    fn sub_assign(&mut self, other: T) {
        *self.w_mut() -= other;
    }
}

impl<T> Zero for Quaternion<T> where T: Zero {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }
    fn is_zero(&self) -> bool {
        self.vec.is_zero()
    }
}

impl<T> Mul for Quaternion<T> where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Clone {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(
            self.w()*other.w() - self.x()*other.x() - self.y()*other.y() - self.z()*other.z(),
            self.w()*other.x() + self.x()*other.w() + self.y()*other.z() - self.z()*other.y(),
            self.w()*other.y() - self.x()*other.z() + self.y()*other.w() + self.z()*other.x(),
            self.w()*other.z() + self.x()*other.y() - self.y()*other.x() + self.z()*other.w(),
        )
    }
}
impl<T> Mul<Complex<T>> for Quaternion<T> where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Clone {
    type Output = Self;
    fn mul(self, other: Complex<T>) -> Self {
        Self::new(
            self.w()*other.re.clone() - self.x()*other.im.clone(),
            self.w()*other.im.clone() + self.x()*other.re.clone(),
            self.z()*other.im.clone() + self.y()*other.re.clone(),
            self.z()*other.re.clone() - self.y()*other.im.clone(),
        )
    }
}
impl<T> Mul<T> for Quaternion<T> where T: Mul<Output=T> + Clone {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        (self.vec * other).into()
    }
}

impl<T> One for Quaternion<T> where T: Zero + One + Sub<Output=T> + Clone {
    fn one() -> Self {
        Self::new(T::one(), T::zero(), T::zero(), T::zero())
    }
}

impl<T> Quaternion<T> where T: Zero + One {
    pub fn i() -> Self {
        Self::new(T::zero(), T::one(), T::zero(), T::zero())
    }
    pub fn j() -> Self {
        Self::new(T::zero(), T::zero(), T::one(), T::zero())
    }
    pub fn k() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::one())
    }
}

impl<T> Quaternion<T> where T: Add<Output=T> + Mul<Output=T> + Clone {
    pub fn norm_sqr(self) -> T {
        self.vec.square_length()
    }
}
impl<T> Quaternion<T> where T: Float + Clone {
    pub fn norm(self) -> T {
        self.vec.length()
    }
}

impl<T> Div<T> for Quaternion<T> where T: Div<Output=T> + Clone {
    type Output = Self;
    fn div(self, other: T) -> Self {
        (self.vec / other).into()
    }
}

impl<T> Quaternion<T> where T: Float + Clone {
    pub fn normalize(self) -> Self {
        self.clone() / self.norm()
    }
}
impl<T> Quaternion<T> where T: Neg<Output=T> + Num + Clone {
    pub fn inv(self) -> Self {
        self.clone().conj() / self.norm_sqr()
    }
}

impl<T> Div for Quaternion<T> where T: Neg<Output=T> + Num + Clone {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}
impl<T> Div<Complex<T>> for Quaternion<T> where T: Neg<Output=T> + Num + Clone {
    type Output = Self;
    fn div(self, other: Complex<T>) -> Self {
        self * other.inv()
    }
}

macro_rules! reverse_mul_div { ($T:ident) => (
    /// Workaround for reverse multiplication.
    impl Mul<Quaternion<$T>> for $T {
        type Output = Quaternion<$T>;
        fn mul(self, other: Quaternion<$T>) -> Self::Output {
            other * self
        }
    }
    /// Workaround for reverse multiplication.
    impl Mul<Quaternion<$T>> for Complex<$T> {
        type Output = Quaternion<$T>;
        fn mul(self, other: Quaternion<$T>) -> Self::Output {
            Quaternion::new(
                self.re.clone()*other.w() - self.im.clone()*other.x(),
                self.re.clone()*other.x() + self.im.clone()*other.w(),
                self.re.clone()*other.y() - self.im.clone()*other.z(),
                self.re.clone()*other.z() + self.im.clone()*other.y(),
            )
        }
    }
    /// Workaround for reverse division.
    impl Div<Quaternion<$T>> for $T {
        type Output = Quaternion<$T>;
        fn div(self, other: Quaternion<$T>) -> Self::Output {
            self * other.inv()
        }
    }
    /// Workaround for reverse division.
    impl Div<Quaternion<$T>> for Complex<$T> {
        type Output = Quaternion<$T>;
        fn div(self, other: Quaternion<$T>) -> Self::Output {
            self * other.inv()
        }
    }
) }

reverse_mul_div!(f32);
reverse_mul_div!(f64);


impl<T> Dot for Quaternion<T> where T: Add<Output=T> + Mul<Output=T> {
    type Output = T;
    fn dot(self, other: Self) -> T {
        self.vec.dot(other.vec).into()
    }
}
