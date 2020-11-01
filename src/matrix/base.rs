
macro_rules! matrix_init { ($M:expr, $N:expr, $V:ident) => (
	/// Vector of fixed-size.
	#[repr(transparent)]
	#[derive(Clone, Copy, Debug, PartialEq)]
	pub struct $V<T> {
		data: [T; $N],
	}

	impl<T> $V<T> {
		/// Initialize vector by closure.
		pub fn init<F: FnMut() -> T>(f: F) -> Self {
			Self { data: <[T; $N]>::init_ext(f) }
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
		type Error = TryFromSliceError;
		fn try_from(s: &'a [T]) -> Result<Self, Self::Error> {
			s.try_into().map(|a| Self { data: a })
		}
	}
	impl<T> $V<T> {
		fn try_from_iter<I>(mut i: I) -> Result<Self, ()> where I: Iterator<Item=T> {
			<[T; $N]>::from_iter_ext(&mut i).map(|a| a.into()).ok_or(())
		}
	}
) }

macro_rules! matrix_index { ($M:expr, $N:expr, $V:ident) => (
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

macro_rules! matrix_iter { ($M:expr, $N:expr, $V:ident, $A:ident) => (
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
		type IntoIter = <[T; $N] as $A<T>>::IntoIter_;
		fn into_iter(self) -> Self::IntoIter {
			self.data.into_iter_ext()
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
		pub fn range() -> Self {
			Self::try_from_iter(0..$N).unwrap()
		}
	}

	impl<T> $V<T> {
		/// Call closure for each element of the vector passing it by value.
		pub fn for_each<F: FnMut(T)>(self, f: F) {
			self.data.for_each_ext(f)
		}
		/// Map vector elements.
		pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> $V<U> {
			self.data.map_ext(f).into()
		}
		/// Zip two vectors into one.
		pub fn zip<U>(self, other: $V<U>) -> $V<(T, U)> {
			self.data.zip_ext(other.data).into()
		}
	}
	impl<T, U> $V<(T, U)> {
		/// Unzip vector of tuples into two vectors.
		pub fn unzip(self) -> ($V<T>, $V<U>) {
			let z = self.data.unzip_ext();
			(z.0.into(), z.1.into())
		}
	}
	impl<T> $V<T> {
		pub fn fold<S, F: FnMut(S, T) -> S>(self, s: S, f: F) -> S {
			self.data.fold_ext(s, f)
		}
        pub fn fold_first<F: FnMut(T, T) -> T>(self, f: F) -> T {
			self.data.fold_first_ext(f)
		}
        pub fn scan<S, U, F: FnMut(&mut S, T) -> U>(self, s: S, f: F) -> [U; $N] {
			self.data.scan_ext(s, f).into()
		}
	}
) }

macro_rules! matrix_display { ($M:expr, $N:expr, $V:ident) => (
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

macro_rules! matrix_base { ($M:expr, $N:expr, $V:ident, $A:ident) => (
	matrix_init!($M, $N, $V);
	//matrix_index!($M, $N, $V);
	//matrix_iter!($M, $N, $V, $A);
	//matrix_display!($M, $N, $V);
) }
