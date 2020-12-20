use core::cell::Cell;
use num_traits::{Float, FloatConst, FromPrimitive};
use rand::{
    distributions::{uniform::SampleUniform, Distribution, Uniform},
    Rng,
};

/// Standard normal distribution.
pub struct Normal<T> {
    other: Cell<Option<T>>,
}

impl<T> Distribution<T> for Normal<T>
where
    T: SampleUniform + Float + FloatConst + FromPrimitive,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T {
        match self.other.take() {
            Some(x) => x,
            None => {
                let r = ((T::one() - Uniform::new(T::zero(), T::one()).sample(rng)).ln()
                    * T::from_f32(-2.0).unwrap())
                .sqrt();
                let phi = Uniform::new(T::zero(), T::PI()).sample(rng);
                self.other.set(Some(r * phi.sin()));
                r * phi.cos()
            }
        }
    }
}

/// Uniform distribution over all possible values.
pub struct UniformAll;

/// Distribution that only guarantees to produce an element which norm is greater than epsilon.
pub struct NonZero;

/// Distribution that provides points uniformly distubuted on the N-dimensional sphere,
/// where N is the number of dimensions of a specified hypercomplex number.
pub struct Unit;

/// Distribution that guarantees to produce an element which could be inverted.
pub struct Invertible;
