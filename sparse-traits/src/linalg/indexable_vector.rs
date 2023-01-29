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

    fn new_from(&self) -> Self;

}

