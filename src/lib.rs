//! Low-dimensional vector algebra.
//!
//! ## Usage example
//!
//! ```
//! use vecmat::{prelude::*, Vector, Matrix};
//!
//! // Create vectors
//! let mut va = Vector::<f64, 3>::zero(); // filled with zeros
//! let vb = Vector::<f64, 3>::from([1.0, 2.0, 3.0]); // from values
//! println!("{}, {}", va, vb);
//!
//! // Vector access
//! va[1] = vb[0]; // read and write
//! va[0] += 3.0; // add-assign
//! *va.z_mut() = -vb.z(); // access by `xyz`
//! println!("{}", va);
//!
//! // Vector products
//! println!("{}", vb * 2.0); // scalar-by-vector
//! println!("{}", va * vb); // component-wise
//! println!("{}", va.dot(vb)); // dot
//! println!("{}", va.cross(vb)); // cross
//!
//! // Distance between two vectors
//! println!("{}", (va - vb).length());
//!
//! // Create matrices
//! let mut ma = Matrix::<f64, 3, 3>::one(); // identity 3x3 matrix
//! let mb = Matrix::<f64, 2, 3>::from([ // 3x2 matrix from values
//!     [1.0, 2.0, 0.0],
//!     [0.0,-1.0, 1.0],
//! ]);
//! println!("{},\n{}", ma, mb);
//!
//! // Access matrix components
//! ma[(1, 1)] = 2.0; // access by (i, j) indices
//! ma[(0, 2)] = -3.0;
//! ma[(2, 0)] = -1.0;
//! println!("{}", ma);
//!
//! // Transpose matrix
//! println!("{}", mb.transpose());
//!
//! // Matrix-vector product
//! println!("{}", mb.dot(vb));
//! println!("{}", va.dot(ma));
//!
//! // Matrix-matrix product
//! println!("{}", mb.dot(ma));
//!
//! // Outer product of vectors
//! println!("{}", va.outer(vb));
//!
//! // Determinant and inverse matrix
//! println!("{}", ma.det());
//! println!("{}", ma.inv());
//! ```

#![no_std]
#[cfg(feature = "std")]
extern crate std;

pub mod complex;
#[cfg(feature = "rand")]
pub mod distr;
pub mod matrix;
pub mod traits;
pub mod transform;
pub mod vector;
//pub mod polynomial;

pub use complex::{Complex, Quaternion};
pub use matrix::Matrix;
pub use transform::Transform;
pub use vector::Vector;

pub mod prelude {
    pub use crate::{traits::*, Transform};
    pub use core::convert::TryFrom;
    pub use num_traits::{One, Zero};
}
