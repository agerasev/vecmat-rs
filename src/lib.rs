extern crate num_traits;
extern crate num_integer;

#[macro_use]
mod macros;

pub mod vec;
#[cfg(test)]
mod vec_test;

pub mod mat;
#[cfg(test)]
mod mat_test;

pub mod map;
#[cfg(test)]
mod map_test;

pub use vec::*;
pub use mat::*;
