use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use super::*;


impl<T: Debug> Debug for Quaternion<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "Quaternion({:?}, {:?}, {:?}, {:?})",
            self.w_ref(),
            self.x_ref(),
            self.y_ref(),
            self.z_ref(),
        )
    }
}
impl<T: Display> Display for Quaternion<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "Quaternion({}, {}, {}, {})",
            self.w_ref(),
            self.x_ref(),
            self.y_ref(),
            self.z_ref(),
        )
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use std::format;
    use super::*;

    #[test]
    fn quaternion() {
        let q = Quaternion::<i32>::new(1, -2, 3, -4);
        assert_eq!(format!("{:?}", q), "Quaternion(1, -2, 3, -4)");
        assert_eq!(format!("{}", q), "Quaternion(1, -2, 3, -4)");
    }
}
