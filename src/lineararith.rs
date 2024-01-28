use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Mul, Sub},
};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Vector<T, N>
where
    T: Sum<T> + Mul<T, Output = T>,
{
    pub fn dot(self, rhs: Self) -> T {
        self.0.into_iter().zip(rhs.0).map(|(a, b)| a * b).sum()
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Sum<T> + Mul<T, Output = T> + Clone,
{
    pub fn length_square(self) -> T {
        let rhs = self.clone();
        self.dot(rhs)
    }
}
impl<T, const N: usize> Vector<T, N>
where
    T: Sum<T> + Mul<T, Output = T> + Clone + Sqrt,
{
    pub fn length(self) -> T {
        self.length_square().sqrt()
    }
}

impl<T, const N: usize> Add<Vector<T, N>> for Vector<T, N>
where
    T: Add<T, Output = T> + Debug,
{
    type Output = Vector<T, N>;

    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        Self(
            self.0
                .into_iter()
                .zip(rhs.0)
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<T, const N: usize> Sub<Vector<T, N>> for Vector<T, N>
where
    T: Sub<T, Output = T> + Debug,
{
    type Output = Vector<T, N>;

    fn sub(self, rhs: Vector<T, N>) -> Self::Output {
        Self(
            self.0
                .into_iter()
                .zip(rhs.0)
                .map(|(a, b)| a - b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Mul<T, Output = T> + Clone,
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0.map(|a| a * rhs.clone()))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const N: usize, const M: usize>(pub [Vector<T, N>; M]);

impl<T, const N: usize, const M: usize> Add<Matrix<T, N, M>> for Matrix<T, N, M>
where
    T: Add<T, Output = T> + Debug,
{
    type Output = Matrix<T, N, M>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0
                .into_iter()
                .zip(rhs.0)
                .map(|(a, b)| a + b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<T, const N: usize, const M: usize> Sub<Matrix<T, N, M>> for Matrix<T, N, M>
where
    T: Sub<T, Output = T> + Debug,
{
    type Output = Matrix<T, N, M>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(
            self.0
                .into_iter()
                .zip(rhs.0)
                .map(|(a, b)| a - b)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<T, const N: usize, const M: usize> Mul<Vector<T, M>> for Matrix<T, N, M>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Clone + Debug,
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: Vector<T, M>) -> Self::Output {
        assert!(M > 0);
        self.0
            .into_iter()
            .zip(rhs.0)
            .map(|(v, s)| v * s)
            .reduce(|acc, v| acc + v)
            .unwrap()
    }
}

impl<T, const N: usize, const M: usize, const K: usize> Mul<Matrix<T, M, K>> for Matrix<T, N, M>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Clone + Debug,
{
    type Output = Matrix<T, N, K>;

    fn mul(self, rhs: Matrix<T, M, K>) -> Self::Output {
        Matrix(rhs.0.map(|v| self.clone() * v))
    }
}
