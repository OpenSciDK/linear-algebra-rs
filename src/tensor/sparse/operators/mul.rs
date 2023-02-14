use crate::{
    number::{c64, Number},
    sparse::SparseTensor,
};
use rayon::prelude::*;
use std::ops::{Mul, MulAssign};

fn mul_scalar<T>(lhs: T, rhs: SparseTensor<T>) -> SparseTensor<T>
where
    T: Number,
{
    let mut rhs = rhs;

    rhs.elems
        .par_iter_mut()
        .map(|r| {
            *r.1 *= lhs;
        })
        .collect::<Vec<_>>();

    rhs
}

fn mul<T>(lhs: SparseTensor<T>, rhs: &SparseTensor<T>) -> SparseTensor<T>
where
    T: Number,
{
    if !lhs.is_same_size(rhs) {
        panic!("Dimension mismatch.")
    }
    let mut lhs = lhs;

    todo!();

    lhs
}

// Scalar and SparseTensor

macro_rules! impl_div_scalar {
  {$t: ty} => {
      impl Mul<SparseTensor<$t>> for $t {
          type Output = SparseTensor<$t>;

          fn mul(self, rhs: SparseTensor<$t>) -> Self::Output {
              mul_scalar(self, rhs)
          }
      }

      impl Mul<SparseTensor<$t>> for &$t {
          type Output = SparseTensor<$t>;

          fn mul(self, rhs: SparseTensor<$t>) -> Self::Output {
              mul_scalar(*self, rhs)
          }
      }
  }
}

impl_div_scalar! {f64}
impl_div_scalar! {c64}

// SparseTensor and Scalar

impl<T> Mul<T> for SparseTensor<T>
where
    T: Number,
{
    type Output = SparseTensor<T>;

    fn mul(self, rhs: T) -> Self::Output {
        mul_scalar(rhs, self)
    }
}

impl<T> Mul<&T> for SparseTensor<T>
where
    T: Number,
{
    type Output = SparseTensor<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        mul_scalar(*rhs, self)
    }
}

// SparseTensor and SparseTensor

impl<T> Mul<SparseTensor<T>> for SparseTensor<T>
where
    T: Number,
{
    type Output = SparseTensor<T>;

    fn mul(self, rhs: SparseTensor<T>) -> Self::Output {
        mul(self, &rhs)
    }
}

impl<T> Mul<&SparseTensor<T>> for SparseTensor<T>
where
    T: Number,
{
    type Output = SparseTensor<T>;

    fn mul(self, rhs: &SparseTensor<T>) -> Self::Output {
        mul(self, rhs)
    }
}

impl<T> Mul<SparseTensor<T>> for &SparseTensor<T>
where
    T: Number,
{
    type Output = SparseTensor<T>;

    fn mul(self, rhs: SparseTensor<T>) -> Self::Output {
        mul(rhs, self)
    }
}

// MulAssign

impl<T> MulAssign<SparseTensor<T>> for SparseTensor<T>
where
    T: Number,
{
    fn mul_assign(&mut self, rhs: SparseTensor<T>) {
        *self = self as &Self * rhs;
    }
}
