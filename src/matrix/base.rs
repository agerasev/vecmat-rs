
macro_rules! matrix_init { ($M:expr, $N:expr, $W:ident, $V:ident, $U:ident, $GI:ident) => (
	/// Matrix with fixed dimensions.
	#[repr(transparent)]
	#[derive(Clone, Copy, Debug, PartialEq)]
	pub struct $W<T> {
		data: $V<$U<T>>,
	}

	impl<T> $W<T> {
		/// Initialize matrix by closure.
		pub fn init<F: FnMut() -> T>(mut f: F) -> Self {
			Self { data: $V::init(|| $U::init(&mut f)) }
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

	impl<T> From<$V<$U<T>>> for $W<T> {
		fn from(a: $V<$U<T>>) -> Self {
			Self { data: a }
		}
	}
	impl<T> From<&$V<$U<T>>> for $W<T> where T: Clone {
		fn from(ar: &$V<$U<T>>) -> Self {
			Self { data: ar.clone() }
		}
	}

	impl<T> Into<$V<$U<T>>> for $W<T> {
		fn into(self) -> $V<$U<T>> {
			self.data
		}
	}
	impl<'a, T> Into<&'a $V<$U<T>>> for &'a $W<T> {
		fn into(self) -> &'a $V<$U<T>> {
			&self.data
		}
	}
	impl<'a, T> Into<&'a mut $V<$U<T>>> for &'a mut $W<T> {
		fn into(self) -> &'a mut $V<$U<T>> {
			&mut self.data
		}
	}

	impl<T> AsRef<$V<$U<T>>> for $W<T> {
		fn as_ref(&self) -> &$V<$U<T>> {
			&self.data
		}
	}
	impl<T> AsMut<$V<$U<T>>> for $W<T> {
		fn as_mut(&mut self) -> &mut $V<$U<T>> {
			&mut self.data
		}
	}

	impl<'a, T> TryFrom<&'a [T]> for $W<T> where T: Copy {
		type Error = ();
		fn try_from(s: &'a [T]) -> Result<Self, ()> {
			$W::try_from_iter(s.iter().cloned())
		}
	}
	impl<T> $W<T> {
		fn try_from_iter<I>(i: I) -> Result<Self, ()> where I: Iterator<Item=T> {
			<$V<$U<T>>>::try_from_iter(&mut $GI::new(i)).map(|a| a.into())
			.map(|a| Self { data: a })
		}
	}
) }

macro_rules! matrix_index { ($M:expr, $N:expr, $W:ident) => (
	impl<T> Index<(usize, usize)> for $W<T> {
		type Output = T;
		fn index(&self, (j, i): (usize, usize)) -> &T {
			&self.data[j][i]
		}
	}
	impl<T> IndexMut<(usize, usize)> for $W<T> {
		fn index_mut(&mut self, (j, i): (usize, usize)) -> &mut T {
			&mut self.data[j][i]
		}
	}
) }

macro_rules! matrix_iter { ($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
	impl <T> $W<T> {
		/// Returns iterator over matrix element refrences.
		pub fn iter(&self) -> impl Iterator<Item=&T> {
			FlatIter::new(self.data.iter()).unwrap()
		}
		/// Returns iterator over matrix element mutable refrences.
		pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
			FlatIter::new(self.data.iter_mut()).unwrap()
		}
	}

	impl<T> IntoIterator for $W<T> {
		type Item = T;
		type IntoIter = FlatIter<<$V<$U<T>> as IntoIterator>::IntoIter, $U<T>, <$U<T> as IntoIterator>::IntoIter>;
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self.data.into_iter()).unwrap()
		}
	}
	impl<'a, T> IntoIterator for &'a $W<T> {
		type Item = &'a T;
		type IntoIter = FlatIter<slice::Iter<'a, $U<T>>, &'a $U<T>, slice::Iter<'a, T>>;
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self.data.iter()).unwrap()
		}
	}
	impl<'a, T> IntoIterator for &'a mut $W<T> {
		type Item = &'a mut T;
		type IntoIter = FlatIter<slice::IterMut<'a, $U<T>>, &'a mut $U<T>, slice::IterMut<'a, T>>;
		fn into_iter(self) -> Self::IntoIter {
			Self::IntoIter::new(self.data.iter_mut()).unwrap()
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
		pub fn for_each<F: FnMut(T)>(self, mut f: F) {
			self.data.for_each(|a| a.for_each(&mut f))
		}
		/// Map matrix elements.
		pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> $W<U> {
			self.data.map(|a| a.map(&mut f)).into()
		}
		/// Zip two matrices into one.
		pub fn zip<U>(self, other: $W<U>) -> $W<(T, U)> {
			self.data.zip(other.data).map(|(a, b)| a.zip(b)).into()
		}
	}
	impl<T, U> $W<(T, U)> {
		/// Unzip matrix of tuples into two matrices.
		pub fn unzip(self) -> ($W<T>, $W<U>) {
			let z = self.data.map(|a| a.unzip()).unzip();
			(z.0.into(), z.1.into())
		}
	}
	impl<T> $W<T> {
		pub fn fold<S, F: Fn(S, T) -> S>(self, s: S, mut f: F) -> S {
			self.data.fold(s, |t, a| a.fold(t, &mut f))
		}
        pub fn fold_first<F: FnMut(T, T) -> T>(self, mut f: F) -> T {
			let mut iter = self.data.into_iter();
			let s = iter.next().unwrap().fold_first(&mut f);
			iter.fold(s, |t, a| a.fold(t, &mut f))
		}
        pub fn scan<S, U, F: FnMut(&mut S, T) -> U>(self, mut s: S, mut f: F) -> $W<U> {
			self.data.scan(&mut s, |r, a| a.scan(r, |r, x| f(*r, x))).into()
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

macro_rules! matrix_base { ($M:expr, $N:expr, $W:ident, $V:ident, $U:ident, $GI:ident) => (
	matrix_init!($M, $N, $W, $V, $U, $GI);
	matrix_index!($M, $N, $W);
	matrix_iter!($M, $N, $W, $V, $U);
	matrix_display!($M, $N, $W);
) }
