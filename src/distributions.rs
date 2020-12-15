pub use rand_distr::StandardNormal;

/// Uniform distribution over all possible values.
pub struct Uniform;

/// Distribution that only guarantees to produce an element which norm is greater than epsilon.
pub struct NonZero;

/// Distribution that provides points uniformly distubuted on the N-dimensional sphere,
/// where N is the number of dimensions of a specified hypercomplex number.
pub struct Unit;

/// Distribution that guarantees to produce an element which could be inverted.
pub struct Invertible;
