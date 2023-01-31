//! This module defines typical traits for linear algebra operations.

use crate::types::Scalar;

/// Inner product with another object.
pub trait Inner {
    type T: Scalar;
    fn inner(&self, other: &Self) -> crate::types::Result<Self::T>;
}

/// Take the sum of the squares of the absolute values of the entries.
pub trait AbsSquareSum {
    type T: Scalar;
    fn abs_square_sum(&self) -> <Self::T as Scalar>::Real;
}

/// Return the 1-Norm (Sum of absolute values of the entries).
pub trait Norm1 {
    type T: Scalar;
    fn norm_1(&self) -> <Self::T as Scalar>::Real;
}

/// Return the 2-Norm (Sqrt of the sum of squares).
pub trait Norm2 {
    type T: Scalar;
    fn norm_2(&self) -> <Self::T as Scalar>::Real;
}

/// Return the supremum norm (largest absolute value of the entries).
pub trait NormInfty {
    type T: Scalar;
    fn norm_infty(&self) -> <Self::T as Scalar>::Real;
}

/// Swap entries with another vector.
pub trait Swap {
    type T: Scalar;
    fn swap(&mut self, other: &mut Self) -> crate::types::Result<()>;
}

/// Fill vector by copying from another vector.
pub trait Fill {
    type T: Scalar;
    fn fill(&mut self, other: &Self) -> crate::types::Result<()>;
}

/// Multiply entries with a scalar.
pub trait ScalarMult {
    type T: Scalar;
    fn scalar_mult(&mut self, scalar: Self::T);
}

/// Compute self -> alpha * other + self.
pub trait MultSumInto {
    type T: Scalar;
    fn mult_sum_into(&mut self, other: &Self, scalar: Self::T) -> crate::types::Result<()>;
}
