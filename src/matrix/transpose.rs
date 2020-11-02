use core::{
	mem::{self, MaybeUninit},
	ptr,
};
use crate::matrix::*;


impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn transpose(self) -> Matrix<T, N, M> {
        let mut s = unsafe { ptr::read(&self as *const _ as *const Matrix<MaybeUninit<T>, M, N>) };
        mem::forget(self);

        let mut d: Matrix<MaybeUninit<T>, N, M> = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        for i in 0..M {
            for j in 0..N {
                unsafe { mem::swap(
                    d.get_unchecked_mut(j, i),
                    s.get_unchecked_mut(i, j),
                ); }
            }
        }

        unsafe { ptr::read(&d as *const _ as *const Matrix<T, N, M>) }
    }
}
