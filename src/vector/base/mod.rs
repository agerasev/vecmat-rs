mod format;
mod init;
mod iter;

#[cfg(test)]
mod tests;

pub use format::*;
pub use init::*;
pub use iter::*;

/// Vector of fixed size.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vector<T, const N: usize> {
    data: [T; N],
}
