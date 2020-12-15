
macro_rules! vector_init { ($N:expr, $V:ident) => (
	/// Vector of fixed-size.
	#[repr(transparent)]
	#[derive(Clone, Copy, PartialEq)]
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
			Self { data: unsafe { ptr::read(&a as *const _ as *const [T; $N]) } }
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
		pub fn try_from_iter<I>(iter: I) -> Result<Self, ()> where I: Iterator<Item=T> {
			let mut a: $V<MaybeUninit<T>> = unsafe {
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
                Ok(unsafe { ptr::read(&a as *const _ as *const $V<T>) })
            } else {
                // Drop loaded items
                unsafe {
                    for x in a.as_mut().get_unchecked_mut(0..pos) {
                        mem::replace(x, MaybeUninit::uninit()).assume_init();
                    }
                }
                Err(())
            }
		}
	}

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

	impl<T> $V<T> {
		pub fn into_inner(self) -> [T; $N] {
			self.into()
		}
		pub fn as_ptr(&self) -> *const T {
			self.as_ref().as_ptr()
		}
		pub fn as_mut_ptr(&mut self) -> *mut T {
			self.as_mut().as_mut_ptr()
		}
		pub unsafe fn get_unchecked(&self, i: usize) -> &T {
			self.as_ref().get_unchecked(i)
		}
		pub unsafe fn get_unchecked_mut(&mut self, i: usize) -> &mut T {
			self.as_mut().get_unchecked_mut(i)
		}
	}
) }
