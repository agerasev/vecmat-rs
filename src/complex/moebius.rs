use core::ops::{Neg, Add, Mul, Div};
use num_traits::{Zero, One, Num, NumCast};
use crate::{
    traits::Dot,
    matrix::Matrix2x2,
    complex::{Complex, Quaternion},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Moebius<T> {
    mat: Matrix2x2<T>,
}

impl<T> Moebius<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self::from([[a, b], [c, d]])
    }

    pub fn from_matrix(mat: Matrix2x2<T>) -> Self {
        Self { mat }
    }
    pub fn into_matrix(self) -> Matrix2x2<T> {
        self.mat
    }
    pub fn as_matrix(&self) -> &Matrix2x2<T> {
        &self.mat
    }
    pub fn as_mut_matrix(&mut self) -> &mut Matrix2x2<T> {
        &mut self.mat
    }

    pub fn from_array(array: [[T; 2]; 2]) -> Self {
        Self::from_matrix(Matrix2x2::from_array_of_arrays(array))
    }
    pub fn into_array(self) -> [[T; 2]; 2] {
        self.into_matrix().into_array_of_arrays()
    }
    pub fn as_array_of_arrays(&self) -> &[[T; 2]; 2] {
        &self.as_matrix().as_array_of_arrays()
    }
    pub fn as_mut_array_of_arrays(&mut self) -> &mut [[T; 2]; 2] {
        self.as_mut_matrix().as_mut_array_of_arrays()
    }
}

impl<T: Copy> Moebius<T> {
    pub fn a(&self) -> T { self.mat[(0, 0)] }
    pub fn b(&self) -> T { self.mat[(0, 1)] }
    pub fn c(&self) -> T { self.mat[(1, 0)] }
    pub fn d(&self) -> T { self.mat[(1, 1)] }
}

impl<T> Moebius<T> {
    pub fn a_ref(&self) -> &T { &self.mat[(0, 0)] }
    pub fn b_ref(&self) -> &T { &self.mat[(0, 1)] }
    pub fn c_ref(&self) -> &T { &self.mat[(1, 0)] }
    pub fn d_ref(&self) -> &T { &self.mat[(1, 1)] }

    pub fn a_mut(&mut self) -> &mut T { &mut self.mat[(0, 0)] }
    pub fn b_mut(&mut self) -> &mut T { &mut self.mat[(0, 1)] }
    pub fn c_mut(&mut self) -> &mut T { &mut self.mat[(1, 0)] }
    pub fn d_mut(&mut self) -> &mut T { &mut self.mat[(1, 1)] }
}

impl<T: Zero + One> Moebius<T> {
    pub fn identity() -> Self {
        Self::from(Matrix2x2::one())
    }
}

impl<T> Moebius<T> where T: Add<Output=T> + Mul<Output=T> + Div<Output=T> + Copy {
    pub fn chain(self, other: Self) -> Self {
        Self::from(Dot::dot(self.mat, other.mat))
    }
}

impl<T> Moebius<T> {
    pub fn apply<U>(&self, x: U) -> U
    where
        T: Mul<U, Output=U> + Copy,
        U: Add<T, Output=U> + Div<Output=U> + Copy,
    {
        (self.a() * x + self.b()) / (self.c() * x + self.d())
    }
}

impl<T: Neg<Output=T> + Num + Copy> Moebius<T> {
    pub fn det(&self) -> T {
        self.mat.det()
    }
    pub fn normalize(self) -> Self {
        let det = self.det();
        (self.mat / det).into()
    }
}

impl<T: Copy> Moebius<Complex<T>> where Complex<T>: Num {
    pub fn deriv(&self, p: Complex<T>) -> Complex<T> {
        let u = self.a() * p + self.b();
        let d = self.c() * p + self.d();
        (self.a() * d - u * self.c()) / (d * d)
    }
}

impl<T> Moebius<Complex<T>>
where
    T: Neg<Output=T> + Num + NumCast + Copy,
    Complex<T>: Mul<Quaternion<T>, Output=Quaternion<T>> + Copy,
    Quaternion<T>: Dot<Output=T> + Copy,
{
    pub fn deriv_dir(&self, p: Quaternion<T>, v: Quaternion<T>) -> Quaternion<T> {
        let u = self.a() * p + self.b();
        let d = self.c() * p + self.d();
        let d2 = d.norm_sqr();
        let g1 = (self.a() * v) / d;
        let g21 = (self.c() * v).conj();
        let g22 = d.conj() * (d.dot(self.c() * v) * T::from(2).unwrap() / d2);
        let g2 = u * ((g21 - g22) / d2);
        g1 + g2
    }
}

impl<T> From<Matrix2x2<T>> for Moebius<T> {
    fn from(mat: Matrix2x2<T>) -> Self {
        Self::from_matrix(mat)
    }
}
impl<T> From<Moebius<T>> for Matrix2x2<T> {
    fn from(mo: Moebius<T>) -> Self {
        mo.into_matrix()
    }
}
impl<'a, T> From<&'a Moebius<T>> for &'a Matrix2x2<T> {
    fn from(mo: &'a Moebius<T>) -> Self {
        mo.as_matrix()
    }
}
impl<'a, T> From<&'a mut Moebius<T>> for &'a mut Matrix2x2<T> {
    fn from(mo: &'a mut Moebius<T>) -> Self {
        mo.as_mut_matrix()
    }
}
impl<T> From<[[T; 2]; 2]> for Moebius<T> {
    fn from(array: [[T; 2]; 2]) -> Self {
        Self::from_array(array)
    }
}
impl<T> From<Moebius<T>> for [[T; 2]; 2] {
    fn from(mo: Moebius<T>) -> Self {
        mo.into_array()
    }
}
impl<'a, T> From<&'a Moebius<T>> for &'a [[T; 2]; 2] {
    fn from(mo: &'a Moebius<T>) -> Self {
        mo.as_array_of_arrays()
    }
}
impl<'a, T> From<&'a mut Moebius<T>> for &'a mut [[T; 2]; 2] {
    fn from(mo: &'a mut Moebius<T>) -> Self {
        mo.as_mut_array_of_arrays()
    }
}
