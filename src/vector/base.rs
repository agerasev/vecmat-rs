
macro_rules! vector_init { ($N:expr, $V:ident) => (
	/// Vector of fixed-size.
	#[repr(transparent)]
	#[derive(Clone, Copy, Debug, PartialEq)]
	pub struct $V<T> {
		data: [T; $N],
	}

	impl<T> $V<T> {
		/// Initialize a vector with values from closure.
		pub fn init<F: FnMut() -> T>(mut f: F) -> Self {
			let mut a: [MaybeUninit<T>; $N] = unsafe {
                MaybeUninit::uninit().assume_init()
            };

            for x in a.iter_mut() {
                *x = MaybeUninit::new(f());
            }

            // unsafe { mem::transmute::<_, [T; $N]>(a) }
			Self { data: unsafe { ptr::read(a.as_ptr() as *const [T; $N]) } }
		}
	}

	impl<T> Default for $V<T> where T: Default {
		/// Create vector filled with default values.
		fn default() -> Self {
			Self::init(|| T::default())
		}
	}
	impl<T> $V<T> where T: Default {
		/// Create default vector.
		pub fn new() -> Self {
			Self::default()
		}
	}
	impl<T> $V<T> where T: Clone {
		/// Create vector which elements are filled with scalar value.
		pub fn fill(v: T) -> Self {
			Self::init(|| v.clone())
		}
		/// Fill with a scalar value reference.
		pub fn fill_ref(v: &T) -> Self {
			Self::init(|| v.clone())
		}
	}

	impl<T> From<[T; $N]> for $V<T> {
		fn from(a: [T; $N]) -> Self {
			Self { data: a }
		}
	}
	impl<T> From<&[T; $N]> for $V<T> where T: Clone {
		fn from(ar: &[T; $N]) -> Self {
			Self { data: ar.clone() }
		}
	}

	impl<T> Into<[T; $N]> for $V<T> {
		fn into(self) -> [T; $N] {
			self.data
		}
	}
	impl<'a, T> Into<&'a [T; $N]> for &'a $V<T> {
		fn into(self) -> &'a [T; $N] {
			&self.data
		}
	}
	impl<'a, T> Into<&'a mut [T; $N]> for &'a mut $V<T> {
		fn into(self) -> &'a mut [T; $N] {
			&mut self.data
		}
	}

	impl<T> AsRef<[T; $N]> for $V<T> {
		fn as_ref(&self) -> &[T; $N] {
			&self.data
		}
	}
	impl<T> AsMut<[T; $N]> for $V<T> {
		fn as_mut(&mut self) -> &mut [T; $N] {
			&mut self.data
		}
	}

	impl<'a, T> TryFrom<&'a [T]> for $V<T> where T: Copy {
		type Error = ();
		fn try_from(s: &'a [T]) -> Result<Self, Self::Error> {
			s.try_into().map(|a| Self { data: a }).map_err(|_| ())
		}
	}
	impl<T> $V<T> {
		/// Try to conctruct a vector from iterator.
        /// If iterator conatins less items than vector, then `Err` is returned.
		///
		/// *FIXME: Implement `TryFrom` without conflict.*
		fn try_from_iter<I>(iter: I) -> Result<Self, ()> where I: Iterator<Item=T> {
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
                Ok(Self { data: unsafe { ptr::read(a.as_ptr() as *const [T; $N]) } })
            } else {
                // Drop loaded items
                unsafe {
                    for x in a.get_unchecked_mut(0..pos) {
                        mem::replace(x, MaybeUninit::uninit()).assume_init();
                    }
                }
                Err(())
            }
		}
	}
) }

macro_rules! vector_index { ($N:expr, $V:ident) => (
	impl<T> Index<usize> for $V<T> {
		type Output = T;
		fn index(&self, i: usize) -> &T {
			&self.data[i]
		}
	}
	impl<T> IndexMut<usize> for $V<T> {
		fn index_mut(&mut self, i: usize) -> &mut T {
			&mut self.data[i]
		}
	}
) }

macro_rules! vector_iter { ($N:expr, $V:ident, $II:ident) => (
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

	impl <T> $V<T> {
		/// Returns iterator over vector element refrences.
		pub fn iter(&self) -> impl Iterator<Item=&T> {
			self.data.iter()
		}
		/// Returns iterator over vector element mutable refrences.
		pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
			self.data.iter_mut()
		}
	}

	impl<T> IntoIterator for $V<T> {
		type Item = T;
		type IntoIter = $II<T>;
		fn into_iter(self) -> Self::IntoIter {
			$II::new(self.data)
		}
	}
	impl<'a, T> IntoIterator for &'a $V<T> {
		type Item = &'a T;
		type IntoIter = slice::Iter<'a, T>;
		fn into_iter(self) -> Self::IntoIter {
			self.data.iter()
		}
	}
	impl<'a, T> IntoIterator for &'a mut $V<T> {
		type Item = &'a mut T;
		type IntoIter = slice::IterMut<'a, T>;
		fn into_iter(self) -> Self::IntoIter {
			self.data.iter_mut()
		}
	}

	impl $V<usize> {
		/// Create vector which element value equals to its position in vector.
		pub fn indices() -> Self {
			Self::try_from_iter(0..$N).unwrap()
		}
	}

	impl<T> $V<T> {
		/// Call closure for each element of the vector passing it by value.
		pub fn for_each<F: FnMut(T)>(self, f: F) {
			self.into_iter().for_each(f)
		}
		/// Map vector elements.
		pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> $V<U> {
			$V::try_from_iter(&mut self.into_iter().map(f)).unwrap()
		}
		/// Zip two vectors into one.
		pub fn zip<U>(self, other: $V<U>) -> $V<(T, U)> {
			$V::try_from_iter(&mut self.into_iter().zip(other.into_iter())).unwrap()
		}
		/// Enumerate vector elements.
		pub fn enumerate(self) -> $V<(usize, T)> {
			$V::indices().zip(self)
		}
	}
	impl<T, U> $V<(T, U)> {
		/// Unzip vector of tuples into two vectors.
		pub fn unzip(self) -> ($V<T>, $V<U>) {
			let mut a: [MaybeUninit<T>; $N] = unsafe {
                MaybeUninit::uninit().assume_init()
            };
            let mut b: [MaybeUninit<U>; $N] = unsafe {
                MaybeUninit::uninit().assume_init()
            };

            for ((x, y), (u, v)) in self.into_iter().zip(a.iter_mut().zip(b.iter_mut())) {
                let _ = mem::replace(u, MaybeUninit::new(x));
                let _ = mem::replace(v, MaybeUninit::new(y));
            }

            unsafe { (
                $V { data: ptr::read(a.as_ptr() as *const [T; $N]) },
                $V { data: ptr::read(b.as_ptr() as *const [U; $N]) },
            ) }
		}
	}
	impl<T> $V<T> {
		pub fn fold<S, F: FnMut(S, T) -> S>(self, s: S, f: F) -> S {
			self.into_iter().fold(s, f)
		}
        pub fn fold_first<F: FnMut(T, T) -> T>(self, f: F) -> T {
			let mut t = self.into_iter();
            let x = t.next().unwrap();
            t.fold(x, f)
		}
        pub fn scan<S, U, F: FnMut(&mut S, T) -> U>(self, s: S, mut f: F) -> $V<U> {
			$V::try_from_iter(&mut self.into_iter().scan(s, |s, x| Some(f(s, x)))).unwrap()
		}
	}
) }

macro_rules! vector_display { ($N:expr, $V:ident) => (
	impl<T> Display for $V<T> where T: Display {
		fn fmt(&self, f: &mut Formatter) -> FmtResult {
			write!(f, "{}(", stringify!($V))?;
			for i in 0..$N-1 {
				write!(f, "{}, ", self[i])?;
			}
			write!(f, "{})", self[$N-1])?;
			Ok(())
		}
	}
) }

macro_rules! vector_base { ($N:expr, $V:ident, $II:ident) => (
	vector_init!($N, $V);
	vector_index!($N, $V);
	vector_iter!($N, $V, $II);
	vector_display!($N, $V);
) }
