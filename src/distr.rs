use num_traits::{Float, FloatConst};
use rand_::{
    distributions::{Distribution, Uniform as RangedUniform},
    Rng,
};

/// Standard normal distribution.
pub struct Normal;

// FIXME: Use another normal generator without 2x overhead
macro_rules! impl_normal_float {
    ($T:ident) => {
        impl Distribution<$T> for Normal {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $T {
                let r = ((1.0 - RangedUniform::new(0.0, 1.0).sample(rng)).ln() * -2.0).sqrt();
                let phi = RangedUniform::new(0.0, $T::PI()).sample(rng);
                r * phi.cos()
                //r * phi.sin()
            }
        }
    };
}
impl_normal_float!(f32);
impl_normal_float!(f64);

/// Uniform distribution over all possible values.
pub struct Uniform;

/// Distribution that only guarantees to produce an element which norm is greater than epsilon.
pub struct NonZero;

/// Distribution that provides points uniformly distubuted on the N-dimensional sphere,
/// where N is the number of dimensions of a specified hypercomplex number.
pub struct Unit;

/// Distribution that guarantees to produce an element which could be inverted.
pub struct Invertible;

/// Distribution that produces *some* elements with no further guarantees (but with reasonable variation).
pub struct SomeDistr;
