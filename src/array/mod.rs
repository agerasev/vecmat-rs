#[macro_use]
mod base;
pub use base::*;

#[macro_use]
mod iter;
pub use iter::*;

#[cfg(test)]
mod tests;


use core::{
    mem::{self, MaybeUninit},
    ptr,
};


macro_rules! array_all { ($N:expr, $A:ident, $II:ident) => (
    array_base!($N, $A, $II);
) }


array_all!(2, Array2Ext, Array2IntoIter);
array_all!(3, Array3Ext, Array3IntoIter);
array_all!(4, Array4Ext, Array4IntoIter);
array_all!(5, Array5Ext, Array5IntoIter);
array_all!(6, Array6Ext, Array6IntoIter);
array_all!(7, Array7Ext, Array7IntoIter);
array_all!(8, Array8Ext, Array8IntoIter);
array_all!(9, Array9Ext, Array9IntoIter);
array_all!(10, Array10Ext, Array10IntoIter);
array_all!(11, Array11Ext, Array11IntoIter);
array_all!(12, Array12Ext, Array12IntoIter);
array_all!(13, Array13Ext, Array13IntoIter);
array_all!(14, Array14Ext, Array14IntoIter);
array_all!(15, Array15Ext, Array15IntoIter);
array_all!(16, Array16Ext, Array16IntoIter);

array_iter_mod!(2, Array2Ext, Array2GroupIter, Array2FlatIter);
array_iter_mod!(3, Array3Ext, Array3GroupIter, Array3FlatIter);
array_iter_mod!(4, Array4Ext, Array4GroupIter, Array4FlatIter);
