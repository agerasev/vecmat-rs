macro_rules! matrix_random { ($M:expr, $N:expr, $W:ident, $D:ident) => (
    #[derive(ImplicitClone, Copy, Debug, PartialEq)]
    pub struct $D<T, D: Distribution<T>> {
        pub inner: D,
        phantom: PhantomData<T>,
    }

    impl<T, D: Distribution<T>> $D<T, D> {
        pub fn new(inner: D) -> Self {
            Self { inner, phantom: PhantomData }
        }
    }

    impl<T, D: Distribution<T> + ImplicitClone> Distribution<$W<T>> for $D<T, D> {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $W<T> {
            $W::init(|| rng.sample(self.inner.clone()))
        }
    }

    impl<T> Distribution<$W<T>> for StandardNormal where StandardNormal: Distribution<T> {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $W<T> {
            $D::<T, StandardNormal>::new(Self).sample(rng)
        }
    }
) }
