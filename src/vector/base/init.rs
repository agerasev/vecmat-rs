#![allow(clippy::missing_safety_doc)]

use crate::traits::ImplicitClone;
use core::{
    mem::{MaybeUninit},
    ops::{Index, IndexMut},
    ptr,
};
use super::Vector;

impl<T, const N: usize> Vector<MaybeUninit<T>, N> {
    /// Transpose `MaybeUninit<Vector<T, N>>` into `Vector<MaybeUninit<T>, N>`.
    fn from_uninit(uninit: MaybeUninit<Vector<T, N>>) -> Self {
        // TODO: Use `mem::transmute` when it will be possible.
        unsafe { ptr::read(&uninit as *const _ as *const Vector<MaybeUninit<T>, N>) }
    }
    /// Transpose `Vector<MaybeUninit<T>, N>` into `MaybeUninit<Vector<T, N>>`.
    fn into_uninit(self) -> MaybeUninit<Vector<T, N>> {
        // TODO: Use `mem::transmute` when it will be possible.
        unsafe { ptr::read(&self as *const _ as *const MaybeUninit<Vector<T, N>>) }
    }
}
impl<T, const N: usize> Vector<T, N> {
    /// Create a vector with uninitialized content.
    pub fn uninit() -> Vector<MaybeUninit<T>, N> {
        Vector::from_uninit(MaybeUninit::uninit())
    }
}
impl<T, const N: usize> Vector<MaybeUninit<T>, N> {
    /// Assume that vector content is initialized.
    pub unsafe fn assume_init(self) -> Vector<T, N> {
        self.into_uninit().assume_init()
    }
}

impl<T, const N: usize> Vector<T, N> {
    /// Initialize a vector with values from closure.
    pub fn init<F: FnMut() -> T>(mut f: F) -> Self {
        let mut a = Vector::uninit();

        for x in a.data.iter_mut() {
            *x = MaybeUninit::new(f());
        }

        unsafe { a.assume_init() }
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default,
{
    /// Create vector filled with default values.
    fn default() -> Self {
        Self::init(T::default)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Default,
{
    /// Create default vector.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: ImplicitClone,
{
    /// Create vector which elements are filled with scalar value.
    pub fn fill(v: T) -> Self {
        Self::init(|| v.clone())
    }
    /// Fill with a scalar value reference.
    pub fn fill_ref(v: &T) -> Self {
        Self::init(|| v.clone())
    }
}

impl<T, const N: usize> Vector<T, N> {
    /// Create from array.
    pub fn from_array(array: [T; N]) -> Self {
        Self { data: array }
    }
    /// Convert to array.
    pub fn into_array(self) -> [T; N] {
        self.data
    }
    /// Get a reference to underlying array.
    pub fn as_array(&self) -> &[T; N] {
        &self.data
    }
    /// Get a mutable reference to underlying array.
    pub fn as_mut_array(&mut self) -> &mut [T; N] {
        &mut self.data
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(a: [T; N]) -> Self {
        Self::from_array(a)
    }
}
impl<T, const N: usize> From<&[T; N]> for Vector<T, N>
where
    T: ImplicitClone,
{
    fn from(ar: &[T; N]) -> Self {
        Self::from_array(ar.clone())
    }
}
impl<T, const N: usize> Into<[T; N]> for Vector<T, N> {
    fn into(self) -> [T; N] {
        self.into_array()
    }
}
impl<'a, T, const N: usize> Into<&'a [T; N]> for &'a Vector<T, N> {
    fn into(self) -> &'a [T; N] {
        self.as_array()
    }
}
impl<'a, T, const N: usize> Into<&'a mut [T; N]> for &'a mut Vector<T, N> {
    fn into(self) -> &'a mut [T; N] {
        self.as_mut_array()
    }
}

impl<T, const N: usize> AsRef<[T; N]> for Vector<T, N> {
    fn as_ref(&self) -> &[T; N] {
        self.as_array()
    }
}
impl<T, const N: usize> AsMut<[T; N]> for Vector<T, N> {
    fn as_mut(&mut self) -> &mut [T; N] {
        self.as_mut_array()
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }
}

impl<T, const N: usize> Vector<T, N> {
    /// Get pointer to the first element.
    pub fn as_ptr(&self) -> *const T {
        self.as_ref().as_ptr()
    }
    /// Get mutable pointer to the first element.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut().as_mut_ptr()
    }
    /// Get reference to the elements without boundary checking.
    pub unsafe fn get_unchecked(&self, i: usize) -> &T {
        self.as_ref().get_unchecked(i)
    }
    /// Get mutable reference to the elements without boundary checking.
    pub unsafe fn get_unchecked_mut(&mut self, i: usize) -> &mut T {
        self.as_mut().get_unchecked_mut(i)
    }
}
