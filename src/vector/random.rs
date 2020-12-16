macro_rules! vector_random { ($N:expr, $V:ident, $D:ident) => (
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

    impl<T, D: Distribution<T> + ImplicitClone> Distribution<$V<T>> for $D<T, D> {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $V<T> {
            $V::init(|| rng.sample(self.inner.clone()))
        }
    }

    impl<T> Distribution<$V<T>> for StandardNormal where StandardNormal: Distribution<T> {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $V<T> {
            $D::<T, StandardNormal>::new(Self).sample(rng)
        }
    }

    impl<T: Float + ImplicitClone> Distribution<$V<T>> for NonZero where StandardNormal: Distribution<$V<T>> {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $V<T> {
            loop {
                let x = rng.sample(&StandardNormal);
                if x.clone().length() > T::epsilon() {
                    break x;
                }
            }
        }
    }

    impl<T: Float + ImplicitClone> Distribution<$V<T>> for Unit where NonZero: Distribution<$V<T>> {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $V<T> {
            rng.sample(&NonZero).normalize()
        }
    }
) }
