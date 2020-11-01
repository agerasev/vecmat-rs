
/// L1 Norm trait.
pub trait NormL1 {
    /// Type of the norm.
    type Output;
    /// Norm of the element.
	fn norm_l1(self) -> Self::Output;
}

/// L2 Norm trait.
pub trait NormL2 {
    /// Type of the norm.
    type Output;
    /// Norm of the element.
	fn norm_l2(self) -> Self::Output;
}

/// L-inf Norm trait.
pub trait NormLInf {
    /// Type of the norm.
    type Output;
    /// Norm of the element.
	fn norm_l_inf(self) -> Self::Output;
}

/// Dot product trait.
pub trait Dot<V=Self> {
    /// Dot product output type.
    type Output;
    /// Perform dot product.
	fn dot(self, other: V) -> Self::Output;
}

/// Outer product trait.
pub trait Outer<V> {
    /// Outer product output type.
	type Output;
    /// Perform outer product.
	fn outer(self, other: V) -> Self::Output;
}
