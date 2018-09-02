pub extern crate num_traits as num;
pub extern crate num_integer as int;

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

pub use vec::*; // rm
pub use mat::*; // rm
