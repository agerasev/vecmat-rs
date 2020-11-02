#![no_std]
#![feature(min_const_generics)]

#[cfg(feature = "std")]
extern crate std;

pub mod traits;
pub use traits::*;

pub mod vector;
pub use vector::*;

pub mod matrix;
pub use matrix::*;

//pub mod map;
//#[cfg(test)]
//mod map_test;
