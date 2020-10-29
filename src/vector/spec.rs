use core::ops::{Sub, Mul};
use num_traits::{Zero};
use super::*;


impl<T> Vector<T, 2> where T: Clone {
    pub fn x(&self) -> T { unsafe { self.as_ref().get_unchecked(0).clone() } }
    pub fn y(&self) -> T { unsafe { self.as_ref().get_unchecked(1).clone() } }
}
impl<T> Vector<T, 3> where T: Clone {
    pub fn x(&self) -> T { unsafe { self.as_ref().get_unchecked(0).clone() } }
    pub fn y(&self) -> T { unsafe { self.as_ref().get_unchecked(1).clone() } }
    pub fn z(&self) -> T { unsafe { self.as_ref().get_unchecked(2).clone() } }
}
impl<T> Vector<T, 4> where T: Clone {
    pub fn x(&self) -> T { unsafe { self.as_ref().get_unchecked(0).clone() } }
    pub fn y(&self) -> T { unsafe { self.as_ref().get_unchecked(1).clone() } }
    pub fn z(&self) -> T { unsafe { self.as_ref().get_unchecked(2).clone() } }
    pub fn w(&self) -> T { unsafe { self.as_ref().get_unchecked(3).clone() } }
}

impl<T> Vector<T, 2> {
    pub fn x_ref(&self) -> &T { unsafe { self.as_ref().get_unchecked(0) } }
    pub fn y_ref(&self) -> &T { unsafe { self.as_ref().get_unchecked(1) } }
    pub fn x_mut(&mut self) -> &mut T { unsafe { self.as_mut().get_unchecked_mut(0) } }
    pub fn y_mut(&mut self) -> &mut T { unsafe { self.as_mut().get_unchecked_mut(1) } }
}
impl<T> Vector<T, 3> {
    pub fn x_ref(&self) -> &T { unsafe { self.as_ref().get_unchecked(0) } }
    pub fn y_ref(&self) -> &T { unsafe { self.as_ref().get_unchecked(1) } }
    pub fn z_ref(&self) -> &T { unsafe { self.as_ref().get_unchecked(2) } }
    pub fn x_mut(&mut self) -> &mut T { unsafe { self.as_mut().get_unchecked_mut(0) } }
    pub fn y_mut(&mut self) -> &mut T { unsafe { self.as_mut().get_unchecked_mut(1) } }
    pub fn z_mut(&mut self) -> &mut T { unsafe { self.as_mut().get_unchecked_mut(2) } }
}
impl<T> Vector<T, 4> {
    pub fn x_ref(&self) -> &T { unsafe { self.as_ref().get_unchecked(0) } }
    pub fn y_ref(&self) -> &T { unsafe { self.as_ref().get_unchecked(1) } }
    pub fn z_ref(&self) -> &T { unsafe { self.as_ref().get_unchecked(2) } }
    pub fn w_ref(&self) -> &T { unsafe { self.as_ref().get_unchecked(3) } }
    pub fn x_mut(&mut self) -> &mut T { unsafe { self.as_mut().get_unchecked_mut(0) } }
    pub fn y_mut(&mut self) -> &mut T { unsafe { self.as_mut().get_unchecked_mut(1) } }
    pub fn z_mut(&mut self) -> &mut T { unsafe { self.as_mut().get_unchecked_mut(2) } }
    pub fn w_mut(&mut self) -> &mut T { unsafe { self.as_mut().get_unchecked_mut(3) } }
}


impl<T> Vector<T, 2> where T: Mul<Output=T> + Sub<Output=T> + Clone {
    /// Pseudo-cross product for 2D vector.
	pub fn cross(self, other: Vector<T, 2>) -> T {
		self.x()*other.y() - self.y()*other.x()
	}
}
impl<T> Vector<T, 3> where T: Mul<Output=T> + Sub<Output=T> + Clone {
    /// Cross product.
	pub fn cross(self, other: Vector<T, 3>) -> Vector<T, 3> {
		let (a, b) = (&self, &other);
		Vector::from([
            a.y()*b.z() - a.z()*b.y(),
            a.z()*b.x() - a.x()*b.z(),
            a.x()*b.y() - a.y()*b.x(),
        ])
	}
}
impl<T> Vector<T, 4> where T: Mul<Output=T> + Sub<Output=T> + Zero + Clone {
    /// Cross product of first three components, fourth one is set to zero.
	pub fn cross(self, other: Vector<T, 4>) -> Vector<T, 4> {
		let (a, b) = (&self, &other);
		Vector::from([
            a.y()*b.z() - a.z()*b.y(),
            a.z()*b.x() - a.x()*b.z(),
            a.x()*b.y() - a.y()*b.x(),
            T::zero(),
        ])
	}
}
