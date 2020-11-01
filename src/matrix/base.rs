
macro_rules! matrix_init { ($M:expr, $N:expr, $W:ident) => (
	/// Matrix with fixed dimensions.
	#[repr(transparent)]
	#[derive(Clone, Copy, Debug, PartialEq)]
	pub struct $W<T> {
		data: [T; $M*$N],
	}

	impl<T> $W<T> {
		/// Initialize matrix by closure.
		pub fn init<F: FnMut() -> T>(f: F) -> Self {
			Self { data: <[T; $M*$N]>::init_ext(f) }
		}
	}
	impl<T> Default for $W<T> where T: Default {
		/// Create matrix filled with default values.
		fn default() -> Self {
			Self::init(|| T::default())
		}
	}
	impl<T> $W<T> where T: Default {
		/// Create default matrix.
		pub fn new() -> Self {
			Self::default()
		}
	}
	impl<T> $W<T> where T: Clone {
		/// Create matrix which elements are filled with scalar value.
		pub fn fill(v: T) -> Self {
			Self::init(|| v.clone())
		}
		/// Fill with a scalar value reference.
		pub fn fill_ref(v: &T) -> Self {
			Self::init(|| v.clone())
		}
	}

	impl<T> From<[T; $M*$N]> for $W<T> {
		fn from(a: [T; $M*$N]) -> Self {
			Self { data: a }
		}
	}
	impl<T> From<&[T; $M*$N]> for $W<T> where T: Clone {
		fn from(ar: &[T; $M*$N]) -> Self {
			Self { data: ar.clone() }
		}
	}

	impl<T> Into<[T; $M*$N]> for $W<T> {
		fn into(self) -> [T; $M*$N] {
			self.data
		}
	}
	impl<'a, T> Into<&'a [T; $M*$N]> for &'a $W<T> {
		fn into(self) -> &'a [T; $M*$N] {
			&self.data
		}
	}
	impl<'a, T> Into<&'a mut [T; $M*$N]> for &'a mut $W<T> {
		fn into(self) -> &'a mut [T; $M*$N] {
			&mut self.data
		}
	}

	impl<T> AsRef<[T; $M*$N]> for $W<T> {
		fn as_ref(&self) -> &[T; $M*$N] {
			&self.data
		}
	}
	impl<T> AsMut<[T; $M*$N]> for $W<T> {
		fn as_mut(&mut self) -> &mut [T; $M*$N] {
			&mut self.data
		}
	}

	impl<'a, T> TryFrom<&'a [T]> for $W<T> where T: Copy {
		type Error = TryFromSliceError;
		fn try_from(s: &'a [T]) -> Result<Self, Self::Error> {
			s.try_into().map(|a| Self { data: a })
		}
	}
	impl<T> $W<T> {
		fn try_from_iter<I>(mut i: I) -> Result<Self, ()> where I: Iterator<Item=T> {
			<[T; $M*$N]>::from_iter_ext(&mut i).map(|a| a.into()).ok_or(())
		}
	}
) }

macro_rules! matrix_index { ($M:expr, $N:expr, $W:ident) => (
	impl<T> Index<(usize, usize)> for $W<T> {
		type Output = T;
		fn index(&self, (j, i): (usize, usize)) -> &T {
			assert!(j < $M && i < $N);
			unsafe { self.data.get_unchecked(j*$N + i) }
		}
	}
	impl<T> IndexMut<(usize, usize)> for $W<T> {
		fn index_mut(&mut self, (j, i): (usize, usize)) -> &mut T {
			assert!(j < $M && i < $N);
			unsafe { self.data.get_unchecked_mut(j*$N + i) }
		}
	}
) }

macro_rules! matrix_iter { ($M:expr, $N:expr, $W:ident, $A:ident) => (
	impl <T> $W<T> {
		/// Returns iterator over matrix element refrences.
		pub fn iter(&self) -> impl Iterator<Item=&T> {
			self.data.iter()
		}
		/// Returns iterator over matrix element mutable refrences.
		pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
			self.data.iter_mut()
		}
	}

	impl<T> IntoIterator for $W<T> {
		type Item = T;
		type IntoIter = <[T; $M*$N] as $A<T>>::IntoIter_;
		fn into_iter(self) -> Self::IntoIter {
			self.data.into_iter_ext()
		}
	}
	impl<'a, T> IntoIterator for &'a $W<T> {
		type Item = &'a T;
		type IntoIter = slice::Iter<'a, T>;
		fn into_iter(self) -> Self::IntoIter {
			self.data.iter()
		}
	}
	impl<'a, T> IntoIterator for &'a mut $W<T> {
		type Item = &'a mut T;
		type IntoIter = slice::IterMut<'a, T>;
		fn into_iter(self) -> Self::IntoIter {
			self.data.iter_mut()
		}
	}

	impl $W<(usize, usize)> {
		/// Create matrix which elements are tuples (j, i) where j and i are coordinates of the matrix cell.
		pub fn indices() -> Self {
			Self::try_from_iter((0..($M*$N)).map(|x| (x / $N, x % $N))).unwrap()
		}
	}

	impl<T> $W<T> {
		/// Call closure for each element of the matrix passing it by value.
		pub fn for_each<F: FnMut(T)>(self, f: F) {
			self.data.for_each_ext(f)
		}
		/// Map matrix elements.
		pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> $W<U> {
			self.data.map_ext(f).into()
		}
		/// Zip two matrices into one.
		pub fn zip<U>(self, other: $W<U>) -> $W<(T, U)> {
			self.data.zip_ext(other.data).into()
		}
	}
	impl<T, U> $W<(T, U)> {
		/// Unzip matrix of tuples into two matrices.
		pub fn unzip(self) -> ($W<T>, $W<U>) {
			let z = self.data.unzip_ext();
			(z.0.into(), z.1.into())
		}
	}
	impl<T> $W<T> {
		pub fn fold<S, F: FnMut(S, T) -> S>(self, s: S, f: F) -> S {
			self.data.fold_ext(s, f)
		}
        pub fn fold_first<F: FnMut(T, T) -> T>(self, f: F) -> T {
			self.data.fold_first_ext(f)
		}
        pub fn scan<S, U, F: FnMut(&mut S, T) -> U>(self, s: S, f: F) -> $W<U> {
			self.data.scan_ext(s, f).into()
		}
	}
) }

macro_rules! matrix_display { ($M:expr, $N:expr, $W:ident) => (
	impl<T> Display for $W<T> where T: Display {
		fn fmt(&self, f: &mut Formatter) -> FmtResult {
			writeln!(f)?;
			writeln!(f, "{}(", stringify!($W))?;
			for j in 0..$M {
				write!(f, "  ")?;
				for i in 0..$N {
					write!(f, "{}, ", self[(j, i)])?;
				}
				writeln!(f)?;
			}
			writeln!(f, ")")?;
			Ok(())
		}
	}
) }

macro_rules! matrix_base { ($M:expr, $N:expr, $W:ident, $A:ident) => (
	matrix_init!($M, $N, $W);
	matrix_index!($M, $N, $W);
	matrix_iter!($M, $N, $W, $A);
	matrix_display!($M, $N, $W);
) }
