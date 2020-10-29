use num_traits::{Float};


/// Norm type.
pub trait NormType {}
/// Metric type.
pub trait MetricType: NormType {}

/// A set of predefined metric types.
pub mod metrics {
    use super::{NormType, MetricType};

    /// L0 mertic.
    pub enum L0 {}
    impl NormType for L0 {}
    impl MetricType for L0 {}

    /// L1 mertic (aka Manhattan metric).
    pub enum L1 {}
    impl NormType for L1 {}
    impl MetricType for L1 {}

    /// L2 (or Euclidean) mertic.
    pub enum L2 {}
    impl NormType for L2 {}
    impl MetricType for L2 {}

    /// LInf mertic.
    pub enum LInf {}
    impl NormType for LInf {}
    impl MetricType for LInf {}

    /// Norm induced by `InnerProduct`.
    pub enum Induced {}
    impl NormType for Induced {}

    pub enum Default {}
    impl NormType for Default {}
    impl MetricType for Default {}
}

/// Metric trait.
pub trait Metric<K: MetricType = metrics::Default> {
    /// Return type of the metric function.
    type Output;
    /// Distance between two elements.
    fn distance(&self, other: &Self) -> Self::Output;
}

/// Norm trait.
pub trait Norm<K: NormType = metrics::Default> {
    /// Type of the norm.
    type Output;
    /// Norm of the element.
	fn norm(&self) -> Self::Output;
}
impl<T, K> Norm<K> for T where T: Metric<K>, K: MetricType {
    type Output = <Self as Metric<K>>::Output;
    fn norm(&self) -> Self::Output {
        self.distance(self)
    }
}

/// Inner product trait.
/// Induces `Norm<metric::Induced>`.
pub trait InnerProduct {
    /// Inner product output type.
    type Output;
    /// Perform inner product.
	fn inner(&self, other: &Self) -> Self::Output;
}
impl<T, R> Norm<metrics::Induced> for T where T: InnerProduct<Output=R>, R: Float {
    type Output = <Self as InnerProduct>::Output;
    fn norm(&self) -> Self::Output {
        self.inner(self).sqrt()
    }
}

/// Dot product trait.
///
/// The `DotProduct` trait is more general than `InnerProduct`.
/// It could be implemented for pair of different types (e.g. dot product of matrix by vector)
/// and doesn't induce `Norm<metric::Induced>`.
pub trait DotProduct<V=Self> {
    /// Dot product output type.
    type Output;
    /// Perform dot product.
	fn dot(self, other: V) -> Self::Output;
}
