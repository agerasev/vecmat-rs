#![feature(test)]

extern crate num_traits;
extern crate num_integer;
extern crate test;

pub mod vec;
#[cfg(test)]
mod vec_test;
#[cfg(test)]
mod vec_bench;

pub mod mat;
#[cfg(test)]
mod mat_test;

pub use vec::*;
pub use mat::*;
