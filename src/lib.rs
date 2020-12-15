//! Low-dimensional vector algebra structures and operations.
//!
//! ## Usage example
//!
//! ```ignore
//! use vecmat::{
//!     prelude::*,
//!     Vector3,
//!     Matrix2x3, Matrix3x3,
//! };
//!
//! fn main() {
//!     // Create vectors
//!     let mut va = Vector3::<f64>::zero(); // filled with zeros
//!     let vb = Vector3::<f64>::from([1.0, 2.0, 3.0]); // from values
//!     println!("{}, {}", va, vb);
//!
//!     // Vector access
//!     va[1] = vb[0]; // read and write
//!     va[0] += 3.0; // add-assign
//!     *va.z_mut() = -vb.z(); // access by `xyz`
//!     println!("{}", va);
//!
//!     // Vector products
//!     println!("{}", vb * 2.0); // scalar-by-vector
//!     println!("{}", va * vb); // component-wise
//!     println!("{}", va.dot(vb)); // dot
//!     println!("{}", va.cross(vb)); // cross
//!
//!     // Distance between two vectors
//!     println!("{}", (va - vb).length());
//!
//!     // Create matrices
//!     let mut ma = Matrix3x3::<f64>::one(); // identity 3x3 matrix
//!     let mb = Matrix2x3::<f64>::from([ // 3x2 matrix from values
//!         [1.0, 2.0, 0.0],
//!         [0.0,-1.0, 1.0],
//!     ]);
//!     println!("{},\n{}", ma, mb);
//!
//!     // Access matrix components
//!     ma[(1, 1)] = 2.0; // access by (i, j) indices
//!     ma[(0, 2)] = -3.0;
//!     ma[(2, 0)] = -1.0;
//!     println!("{}", ma);
//!
//!     // Transpose matrix
//!     println!("{}", mb.transpose());
//!
//!     // Matrix-vector product
//!     println!("{}", mb.dot(vb));
//!     println!("{}", va.dot(ma));
//!
//!     // Matrix-matrix product
//!     println!("{}", mb.dot(ma));
//!
//!     // Outer product of vectors
//!     println!("{}", va.outer(vb));
//!
//!     // Determinant and inverse matrix
//!     println!("{}", ma.det());
//!     println!("{}", ma.inverse());
//! }
//! ```

#![feature(min_const_generics)]
#![no_std]
#[cfg(feature = "std")]
extern crate std;

mod traits;
pub use traits::*;

mod vector;
pub use vector::*;

//pub mod matrix;
//pub use matrix::*;

//pub mod complex;
//pub use complex::*;

//pub mod transform;
//pub use transform::*;

//#[cfg(feature = "random")]
//pub mod distributions;

pub mod prelude {
    pub use crate::traits::*;
    pub use num_traits::{One, Zero};
}
