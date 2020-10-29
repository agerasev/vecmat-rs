use super::*;


impl<T> Vector2<T> {
    /// Initialize by components.
	pub fn make(v0: T, v1: T) -> Self {
		Self { data: [v0, v1] }
    }
}
impl<T> Vector3<T> {
    /// Initialize by components.
	pub fn make(v0: T, v1: T, v2: T) -> Self {
		Self { data: [v0, v1, v2] }
    }
}
impl<T> Vector4<T> {
    /// Initialize by components.
	pub fn make(v0: T, v1: T, v2: T, v3: T) -> Self {
		Self { data: [v0, v1, v2, v3] }
    }
}

impl<T> Vector2<T> where T: Clone {
    pub fn x(&self) -> T { unsafe { self.data.get_unchecked(0).clone() } }
    pub fn y(&self) -> T { unsafe { self.data.get_unchecked(1).clone() } }
}
impl<T> Vector3<T> where T: Clone {
    pub fn x(&self) -> T { unsafe { self.data.get_unchecked(0).clone() } }
    pub fn y(&self) -> T { unsafe { self.data.get_unchecked(1).clone() } }
    pub fn z(&self) -> T { unsafe { self.data.get_unchecked(2).clone() } }
}
impl<T> Vector4<T> where T: Clone {
    pub fn x(&self) -> T { unsafe { self.data.get_unchecked(0).clone() } }
    pub fn y(&self) -> T { unsafe { self.data.get_unchecked(1).clone() } }
    pub fn z(&self) -> T { unsafe { self.data.get_unchecked(2).clone() } }
    pub fn w(&self) -> T { unsafe { self.data.get_unchecked(3).clone() } }
}

impl<T> Vector2<T> {
    pub fn x_ref(&self) -> &T { unsafe { self.data.get_unchecked(0) } }
    pub fn y_ref(&self) -> &T { unsafe { self.data.get_unchecked(1) } }
    pub fn x_mut(&mut self) -> &mut T { unsafe { self.data.get_unchecked_mut(0) } }
    pub fn y_mut(&mut self) -> &mut T { unsafe { self.data.get_unchecked_mut(1) } }
}
impl<T> Vector3<T> {
    pub fn x_ref(&self) -> &T { unsafe { self.data.get_unchecked(0) } }
    pub fn y_ref(&self) -> &T { unsafe { self.data.get_unchecked(1) } }
    pub fn z_ref(&self) -> &T { unsafe { self.data.get_unchecked(2) } }
    pub fn x_mut(&mut self) -> &mut T { unsafe { self.data.get_unchecked_mut(0) } }
    pub fn y_mut(&mut self) -> &mut T { unsafe { self.data.get_unchecked_mut(1) } }
    pub fn z_mut(&mut self) -> &mut T { unsafe { self.data.get_unchecked_mut(2) } }
}
impl<T> Vector4<T> {
    pub fn x_ref(&self) -> &T { unsafe { self.data.get_unchecked(0) } }
    pub fn y_ref(&self) -> &T { unsafe { self.data.get_unchecked(1) } }
    pub fn z_ref(&self) -> &T { unsafe { self.data.get_unchecked(2) } }
    pub fn w_ref(&self) -> &T { unsafe { self.data.get_unchecked(3) } }
    pub fn x_mut(&mut self) -> &mut T { unsafe { self.data.get_unchecked_mut(0) } }
    pub fn y_mut(&mut self) -> &mut T { unsafe { self.data.get_unchecked_mut(1) } }
    pub fn z_mut(&mut self) -> &mut T { unsafe { self.data.get_unchecked_mut(2) } }
    pub fn w_mut(&mut self) -> &mut T { unsafe { self.data.get_unchecked_mut(3) } }
}


impl<T> Vector2<T> where T: Mul<Output=T> + Sub<Output=T> + Clone {
    /// Pseudo-cross product for 2D vector.
	pub fn cross(self, other: Vector2<T>) -> T {
		self.x()*other.y() - self.y()*other.x()
	}
}
impl<T> Vector3<T> where T: Mul<Output=T> + Sub<Output=T> + Clone {
    /// Cross product.
	pub fn cross(self, other: Vector3<T>) -> Vector3<T> {
		let (a, b) = (&self, &other);
		Vector3::make(
            a.y()*b.z() - a.z()*b.y(),
            a.z()*b.x() - a.x()*b.z(),
            a.x()*b.y() - a.y()*b.x(),
        )
	}
}
impl<T> Vector4<T> where T: Mul<Output=T> + Sub<Output=T> + Zero + Clone {
    /// Cross product of first three components, fourth one is set to zero.
	pub fn cross(self, other: Vector4<T>) -> Vector4<T> {
		let (a, b) = (&self, &other);
		Vector4::make(
            a.y()*b.z() - a.z()*b.y(),
            a.z()*b.x() - a.x()*b.z(),
            a.x()*b.y() - a.y()*b.x(),
            T::zero(),
        )
	}
}

#[allow(unused_macros)]
macro_rules! vec_type {
	($Va:ident, $V:ident, $T:ident) => (
		pub type $Va = $V<$T>;
	)
}

#[cfg(test)]
#[test]
fn dummy_test() {
    Vector3::<f64>::new();
}
