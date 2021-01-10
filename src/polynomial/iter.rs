use crate::vector::{self, Vector};


pub struct IntoIter<T, const N: usize>(
    Option<T>,
    vector::IntoIter<T, N>,
);
pub struct Iter<'a, T: 'a>(
    Option<&'a T>,
    vector::Iter<'a, T>,
);
pub struct IterMut<'a, T: 'a>(
    Option<&'a mut T>,
    vector::IterMut<'a, T>,
);

impl<T, const N: usize> IntoIter<T, N> {
    pub(super) fn new(c: T, v: Vector<T, N>) -> Self {
        Self(Some(c), v.into_iter())
    }
}
impl<'a, T: 'a> Iter<'a, T> {
    pub(super) fn new<const N: usize>(c: &'a T, v: &'a Vector<T, N>) -> Self {
        Self(Some(c), v.iter())
    }
}
impl<'a, T: 'a> IterMut<'a, T> {
    pub(super) fn new<const N: usize>(c: &'a mut T, v: &'a mut Vector<T, N>) -> Self {
        Self(Some(c), v.iter_mut())
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if let Some(c) = self.0.take() {
            Some(c)
        } else {
            self.1.next()
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.1.len() + if self.0.is_some() { 1 } else { 0 };
        (len, Some(len))
    }
}
impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if let Some(c) = self.0.take() {
            Some(c)
        } else {
            self.1.next()
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.1.len() + if self.0.is_some() { 1 } else { 0 };
        (len, Some(len))
    }
}
impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        if let Some(c) = self.0.take() {
            Some(c)
        } else {
            self.1.next()
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.1.len() + if self.0.is_some() { 1 } else { 0 };
        (len, Some(len))
    }
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    fn next_back(&mut self) -> Option<T> {
        if let Some(c) = self.1.next() {
            Some(c)
        } else {
            self.0.take()
        }
    }
}
impl<'a, T: 'a> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        if let Some(c) = self.1.next() {
            Some(c)
        } else {
            self.0.take()
        }
    }
}
impl<'a, T: 'a> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut T> {
        if let Some(c) = self.1.next() {
            Some(c)
        } else {
            self.0.take()
        }
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {
    fn len(&self) -> usize {
        self.1.len() + if self.0.is_some() { 1 } else { 0 }
    }
}
impl<'a, T: 'a> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.1.len() + if self.0.is_some() { 1 } else { 0 }
    }
}
impl<'a, T: 'a> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.1.len() + if self.0.is_some() { 1 } else { 0 }
    }
}
