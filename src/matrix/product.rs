
macro_rules! matrix_outer { ($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
    impl<T> Outer<$U<T>> for $V<T> where T: Mul<Output=T> + Clone {
        type Output = $W<T>;
        fn outer(self, other: $U<T>) -> Self::Output {
            $W::indices().map(|(i, j)| {
                self[i].clone() * other[j].clone()
            })
        }
    }
) }

macro_rules! matrix_row_col { ($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
    impl<T> $W<T> {
        pub fn row_ref(&self, i: usize) -> &$U<T> {
            &self.as_ref()[i]
        }
        pub fn row_mut(&mut self, i: usize) -> &mut $U<T> {
            &mut self.as_mut()[i]
        }
    }
    impl<T> $W<T> where T: Clone {
        pub fn row(&self, i: usize) -> $U<T> {
            self.row_ref(i).clone()
        }
        pub fn col(&self, j: usize) -> $V<T> {
            $V::indices().map(|i| self[(i, j)].clone())
        }
    }
) }


macro_rules! matrix_dot_mv { ($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
    impl<T> Dot<$U<T>> for $W<T> where T: Mul<Output=T> + Add<Output=T> + Clone {
        type Output = $V<T>;
        fn dot(self, vec: $U<T>) -> Self::Output {
            $V::indices().map(|i| { self.row(i).clone().dot(vec.clone()) })
        }
    }
) }

macro_rules! matrix_dot_vm { ($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
    impl<T> Dot<$W<T>> for $V<T> where T: Mul<Output=T> + Add<Output=T> + Clone {
        type Output = $U<T>;
        fn dot(self, mat: $W<T>) -> Self::Output {
            $U::indices().map(|j| { self.clone().dot(mat.col(j).clone()) })
        }
    }
) }

macro_rules! matrix_product_vec { ($M:expr, $N:expr, $W:ident, $V:ident, $U:ident) => (
    matrix_outer!($M, $N, $W, $V, $U);
    matrix_row_col!($M, $N, $W, $V, $U);
    matrix_dot_mv!($M, $N, $W, $V, $U);
    matrix_dot_vm!($M, $N, $W, $V, $U);
) }


macro_rules! matrix_dot { ($L:expr, $M:expr, $N:expr, $Wlm:ident, $Wmn:ident, $Wln:ident) => (
    impl<T> Dot<$Wmn<T>> for $Wlm<T> where T: Mul<Output=T> + Add<Output=T> + Clone {
        type Output = $Wln<T>;
        fn dot(self, mat: $Wmn<T>) -> Self::Output {
            $Wln::indices().map(|(i, j)| self.row(i).clone().dot(mat.col(j).clone()))
        }
    }
) }
