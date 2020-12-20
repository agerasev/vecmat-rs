use crate::{traits::ImplicitClone, Vector};
use core::ops::{Mul, Sub};
use num_traits::Zero;

impl<T> Vector<T, 2>
where
    T: ImplicitClone,
{
    pub fn x(&self) -> T {
        unsafe { self.get_unchecked(0) }.clone()
    }
    pub fn y(&self) -> T {
        unsafe { self.get_unchecked(1) }.clone()
    }
}
impl<T> Vector<T, 3>
where
    T: ImplicitClone,
{
    pub fn x(&self) -> T {
        unsafe { self.get_unchecked(0) }.clone()
    }
    pub fn y(&self) -> T {
        unsafe { self.get_unchecked(1) }.clone()
    }
    pub fn z(&self) -> T {
        unsafe { self.get_unchecked(2) }.clone()
    }
}
impl<T> Vector<T, 4>
where
    T: ImplicitClone,
{
    pub fn x(&self) -> T {
        unsafe { self.get_unchecked(0) }.clone()
    }
    pub fn y(&self) -> T {
        unsafe { self.get_unchecked(1) }.clone()
    }
    pub fn z(&self) -> T {
        unsafe { self.get_unchecked(2) }.clone()
    }
    pub fn w(&self) -> T {
        unsafe { self.get_unchecked(3) }.clone()
    }
}

impl<T> Vector<T, 2> {
    pub fn x_ref(&self) -> &T {
        unsafe { self.get_unchecked(0) }
    }
    pub fn y_ref(&self) -> &T {
        unsafe { self.get_unchecked(1) }
    }
    pub fn x_mut(&mut self) -> &mut T {
        unsafe { self.get_unchecked_mut(0) }
    }
    pub fn y_mut(&mut self) -> &mut T {
        unsafe { self.get_unchecked_mut(1) }
    }
}
impl<T> Vector<T, 3> {
    pub fn x_ref(&self) -> &T {
        unsafe { self.get_unchecked(0) }
    }
    pub fn y_ref(&self) -> &T {
        unsafe { self.get_unchecked(1) }
    }
    pub fn z_ref(&self) -> &T {
        unsafe { self.get_unchecked(2) }
    }
    pub fn x_mut(&mut self) -> &mut T {
        unsafe { self.get_unchecked_mut(0) }
    }
    pub fn y_mut(&mut self) -> &mut T {
        unsafe { self.get_unchecked_mut(1) }
    }
    pub fn z_mut(&mut self) -> &mut T {
        unsafe { self.get_unchecked_mut(2) }
    }
}
impl<T> Vector<T, 4> {
    pub fn x_ref(&self) -> &T {
        unsafe { self.get_unchecked(0) }
    }
    pub fn y_ref(&self) -> &T {
        unsafe { self.get_unchecked(1) }
    }
    pub fn z_ref(&self) -> &T {
        unsafe { self.get_unchecked(2) }
    }
    pub fn w_ref(&self) -> &T {
        unsafe { self.get_unchecked(3) }
    }
    pub fn x_mut(&mut self) -> &mut T {
        unsafe { self.get_unchecked_mut(0) }
    }
    pub fn y_mut(&mut self) -> &mut T {
        unsafe { self.get_unchecked_mut(1) }
    }
    pub fn z_mut(&mut self) -> &mut T {
        unsafe { self.get_unchecked_mut(2) }
    }
    pub fn w_mut(&mut self) -> &mut T {
        unsafe { self.get_unchecked_mut(3) }
    }
}

impl<T> Vector<T, 2>
where
    T: Mul<Output = T> + Sub<Output = T> + ImplicitClone,
{
    /// Pseudo-cross product for 2D vector.
    pub fn cross(self, other: Vector<T, 2>) -> T {
        self.x() * other.y() - self.y() * other.x()
    }
}
impl<T> Vector<T, 3>
where
    T: Mul<Output = T> + Sub<Output = T> + ImplicitClone,
{
    /// Cross product.
    pub fn cross(self, other: Vector<T, 3>) -> Vector<T, 3> {
        let (a, b) = (&self, &other);
        Self::from([
            a.y() * b.z() - a.z() * b.y(),
            a.z() * b.x() - a.x() * b.z(),
            a.x() * b.y() - a.y() * b.x(),
        ])
    }
}
impl<T> Vector<T, 4>
where
    T: Mul<Output = T> + Sub<Output = T> + Zero + ImplicitClone,
{
    /// Cross product of first three components, fourth one is set to zero.
    pub fn cross(self, other: Vector<T, 4>) -> Vector<T, 4> {
        let (a, b) = (&self, &other);
        Self::from([
            a.y() * b.z() - a.z() * b.y(),
            a.z() * b.x() - a.x() * b.z(),
            a.x() * b.y() - a.y() * b.x(),
            T::zero(),
        ])
    }
}

impl<T> Vector<T, 2> {
    pub fn from_tuple((x, y): (T, T)) -> Self {
        Self::from_array([x, y])
    }
    pub fn into_tuple(self) -> (T, T) {
        let mut it = self.into_iter();
        (it.next().unwrap(), it.next().unwrap())
    }
}
impl<T> Vector<T, 3> {
    pub fn from_tuple((x, y, z): (T, T, T)) -> Self {
        Self::from_array([x, y, z])
    }
    pub fn into_tuple(self) -> (T, T, T) {
        let mut it = self.into_iter();
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    }
}
impl<T> Vector<T, 4> {
    pub fn from_tuple((x, y, z, w): (T, T, T, T)) -> Self {
        Self::from_array([x, y, z, w])
    }
    pub fn into_tuple(self) -> (T, T, T, T) {
        let mut it = self.into_iter();
        (
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
            it.next().unwrap(),
        )
    }
}

impl<T> From<(T, T)> for Vector<T, 2> {
    fn from(tuple: (T, T)) -> Self {
        Self::from_tuple(tuple)
    }
}
impl<T> Into<(T, T)> for Vector<T, 2> {
    fn into(self) -> (T, T) {
        self.into_tuple()
    }
}
impl<T> From<(T, T, T)> for Vector<T, 3> {
    fn from(tuple: (T, T, T)) -> Self {
        Self::from_tuple(tuple)
    }
}
impl<T> Into<(T, T, T)> for Vector<T, 3> {
    fn into(self) -> (T, T, T) {
        self.into_tuple()
    }
}
impl<T> From<(T, T, T, T)> for Vector<T, 4> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Self::from_tuple(tuple)
    }
}
impl<T> Into<(T, T, T, T)> for Vector<T, 4> {
    fn into(self) -> (T, T, T, T) {
        self.into_tuple()
    }
}
