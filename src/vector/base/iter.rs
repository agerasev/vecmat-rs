pub use std::array::IntoIter;
use super::Vector;
use core::{
    convert::{TryFrom, TryInto},
    iter::IntoIterator,
    mem::{self, MaybeUninit},
    slice,
};

impl<T, const N: usize> Vector<T, N> {
    /// Returns iterator over vector element refrences.
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_ref().iter()
    }
    /// Returns iterator over vector element mutable refrences.
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut().iter_mut()
    }
}

impl<T, const N: usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.into())
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Vector<T, N> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut Vector<T, N> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().iter_mut()
    }
}

impl<'a, T, const N: usize> TryFrom<&'a [T]> for Vector<T, N>
where
    T: Copy,
{
    type Error = ();
    fn try_from(s: &'a [T]) -> Result<Self, Self::Error> {
        s.try_into().map(Self::from_array).map_err(|_| ())
    }
}

impl<T, const N: usize> Vector<T, N> {
    // TODO: Implement `TryFrom` without conflict.
    /// Try to conctruct a vector from iterator.
    /// If iterator conatains less items than vector, then `Err` is returned.
    pub fn try_from_iter<I>(iter: I) -> Option<Self>
    where
        I: Iterator<Item = T>,
    {
        let mut a = Vector::uninit();

        let mut pos: usize = 0;
        for (x, y) in a.iter_mut().zip(iter) {
            let _ = mem::replace(x, MaybeUninit::new(y));
            pos += 1;
        }

        // TODO: Use exclusive range pattern when it will be possible.
        #[allow(clippy::comparison_chain)]
        if pos > N {
            unreachable!();
        } else if pos == N {
            Some(unsafe { a.assume_init() })
        } else {
            // Drop loaded items
            unsafe {
                for x in a.as_mut().get_unchecked_mut(0..pos) {
                    mem::replace(x, MaybeUninit::uninit()).assume_init();
                }
            }
            None
        }
    }
}

impl<const N: usize> Vector<usize, N> {
    /// Create vector which element value equals to its position in vector.
    pub fn indices() -> Self {
        Self::try_from_iter(0..N).unwrap()
    }
}

impl<T, const N: usize> Vector<T, N> {
    /// Call closure for each element of the vector passing it by value.
    pub fn for_each<F: FnMut(T)>(self, f: F) {
        self.into_iter().for_each(f)
    }
    /// Map vector elements.
    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> Vector<U, N> {
        Vector::try_from_iter(&mut self.into_iter().map(f)).unwrap()
    }
    /// Zip two vectors into one.
    pub fn zip<U>(self, other: Vector<U, N>) -> Vector<(T, U), N> {
        Vector::try_from_iter(&mut self.into_iter().zip(other.into_iter())).unwrap()
    }
    /// Enumerate vector elements.
    pub fn enumerate(self) -> Vector<(usize, T), N> {
        Vector::indices().zip(self)
    }
}

impl<T, U, const N: usize> Vector<(T, U), N> {
    /// Unzip vector of tuples into two vectors.
    pub fn unzip(self) -> (Vector<T, N>, Vector<U, N>) {
        let mut a = Vector::<T, N>::uninit();
        let mut b = Vector::<U, N>::uninit();

        for ((x, y), (u, v)) in self.into_iter().zip(a.iter_mut().zip(b.iter_mut())) {
            let _ = mem::replace(u, MaybeUninit::new(x));
            let _ = mem::replace(v, MaybeUninit::new(y));
        }

        unsafe { (a.assume_init(), b.assume_init()) }
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn fold<S, F: FnMut(S, T) -> S>(self, s: S, f: F) -> S {
        self.into_iter().fold(s, f)
    }
    pub fn fold_first<F: FnMut(T, T) -> T>(self, f: F) -> T {
        let mut t = self.into_iter();
        let x = t.next().unwrap();
        t.fold(x, f)
    }
    pub fn scan<S, U, F: FnMut(&mut S, T) -> U>(self, s: S, mut f: F) -> Vector<U, N> {
        Vector::try_from_iter(&mut self.into_iter().scan(s, |s, x| Some(f(s, x)))).unwrap()
    }
}

/// Iterator that groups each N elements in original sequence into N-dimensional vector and yields it.
pub struct GroupIter<I: Iterator, const N: usize> {
    iter: I,
}

impl<I, const N: usize> GroupIter<I, N>
where
    I: Iterator,
{
    /// Create `GroupIter` from sequence.
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I, const N: usize> Iterator for GroupIter<I, N>
where
    I: Iterator,
{
    type Item = Vector<I::Item, N>;
    fn next(&mut self) -> Option<Self::Item> {
        <Self::Item>::try_from_iter(&mut self.iter)
    }
}
