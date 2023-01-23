use crate::types::Scalar;

pub trait Inner {
    type T: Scalar;
    fn inner(&self, other: &Self) -> crate::types::Result<Self::T>;
}

pub trait SquareSum {
    type T: Scalar;
    fn square_sum(&self) -> <Self::T as Scalar>::Real;
}

pub trait Norm1 {
    type T: Scalar;
    fn norm_1(&self) -> <Self::T as Scalar>::Real;
}

pub trait Norm2 {
    type T: Scalar;
    fn norm_2(&self) -> <Self::T as Scalar>::Real;
}

pub trait NormInf {
    type T: Scalar;
    fn norm_inf(&self) -> <Self::T as Scalar>::Real;
}
