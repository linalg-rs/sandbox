//! An indexable vector is the standard type for n-dimensional containers
use crate::types::{IndexType, Scalar};
use crate::IndexLayout;

pub trait IndexableVector {
    type T: Scalar;
    type Iter<'a>: std::iter::Iterator<Item = &'a Self::T>
    where
        Self: 'a;
    type IterMut<'a>: std::iter::Iterator<Item = &'a mut Self::T>
    where
        Self: 'a;

    type Ind: IndexLayout;

    fn iter(&self) -> Self::Iter<'_>;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    fn get(&self, index: IndexType) -> Option<&Self::T>;

    fn get_mut(&mut self, index: IndexType) -> Option<&mut Self::T>;

    unsafe fn get_unchecked(&self, index: IndexType) -> &Self::T;
    unsafe fn get_unchecked_mut(&mut self, index: IndexType) -> &mut Self::T;

    fn len(&self) -> IndexType;

    fn index_layout(&self) -> &Self::Ind;
}

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
