use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use crate::lineararith::Sqrt;

#[derive(Debug, Clone, Copy)]
pub struct PartialDual<const N: usize> {
    pub val: f64,
    pub partial: [f64; N],
}

impl<const N: usize> PartialDual<N> {
    pub const fn constant(val: f64) -> Self {
        Self {
            val,
            partial: [0.0; N],
        }
    }

    pub const fn variable(val: f64, index: usize) -> Self {
        assert!(index < N);
        let mut partial = [0.0; N];
        partial[index] = 1.0;
        Self { val, partial }
    }

    pub const fn select(vars: [f64; N], index: usize) -> Self {
        assert!(index < N);
        Self::variable(vars[index], index)
    }

    pub fn recip(self) -> Self {
        let partial = self
            .partial
            .into_iter()
            .map(|a| -a / (self.val * self.val))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            val: self.val.recip(),
            partial,
        }
    }

    pub fn powu(self, n: u32) -> Self {
        let mut n = n;
        let mut v = self;
        let mut acc = PartialDual {
            val: 1.0,
            ..Default::default()
        };
        while n > 0 {
            if n & 1 == 1 {
                acc = acc * v;
            }
            n = n >> 1;
            v = v * v;
        }
        acc
    }
    pub fn powi(self, n: i32) -> Self {
        if n >= 0 {
            self.powu(n as u32)
        } else {
            self.recip().powu((-n) as u32)
        }
    }

    pub fn powf(self, v: f64) -> Self {
        let partial = self
            .partial
            .into_iter()
            .map(|a| a * v * self.val.powf(v - 1.0))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            val: self.val.powf(v),
            partial,
        }
    }
}

impl<const N: usize> Sqrt for PartialDual<N> {
    fn sqrt(self) -> Self {
        self.powf(0.5)
    }
}

impl<const N: usize> Default for PartialDual<N> {
    fn default() -> Self {
        Self {
            val: 0.0,
            partial: [0.0; N],
        }
    }
}

impl<const N: usize> Add<PartialDual<N>> for PartialDual<N> {
    type Output = PartialDual<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let partial = self
            .partial
            .into_iter()
            .zip(rhs.partial)
            .map(|(a, b)| a + b)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            val: self.val + rhs.val,
            partial,
        }
    }
}

impl<const N: usize> Sub<PartialDual<N>> for PartialDual<N> {
    type Output = PartialDual<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let partial = self
            .partial
            .into_iter()
            .zip(rhs.partial)
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            val: self.val - rhs.val,
            partial,
        }
    }
}

impl<const N: usize> Mul<PartialDual<N>> for PartialDual<N> {
    type Output = PartialDual<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let partial = self
            .partial
            .into_iter()
            .zip(rhs.partial)
            .map(|(a, b)| a * rhs.val + self.val * b)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            val: self.val * rhs.val,
            partial,
        }
    }
}

impl<const N: usize> Div<PartialDual<N>> for PartialDual<N> {
    type Output = PartialDual<N>;

    fn div(self, rhs: PartialDual<N>) -> Self::Output {
        rhs.recip() * self
    }
}

impl<const N: usize> Sum<PartialDual<N>> for PartialDual<N> {
    fn sum<I: Iterator<Item = PartialDual<N>>>(iter: I) -> Self {
        let mut acc = Self::default();
        for val in iter {
            acc = acc + val;
        }
        acc
    }
}

impl<const N: usize> From<f64> for PartialDual<N> {
    fn from(value: f64) -> Self {
        Self::constant(value)
    }
}
