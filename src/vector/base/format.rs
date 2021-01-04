use crate::Vector;
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

impl<T, const N: usize> Debug for Vector<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Vector{}(", N)?;
        for i in 0..(N - 1) {
            write!(f, "{:?}, ", self[i])?;
        }
        write!(f, "{:?})", self[N - 1])?;
        Ok(())
    }
}

impl<T, const N: usize> Display for Vector<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Vector{}(", N)?;
        for i in 0..(N - 1) {
            write!(f, "{}, ", self[i])?;
        }
        write!(f, "{})", self[N - 1])?;
        Ok(())
    }
}
