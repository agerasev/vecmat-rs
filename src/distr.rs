use core::cell::Cell;
use num_traits::{Float, FloatConst, FromPrimitive};
use rand_::{
    distributions::{uniform::SampleUniform, Distribution, Uniform as RangedUniform},
    Rng,
};

pub struct NormalPair;

impl<T> Distribution<(T, T)> for NormalPair
where
    T: SampleUniform + Float + FloatConst + FromPrimitive,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (T, T) {
        let r = ((T::one() - RangedUniform::new(T::zero(), T::one()).sample(rng)).ln()
            * T::from_f32(-2.0).unwrap())
        .sqrt();
        let phi = RangedUniform::new(T::zero(), T::PI()).sample(rng);
        (r * phi.cos(), r * phi.sin())
    }
}

#[derive(Default)]
pub struct StatefulNormal<T> {
    other: Cell<Option<T>>,
}

impl<T> StatefulNormal<T> {
    pub fn new() -> Self {
        Self { other: Cell::new(None) }
    }
}

impl<T> Distribution<T> for StatefulNormal<T>
where
    NormalPair: Distribution<(T, T)>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        match self.other.take() {
            Some(x) => x,
            None => {
                let (x, y) = rng.sample(NormalPair);
                self.other.set(Some(y));
                x
            }
        }
    }
}

/// Standard normal distribution.
pub struct Normal;

impl<T> Distribution<T> for Normal
where
    NormalPair: Distribution<(T, T)>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        rng.sample(&NormalPair).0
    }
}

/// Uniform distribution over all possible values.
pub struct Uniform;

/// Distribution that only guarantees to produce an element which norm is greater than epsilon.
pub struct NonZero;

/// Distribution that provides points uniformly distubuted on the N-dimensional sphere,
/// where N is the number of dimensions of a specified hypercomplex number.
pub struct Unit;

/// Distribution that guarantees to produce an element which could be inverted.
pub struct Invertible;
