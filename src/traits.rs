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
        }
        impl NormLInf for $T {
            type Output = Self;
            fn norm_l_inf(self) -> Self {
                <$T as FloatCore>::abs(self)
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
