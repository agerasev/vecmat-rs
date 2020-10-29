use core::{
    mem::{self, MaybeUninit},
    ptr,
};


macro_rules! array_base { ($N:expr, $I:ident, $II:ident) => (
    /// Trait that extends array to be constructible element-by-element and iterated.
    pub trait $I<T>: From<[T; $N]> + Into<[T; $N]> {
        /// Initializes an array with values from function.
        fn init_<F: FnMut() -> T>(f: F) -> Self;

        /// Conctructs an array from iterator.
        /// If iterator conatins less items than array, then `None` is returned.
        fn from_iter_<I: Iterator<Item=T>>(iter: &mut I) -> Option<Self>;

        /// Converts an array into iterator by values.
        fn into_iter_(self) -> $II<T> {
            $II::new(self.into())
        }

        /// Method similar to `Iterator::for_each` for arrays.
        /// `_` added to avoid possible future ambiguity.
        fn for_each_<F: FnMut(T)>(self, f: F);

        /// Method similar to `Iterator::map` for arrays.
        /// `_` added to avoid possible future ambiguity.
        fn map_<U, F: FnMut(T) -> U>(self, f: F) -> [U; $N];

        /// Method similar to `Iterator::zip` for arrays.
        /// `_` added to avoid possible future ambiguity.
        fn zip_<U>(self, other: [U; $N]) -> [(T, U); $N];

        /// Method similar to `Iterator::unzip` for arrays.
        /// `_` added to avoid possible future ambiguity.
        fn unzip_<U, V>(self) -> ([U; $N], [V; $N]) where Self: $I<(U, V)>;
    }

    impl<T> $I<T> for [T; $N] {
        fn init_<F: FnMut() -> T>(mut f: F) -> Self {
            let mut a: [MaybeUninit<T>; $N] = unsafe {
                MaybeUninit::uninit().assume_init()
            };

            for x in a.iter_mut() {
                *x = MaybeUninit::new(f());
            }

            // unsafe { mem::transmute::<_, [T; $N]>(a) }
            unsafe { ptr::read(a.as_ptr() as *const [T; $N]) }
        }

        fn from_iter_<I: Iterator<Item=T>>(iter: &mut I) -> Option<Self> {
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

        fn for_each_<F: FnMut(T)>(self, f: F) {
            self.into_iter_().for_each(f)
        }
        fn map_<U, F: FnMut(T) -> U>(self, f: F) -> [U; $N] {
            <[U; $N]>::from_iter_(&mut self.into_iter_().map(f)).unwrap()
        }
        fn zip_<U>(self, other: [U; $N]) -> [(T, U); $N] {
            <[(T, U); $N]>::from_iter_(&mut self.into_iter_().zip(other.into_iter_())).unwrap()
        }
        fn unzip_<U, V>(self) -> ([U; $N], [V; $N]) where Self: $I<(U, V)> {
            let mut a: [MaybeUninit<U>; $N] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
            let mut b: [MaybeUninit<V>; $N] = unsafe {
                MaybeUninit::uninit().assume_init()
            };

            for ((x, y), (u, v)) in self.into_iter_().zip(a.iter_mut().zip(b.iter_mut())) {
                let _ = mem::replace(u, MaybeUninit::new(x));
                let _ = mem::replace(v, MaybeUninit::new(y));
            }

            unsafe { (
                ptr::read(a.as_ptr() as *const [U; $N]),
                ptr::read(b.as_ptr() as *const [V; $N]),
            ) }
        }
    }

    /// Iterator by values for array.
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

array_base!(2, Array2Ext, Array2IntoIter);
array_base!(3, Array3Ext, Array3IntoIter);
array_base!(4, Array4Ext, Array4IntoIter);
array_base!(5, Array5Ext, Array5IntoIter);
array_base!(6, Array6Ext, Array6IntoIter);
array_base!(7, Array7Ext, Array7IntoIter);
array_base!(8, Array8Ext, Array8IntoIter);
array_base!(9, Array9Ext, Array9IntoIter);
array_base!(10, Array10Ext, Array10IntoIter);
array_base!(11, Array11Ext, Array11IntoIter);
array_base!(12, Array12Ext, Array12IntoIter);
array_base!(13, Array13Ext, Array13IntoIter);
array_base!(14, Array14Ext, Array14IntoIter);
array_base!(15, Array15Ext, Array15IntoIter);
array_base!(16, Array16Ext, Array16IntoIter);


#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        rc::Rc,
        vec::Vec,
    };


    #[test]
    fn init_drop() {
        let a = <[_; 16]>::init_(|| Rc::new(()));
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
        let a = <[_; 16]>::init_(|| Rc::new(()));
        let b = a.clone();
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }

        let mut c = b.into_iter_().skip(8);
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
    fn iter_loop() {
        let a = <[_; 16]>::init_(|| Rc::new(()));
        let b = a.clone();
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 2);
        }

        let mut c = b.into_iter_();
        for x in &mut c {
            assert_eq!(Rc::strong_count(&x), 2);
        }

        mem::drop(c);
        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn from_iter() {
        let v = (0..16).map(|i| Rc::new(i)).collect::<Vec<_>>();
        let a = <[_; 16]>::from_iter_(&mut v.iter().cloned()).unwrap();

        for (i, x) in v.iter().enumerate() {
            assert_eq!(Rc::strong_count(x), 2);
            assert_eq!(**x, i);
        }
        mem::drop(a);

        assert!(<[_; 16]>::from_iter_(&mut v.iter().cloned().take(8)).is_none());
        for x in v.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn for_each() {
        let a = <[_; 16]>::from_iter_(&mut (0..16).map(|i| Rc::new(i))).unwrap();
        let b = a.clone();

        let mut i = 0;
        b.for_each_(|x| {
            assert_eq!(Rc::strong_count(&x), 2);
            assert_eq!(*x, i);
            i += 1;
        });

        for x in a.iter() {
            assert_eq!(Rc::strong_count(x), 1);
        }
    }

    #[test]
    fn map() {
        let a = <[usize; 16]>::from_iter_(&mut (0..16)).unwrap();

        for (i, x) in a.map_(|x| 15 - x).iter().enumerate() {
            assert_eq!(15 - i, *x);
        }
    }

    #[test]
    fn zip() {
        let a = <[i32; 16]>::from_iter_(&mut (0..16)).unwrap();
        let b = <[i8; 16]>::from_iter_(&mut (-16..0)).unwrap();
        let c = a.clone().zip_(b.clone());

        for ((x, y), (a, b)) in c.into_iter_().zip(a.iter().zip(b.iter())) {
            assert_eq!(x, *a);
            assert_eq!(y, *b);
        }
    }

    #[test]
    fn unzip() {
        let c = <[_; 16]>::from_iter_(&mut (0i32..16).zip(-16..0i8)).unwrap();
        let (a, b) = c.clone().unzip_();

        for ((x, y), (a, b)) in c.into_iter_().zip(a.iter().zip(b.iter())) {
            assert_eq!(x, *a);
            assert_eq!(y, *b);
        }
    }
}
