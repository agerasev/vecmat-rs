use crate::{Matrix, Vector};
use core::ops::{Index, IndexMut, Neg};
use num_traits::{Num, One, Zero, Inv};

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Zero,
{
    /// Create diagonal matrix.
    pub fn diagonal(diag: Vector<T, N>) -> Self {
        let mut iter = diag.into_iter();
        Matrix::indices().map(|(i, j)| {
            if i == j {
                iter.next().unwrap()
            } else {
                T::zero()
            }
        })
    }
}
impl<T, const N: usize> One for Matrix<T, N, N>
where
    T: One + Zero,
{
    /// Create identity matrix.
    fn one() -> Self {
        Matrix::indices().map(|(i, j)| if i == j { T::one() } else { T::zero() })
    }
}

// TODO: Implement when it will be possible to perform const generic arithmetics and specialization.
/*
impl<T, const N: usize> Matrix<T, N, N> where T: Copy, N > 1 {
    /// Take submatrix from original matrix.
    pub fn submatrix(&self, y: usize, x: usize) -> Matrix<T, N - 1, N - 1> {
        Matrix::indices().map(|(i, j)| self[(i + (i >= y) as usize, j + (j >= x) as usize)].clone())
    }
}
impl<T> Matrix<T, 1, 1> where T: Copy {
    /// Take submatrix from original matrix.
    pub fn submatrix(&self, y: usize, x: usize) -> T {
        self[(0, 0)].clone()
    }
}
*/

struct IndexMask<const N: usize> {
    data: [bool; N],
}

impl<const N: usize> IndexMask<N> {
    pub fn new() -> Self {
        Self { data: [true; N] }
    }
    pub fn find(&self, mut i: usize) -> usize {
        loop {
            if self.data[i] {
                break i;
            }
            i += 1;
        }
    }
}

impl<const N: usize> Index<usize> for IndexMask<N> {
    type Output = bool;
    fn index(&self, i: usize) -> &bool {
        &self.data[i]
    }
}
impl<const N: usize> IndexMut<usize> for IndexMask<N> {
    fn index_mut(&mut self, i: usize) -> &mut bool {
        &mut self.data[i]
    }
}

struct SubmatrixMask<const N: usize> {
    pub col: IndexMask<N>,
    pub row: IndexMask<N>,
    pub deg: usize,
}

impl<const N: usize> SubmatrixMask<N> {
    fn new() -> Self {
        Self {
            col: IndexMask::new(),
            row: IndexMask::new(),
            deg: N,
        }
    }
    fn exclude(&mut self, i: usize, j: usize) {
        self.col[i] = false;
        self.row[j] = false;
        self.deg -= 1;
    }
    fn include(&mut self, i: usize, j: usize) {
        self.col[i] = true;
        self.row[j] = true;
        self.deg += 1;
    }
}

struct Determinator<'a, T, const N: usize> {
    matrix: &'a Matrix<T, N, N>,
    mask: SubmatrixMask<N>,
}

impl<'a, T, const N: usize> Determinator<'a, T, N>
where
    T: Neg<Output = T> + Num + Copy,
{
    fn new(matrix: &'a Matrix<T, N, N>) -> Self {
        Self {
            matrix,
            mask: SubmatrixMask::new(),
        }
    }
    fn cofactor(&mut self, (i, ri): (usize, usize), (j, rj): (usize, usize)) -> T {
        self.mask.exclude(i, j);
        let mut a = self.det();
        if (ri + rj) % 2 != 0 {
            a = -a;
        }
        self.mask.include(i, j);
        a
    }
    fn det(&mut self) -> T {
        if self.mask.deg == 0 {
            T::one()
        } else {
            let i = self.mask.col.find(0);
            let mut j = 0;
            let mut a = T::zero();
            for rj in 0..self.mask.deg {
                j = self.mask.row.find(j);
                a = a + self.matrix[(i, j)] * self.cofactor((i, 0), (j, rj));
                j += 1;
            }
            a
        }
    }
}

impl<T, const N: usize> Matrix<T, N, N>
where
    T: Neg<Output = T> + Num + Copy,
{
    /// Cofactor of the matrix at (i, j).
    pub fn cofactor(&self, i: usize, j: usize) -> T {
        assert!(i < N && j < N);
        Determinator::new(self).cofactor((i, i), (j, j))
    }

    /// Determinant of the matrix.
    pub fn det(&self) -> T {
        Determinator::new(self).det()
    }

    /// Adjugate matrix.
    pub fn adj(&self) -> Self {
        Matrix::indices().map(|(i, j)| self.cofactor(j, i))
    }

    /// Inverse matrix.
    pub fn inv(&self) -> Self {
        self.adj() / self.det()
    }
}

impl<T, const N: usize> Inv for Matrix<T, N, N>
where
    T: Neg<Output = T> + Num + Copy,
{
    type Output = Self;

    fn inv(self) -> Self::Output {
        self.adj() / self.det()
    }
}
