use core::ops::{Div};
use num_traits::Float;
use rand::Rng;
use rand_distr::Distribution;
use super::*;


pub use rand_distr::StandardNormal;

/// Distribution that only guarantees to produce an element which norm is greater than epsilon.
pub struct NonZero;

/// Distribution that provides points uniformly distubuted on the N-dimensional sphere,
/// where N is the number of dimensions of a specified hypercomplex number.
pub struct Unit;


impl<T, U> Distribution<Construct<T, U>> for StandardNormal where StandardNormal: Distribution<U> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Construct<T, U> {
        Construct::new(rng.sample(Self), rng.sample(Self))
    }
}

impl<T: Float, U: NormSqr<Output=T> + Clone> Distribution<Construct<T, U>> for NonZero where StandardNormal: Distribution<Construct<T, U>> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Construct<T, U> {
        loop {
            let x = rng.sample(&StandardNormal);
            if x.clone().norm() > T::epsilon() {
                break x;
            }
        }
    }
}

impl<T: Float, U: NormSqr<Output=T> + Div<T, Output=U> + Clone> Distribution<Construct<T, U>> for Unit where NonZero: Distribution<Construct<T, U>> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Construct<T, U> {
        rng.sample(&NonZero).normalize()
    }
}
