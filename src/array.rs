use core::{
    mem::{self, MaybeUninit},
    ptr,
};


macro_rules! array_base { ($N:expr, $I:ident, $II:ident) => (
    pub trait $I<T>: From<[T; $N]> + Into<[T; $N]> {
        fn init<F: FnMut() -> T>(f: F) -> Self;
        fn from_iter<I: Iterator<Item=T>>(iter: &mut I) -> Option<Self>;
        fn into_iter(self) -> $II<T> {
            $II::new(self.into())
        }
    }

    impl<T> $I<T> for [T; $N] {
        fn init<F: FnMut() -> T>(mut f: F) -> Self {
            let mut a: [MaybeUninit<T>; $N] = unsafe {
                MaybeUninit::uninit().assume_init()
            };

            for x in a.iter_mut() {
                *x = MaybeUninit::new(f());
            }

            // unsafe { mem::transmute::<_, [T; $N]>(a) }
            unsafe { ptr::read(a.as_ptr() as *const [T; $N]) }
        }

        fn from_iter<I: Iterator<Item=T>>(iter: &mut I) -> Option<Self> {
            let mut a: [MaybeUninit<T>; $N] = unsafe {
                MaybeUninit::uninit().assume_init()
            };

            let mut pos: usize = 0;
            for (x, y) in a.iter_mut().zip(iter) {
                let _ = mem::replace(x, MaybeUninit::new(y));
                pos += 1;
            }

            if pos > $N {
                unreachable!();
            } else if pos == $N {
                Some(unsafe { ptr::read(a.as_ptr() as *const [T; $N]) })
            } else {
                // Drop loaded items
                unsafe {
                    for x in a.get_unchecked_mut(0..pos) {
                        mem::replace(x, MaybeUninit::uninit()).assume_init();
                    }
                }
                None
            }
        }
    }

    pub struct $II<T> {
        data: [MaybeUninit<T>; $N],
        pos: usize,
    }
    impl<T> $II<T> {
        pub fn new(a: [T; $N]) -> Self {
            let it = Self {
                data: unsafe {
                    // unsafe { mem::transmute::<_, [MaybeUninit<T>; $N]>(a) }
                    ptr::read(a.as_ptr() as *const [MaybeUninit<T>; $N])
                },
                pos: 0
            };
            mem::forget(a);
            it
        }
    }
    impl<T> Iterator for $II<T> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            if self.pos < $N {
                let o = unsafe { mem::replace(
                    self.data.get_unchecked_mut(self.pos),
                    MaybeUninit::uninit()
                ).assume_init() };
                self.pos += 1;
                Some(o)
            } else {
                None
            }
        }
    }
    impl<T> Drop for $II<T> {
        fn drop(&mut self) {
            unsafe {
                for x in self.data.get_unchecked_mut(self.pos..$N) {
                    mem::replace(x, MaybeUninit::uninit()).assume_init();
                }
            }
        }
    }
)}

array_base!(16, Array16Ext, Array16IntoIter);


#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        rc::Rc,
        vec::Vec,
        iter,
    };


    #[test]
    fn init_drop() {
        let a = <[Rc<()>; 16]>::init(|| Rc::new(()));
        let b = a.clone();
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }

        mem::drop(b);
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn into_iter() {
        let a = <[Rc<()>; 16]>::init(|| Rc::new(()));
        let b = a.clone();
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }

        let mut c = b.into_iter().skip(8);
        c.next().unwrap();

        for (i, x) in a.iter().enumerate() {
            if i < 9 {
                assert_eq!(Rc::strong_count(x), 1);
            } else {
                assert_eq!(Rc::strong_count(x), 2);
            }
        }

        mem::drop(c);
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn from_iter() {
        let v = iter::repeat_with(|| Rc::new(())).take(16).collect::<Vec<_>>();
        let a = <[Rc<()>; 16]>::from_iter(&mut v.iter().cloned()).unwrap();

        for x in v.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }
        mem::drop(a);

        assert!(<[Rc<()>; 16]>::from_iter(&mut v.iter().cloned().take(8)).is_none());
        for x in v.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }
}
