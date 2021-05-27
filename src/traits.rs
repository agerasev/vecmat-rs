use num_traits::float::FloatCore;

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
    /// Square norm of the element.
    fn norm_l2_sqr(self) -> Self::Output;
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

/// Something that could be normalized using most suitable norm.
pub trait Normalize {
    /// Normalize object.
    fn normalize(self) -> Self;
}

/// Dot product trait.
pub trait Dot<V = Self> {
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

/// Complex conjugate trait.
pub trait Conj {
    /// Perform complex conjugation.
    fn conj(self) -> Self;
}

/// Type that has epsilon value.
pub trait Epsilon {
    /// Check that value is inside epsilon area.
    fn is_epsilon(&self) -> bool;
}

/// Make new dimensions by copying values along axes. 
pub trait Broadcast<V> {
    /// Copy values along axes to get `V`.
    fn broadcast(self) -> V;
}

macro_rules! derive_primitive_base {
    ($T:ident) => {
        impl Dot for $T {
            type Output = Self;
            fn dot(self, other: Self) -> Self {
                self * other
            }
        }
    };
}

macro_rules! derive_primitive_unsigned {
    ($T:ident) => {
        derive_primitive_base!($T);

        impl NormL1 for $T {
            type Output = Self;
            fn norm_l1(self) -> Self {
                self
            }
        }
        impl NormL2 for $T {
            type Output = Self;
            fn norm_l2(self) -> Self {
                self
            }
            fn norm_l2_sqr(self) -> Self {
                self * self
            }
        }
        impl NormLInf for $T {
            type Output = Self;
            fn norm_l_inf(self) -> Self {
                self
            }
        }
    };
}

macro_rules! derive_primitive_signed {
    ($T:ident) => {
        derive_primitive_base!($T);

        impl NormL1 for $T {
            type Output = Self;
            fn norm_l1(self) -> Self {
                self.abs()
            }
        }
        impl NormL2 for $T {
            type Output = Self;
            fn norm_l2(self) -> Self {
                self.abs()
            }
            fn norm_l2_sqr(self) -> Self {
                self * self
            }
        }
        impl NormLInf for $T {
            type Output = Self;
            fn norm_l_inf(self) -> Self {
                self.abs()
            }
        }
    };
}

macro_rules! derive_primitive_float {
    ($T:ident) => {
        derive_primitive_base!($T);

        impl NormL1 for $T {
            type Output = Self;
            fn norm_l1(self) -> Self {
                <$T as FloatCore>::abs(self)
            }
        }
        impl NormL2 for $T {
            type Output = Self;
            fn norm_l2(self) -> Self {
                <$T as FloatCore>::abs(self)
            }
            fn norm_l2_sqr(self) -> Self {
                self * self
            }
        }
        impl NormLInf for $T {
            type Output = Self;
            fn norm_l_inf(self) -> Self {
                <$T as FloatCore>::abs(self)
            }
        }
        impl Conj for $T {
            fn conj(self) -> Self {
                self
            }
        }
        impl Epsilon for $T {
            fn is_epsilon(&self) -> bool {
                self.abs() <= Self::EPSILON
            }
        }
    };
}

derive_primitive_unsigned!(u8);
derive_primitive_unsigned!(u16);
derive_primitive_unsigned!(u32);
derive_primitive_unsigned!(u64);
derive_primitive_unsigned!(usize);

derive_primitive_signed!(i8);
derive_primitive_signed!(i16);
derive_primitive_signed!(i32);
derive_primitive_signed!(i64);
derive_primitive_signed!(isize);

derive_primitive_float!(f32);
derive_primitive_float!(f64);
