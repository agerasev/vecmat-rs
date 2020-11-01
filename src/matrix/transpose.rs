
macro_rules! matrix_transpose { ($M:expr, $N:expr, $V:ident, $W:ident) => (
    impl<T> $V<T> {
        pub fn transpose(self) -> $W<T> {
            let a: [T; $M*$N] = self.into();
            let mut s = unsafe { ptr::read(a.as_ptr() as *const [MaybeUninit<T>; $M*$N]) };
            mem::forget(a);

            let mut d: [MaybeUninit<T>; $N*$M] = unsafe {
                MaybeUninit::uninit().assume_init()
            };

            for i in 0..$M {
                for j in 0..$N {
                    unsafe { mem::swap(
                        d.get_unchecked_mut(j*$M + i),
                        s.get_unchecked_mut(i*$N + j),
                    ); }
                }
            }

            unsafe { ptr::read(d.as_ptr() as *const [T; $N*$M]) }.into()
        }
    }
) }
