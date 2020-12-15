
macro_rules! matrix_transpose { ($M:expr, $N:expr, $W:ident, $R:ident, $V:ident, $U:ident) => (
    impl<T> $W<T> {
        pub fn transpose(self) -> $R<T> {
            let mut s = unsafe { ptr::read(&self as *const _ as *const $W<MaybeUninit<T>>) };
            mem::forget(self);

            let mut d: $R<MaybeUninit<T>> = unsafe {
                MaybeUninit::uninit().assume_init()
            };

            for i in 0..$M {
                for j in 0..$N {
                    unsafe { mem::swap(
                        d.get_unchecked_mut(j, i),
                        s.get_unchecked_mut(i, j),
                    ); }
                }
            }

            unsafe { ptr::read(&d as *const _ as *const $R<T>) }
        }
    }
) }
