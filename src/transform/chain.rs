use crate::{Transform, transform::Directional, traits::Normalize};
#[cfg(feature = "approx")]
use approx::{abs_diff_eq, AbsDiffEq};
use core::marker::PhantomData;

/// Transformation obtained by combining two other ones.
///
/// Transformations are applied in the following order: `A(B(x))`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Chain<A, B, T> {
    outer: A,
    inner: B,
    phantom: PhantomData<T>,
}

impl<A, B, T> Chain<A, B, T> {
    /// Construct chained transformation from outer and inner ones.
    pub fn new(outer: A, inner: B) -> Self {
        Self {
            outer,
            inner,
            phantom: PhantomData,
        }
    }

    /// Split into two components components.
    pub fn split(self) -> (A, B) {
        (self.outer, self.inner)
    }

    /// Get outer transformation reference.
    pub fn outer(&self) -> &A {
        &self.outer
    }
    /// Get outer transformation mutable reference.
    pub fn outer_mut(&mut self) -> &mut A {
        &mut self.outer
    }

    /// Get inner transformation reference.
    pub fn inner(&self) -> &B {
        &self.inner
    }
    /// Get inner transformation mutable reference.
    pub fn inner_mut(&mut self) -> &mut B {
        &mut self.inner
    }
}

impl<A, B, T> From<(A, B)> for Chain<A, B, T> {
    fn from((a, b): (A, B)) -> Self {
        Self::new(a, b)
    }
}
impl<A, B, T> From<Chain<A, B, T>> for (A, B) {
    fn from(c: Chain<A, B, T>) -> Self {
        c.split()
    }
}

/// Transformations that can be reordered, e.g. for some `A` and `B` find such `A'` and `B'`
/// that satisfies `A(B(x)) = B'(A'(x))` for any `x`.
pub trait Reorder<B: Transform<T>, T>: Transform<T> {
    /// For given `A` and `B` returns `B'` and `A'`.
    fn reorder(self, other: B) -> (B, Self);
}

impl<A, B, T> Transform<T> for Chain<A, B, T>
where
    A: Transform<T> + Reorder<B, T>,
    B: Transform<T> + Reorder<A, T>,
    T: Copy,
{
    fn identity() -> Self {
        Self::new(A::identity(), B::identity())
    }
    fn inv(self) -> Self {
        self.inner.inv().reorder(self.outer.inv()).into()
    }
    fn apply(&self, pos: T) -> T {
        self.outer.apply(self.inner.apply(pos))
    }
    fn deriv(&self, pos: T, dir: T) -> T {
        self.outer
            .deriv(self.inner.apply(pos), self.inner.deriv(pos, dir))
    }
    fn chain(self, other: Self) -> Self {
        let (roa, rsb) = self.inner.reorder(other.outer);
        Self::new(self.outer.chain(roa), rsb.chain(other.inner))
    }
}

impl<A, B, T> Directional<T> for Chain<A, B, T>
where
    A: Directional<T>,
    B: Directional<T>,
    Self: Transform<T>,
    T: Normalize + Copy
{
    fn apply_dir(&self, pos: T, dir: T) -> T {
        self.outer.apply_dir(self.inner.apply(pos), self.inner.apply_dir(pos, dir))
    }
    fn apply_normal(&self, pos: T, normal: T) -> T {
        self.outer.apply_normal(self.inner.apply(pos), self.inner.apply_normal(pos, normal))
    }
}

#[cfg(feature = "approx")]
impl<A, B, T> AbsDiffEq for Chain<A, B, T>
where
    A: AbsDiffEq<Epsilon = T>,
    B: AbsDiffEq<Epsilon = T>,
    T: AbsDiffEq<Epsilon = T> + Copy,
{
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.outer, other.outer, epsilon = epsilon)
            && abs_diff_eq!(self.inner, other.inner, epsilon = epsilon)
    }
}
