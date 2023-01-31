//! An indexable vector is the standard type for n-dimensional containers

use crate::types::{IndexType, Scalar};
use crate::IndexLayout;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockResult};

pub trait IndexableVector {
    type T: Scalar;
    type Ind: IndexLayout;

    type Data: IndexableVectorData;

    fn data(&self) -> Option<&RwLock<Self::Data>>;

    fn try_read(&self) -> TryLockResult<RwLockReadGuard<'_, Self::Data>> {
        self.data().unwrap().try_read()
    }
    fn try_write(&self) -> TryLockResult<RwLockWriteGuard<'_, Self::Data>> {
        self.data().unwrap().try_write()
    }

    fn index_layout(&self) -> &Self::Ind;
}

pub trait IndexableVectorData {
    type T: Scalar;
    type Iter<'a>: std::iter::Iterator<Item = &'a Self::T>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;

    fn get(&self, index: IndexType) -> Option<&Self::T>;

    unsafe fn get_unchecked(&self, index: IndexType) -> &Self::T;

    fn len(&self) -> IndexType;

    fn data(&self) -> &[Self::T];
}

pub trait IndexableVectorDataMut: IndexableVectorData {
    type IterMut<'a>: std::iter::Iterator<Item = &'a mut Self::T>
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    fn get_mut(&mut self, index: IndexType) -> Option<&mut Self::T>;

    unsafe fn get_unchecked_mut(&mut self, index: IndexType) -> &mut Self::T;

    fn data_mut(&mut self) -> &mut [Self::T];
}
