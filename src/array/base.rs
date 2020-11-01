use core::{
    mem::{self, MaybeUninit},
    ptr,
};


/// Trait that extends array to be constructible element-by-element and iterated.
/// Postfix `_ext` is added to methods to avoid possible future ambiguity.
pub trait ArrayExt<T, const N: usize>: From<[T; N]> + Into<[T; N]> {
    /// IntoIterator by values for array.
    type IntoIter_: Iterator<Item=T>;

    /// Initializes an array with values from function.
    fn init_ext<F: FnMut() -> T>(f: F) -> Self;

    /// Conctructs an array from iterator.
    /// If iterator conatins less items than array, then `None` is returned.
    fn from_iter_ext<I: Iterator<Item=T>>(iter: &mut I) -> Option<Self>;

    /// Converts an array into iterator by values.
    fn into_iter_ext(self) -> Self::IntoIter_;

    fn for_each_ext<F: FnMut(T)>(self, f: F);
    fn map_ext<U, F: FnMut(T) -> U>(self, f: F) -> [U; N];
    fn zip_ext<U>(self, other: [U; N]) -> [(T, U); N];
    fn unzip_ext<U, V>(self) -> ([U; N], [V; N]) where Self: ArrayExt<(U, V), N>;
    fn fold_ext<S, F: FnMut(S, T) -> S>(self, s: S, f: F) -> S;
    fn fold_first_ext<F: FnMut(T, T) -> T>(self, f: F) -> T;
    fn scan_ext<S, U, F: FnMut(&mut S, T) -> U>(self, s: S, f: F) -> [U; N];
}

impl<T, const N: usize> ArrayExt<T, N> for [T; N] {
    type IntoIter_ = ArrayIntoIter<T, N>;

    fn init_ext<F: FnMut() -> T>(mut f: F) -> Self {
        let mut a: [MaybeUninit<T>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        for x in a.iter_mut() {
            *x = MaybeUninit::new(f());
        }

        // unsafe { mem::transmute::<_, [T; N]>(a) }
        unsafe { ptr::read(a.as_ptr() as *const [T; N]) }
    }

    fn from_iter_ext<I: Iterator<Item=T>>(iter: &mut I) -> Option<Self> {
        let mut a: [MaybeUninit<T>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        let mut pos: usize = 0;
        for (x, y) in a.iter_mut().zip(iter) {
            let _ = mem::replace(x, MaybeUninit::new(y));
            pos += 1;
        }

        if pos > N {
            unreachable!();
        } else if pos == N {
            Some(unsafe { ptr::read(a.as_ptr() as *const [T; N]) })
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

    fn into_iter_ext(self) -> ArrayIntoIter<T, N> {
        ArrayIntoIter::new(self.into())
    }

    fn for_each_ext<F: FnMut(T)>(self, f: F) {
        self.into_iter_ext().for_each(f)
    }
    fn map_ext<U, F: FnMut(T) -> U>(self, f: F) -> [U; N] {
        <[U; N]>::from_iter_ext(&mut self.into_iter_ext().map(f)).unwrap()
    }
    fn zip_ext<U>(self, other: [U; N]) -> [(T, U); N] {
        <[(T, U); N]>::from_iter_ext(&mut self.into_iter_ext().zip(other.into_iter_ext())).unwrap()
    }
    fn unzip_ext<U, V>(self) -> ([U; N], [V; N]) where Self: ArrayExt<(U, V), N> {
        let mut a: [MaybeUninit<U>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        let mut b: [MaybeUninit<V>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        for ((x, y), (u, v)) in self.into_iter_ext().zip(a.iter_mut().zip(b.iter_mut())) {
            let _ = mem::replace(u, MaybeUninit::new(x));
            let _ = mem::replace(v, MaybeUninit::new(y));
        }

        unsafe { (
            ptr::read(a.as_ptr() as *const [U; N]),
            ptr::read(b.as_ptr() as *const [V; N]),
        ) }
    }
    fn fold_ext<S, F: FnMut(S, T) -> S>(self, s: S, f: F) -> S {
        self.into_iter_ext().fold(s, f)
    }
    fn fold_first_ext<F: FnMut(T, T) -> T>(self, f: F) -> T {
        let mut t = self.into_iter_ext();
        let x = t.next().unwrap();
        t.fold(x, f)
    }
    fn scan_ext<S, U, F: FnMut(&mut S, T) -> U>(self, s: S, mut f: F) -> [U; N] {
        <[U; N]>::from_iter_ext(&mut self.into_iter_ext().scan(s, |s, x| Some(f(s, x)))).unwrap()
    }
}

/// Iterator by values for array.
pub struct ArrayIntoIter<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    pos: usize,
}
impl<T, const N: usize> ArrayIntoIter<T, N> {
    pub fn new(a: [T; N]) -> Self {
        let it = Self {
            data: unsafe {
                // unsafe { mem::transmute::<_, [MaybeUninit<T>; N]>(a) }
                ptr::read(a.as_ptr() as *const [MaybeUninit<T>; N])
            },
            pos: 0
        };
        mem::forget(a);
        it
    }
}
impl<T, const N: usize> Iterator for ArrayIntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.pos < N {
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
impl<T, const N: usize> Drop for ArrayIntoIter<T, N> {
    fn drop(&mut self) {
        unsafe {
            for x in self.data.get_unchecked_mut(self.pos..N) {
                mem::replace(x, MaybeUninit::uninit()).assume_init();
            }
        }
    }
}
