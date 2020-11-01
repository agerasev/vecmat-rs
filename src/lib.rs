#![no_std]
#[cfg(feature = "std")]
extern crate std;


#[macro_use]
mod macros;

pub mod traits;
pub use traits::*;

pub mod array;
pub use array::*;

pub mod vector;
pub use vector::*;

//pub mod matrix;
//pub use matrix::*;

//pub mod map;
//#[cfg(test)]
//mod map_test;
