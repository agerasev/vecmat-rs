
macro_rules! array_iter_mod { ($N:expr, $A:ident, $GI:ident, $FI:ident) => (
    pub struct $GI<I: Iterator> {
        iter: I,
    }
    impl<I> $GI<I> where I: Iterator {
        pub fn new(iter: I) -> Self {
            Self { iter }
        }
    }
    impl<I> Iterator for $GI<I> where I: Iterator {
        type Item = [I::Item; $N];
        fn next(&mut self) -> Option<Self::Item> {
            <Self::Item as $A<I::Item>>::from_iter_ext(&mut self.iter)
        }
    }

    pub struct $FI<I, IT, II> where I: Iterator<Item=IT>, IT: IntoIterator<IntoIter=II>, II: Iterator {
        iter: I,
        subiter: II,
    }
    impl<I, IT, II> $FI<I, IT, II> where I: Iterator<Item=IT>, IT: IntoIterator<IntoIter=II>, II: Iterator {
        pub fn new(iter: I) -> Option<Self> {
            iter.next().map(|a| Self { iter, subiter: a.into_iter_ext() })
        }
    }
    impl<I, IT, II> Iterator for $FI<I, IT, II> where I: Iterator<Item=IT>, IT: IntoIterator<IntoIter=II>, II: Iterator {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.subiter.next().or_else(|| {
                self.iter.next().and_then(|a| {
                    self.subiter = a.into_iter_ext();
                    self.subiter.next()
                })
            })
        }
    }
)}
