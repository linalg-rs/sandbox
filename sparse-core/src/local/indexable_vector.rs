//! An Indexable Vector is a container whose elements can be 1d indexed.
use num::{Float, Zero};
use sparse_traits::linalg::traits::*;
use sparse_traits::linalg::IndexableVector;
use sparse_traits::types::{Error, Result};
use sparse_traits::Scalar;
use sparse_traits::{IndexLayout, IndexType};

use super::index_layout::LocalIndexLayout;

pub struct LocalIndexableVector<'a, T: Scalar> {
    data: Vec<T>,
    index_layout: &'a LocalIndexLayout,
}

impl<'a, T: Scalar> LocalIndexableVector<'a, T> {
    pub fn new(index_layout: &'a LocalIndexLayout) -> LocalIndexableVector<'a, T> {
        LocalIndexableVector {
            data: vec![T::zero(); index_layout.number_of_global_indices()],
            index_layout,
        }
    }
}

impl<T: Scalar> IndexableVector for LocalIndexableVector<'_, T> {
    type Ind = LocalIndexLayout;
    type Iter<'b> = std::slice::Iter<'b, T> where Self: 'b;

    type IterMut<'b> = std::slice::IterMut<'b, T> where Self: 'b;
    type T = T;

    fn get(&self, index: IndexType) -> Option<&Self::T> {
        self.data.get(index)
    }

    fn get_mut(&mut self, index: IndexType) -> Option<&mut Self::T> {
        self.data.get_mut(index)
    }

    unsafe fn get_unchecked(&self, index: IndexType) -> &Self::T {
        self.data.get_unchecked(index)
    }

    unsafe fn get_unchecked_mut(&mut self, index: IndexType) -> &mut Self::T {
        self.data.get_unchecked_mut(index)
    }

    fn index_layout(&self) -> &Self::Ind {
        &self.index_layout
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.data.as_slice().iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.data.as_mut_slice().iter_mut()
    }

    fn len(&self) -> IndexType {
        self.index_layout.number_of_global_indices()
    }
}

impl<T: Scalar> Inner for LocalIndexableVector<'_, T> {
    type T = T;
    fn inner(&self, other: &Self) -> Result<Self::T> {
        if self.len() != other.len() {
            return Err(Error::OperationFailed);
        }
        let result = self
            .iter()
            .zip(other.iter())
            .fold(<Self::T as Zero>::zero(), |acc, (&first, &second)| {
                acc + first * second.conj()
            });
        Ok(result)
    }
}

impl<T: Scalar> AbsSquareSum for LocalIndexableVector<'_, T> {
    type T = T;
    fn square_sum(&self) -> <Self::T as Scalar>::Real {
        self.iter()
            .fold(<<Self::T as Scalar>::Real>::zero(), |acc, &elem| {
                acc + elem.square()
            })
    }
}

impl<T: Scalar> Norm1 for LocalIndexableVector<'_, T> {
    type T = T;
    fn norm_1(&self) -> <Self::T as Scalar>::Real {
        self.iter()
            .fold(<<Self::T as Scalar>::Real>::zero(), |acc, &elem| {
                acc + elem.abs()
            })
    }
}

impl<T: Scalar> Norm2 for LocalIndexableVector<'_, T> {
    type T = T;
    fn norm_2(&self) -> <Self::T as Scalar>::Real {
        <<Self::T as Scalar>::Real as Float>::sqrt(self.square_sum())
    }
}

impl<T: Scalar> NormInf for LocalIndexableVector<'_, T> {
    type T = T;
    fn norm_inf(&self) -> <Self::T as Scalar>::Real {
        self.iter().fold(
            <<Self::T as Scalar>::Real as Float>::neg_infinity(),
            |acc, &elem| <<Self::T as Scalar>::Real as Float>::max(acc, elem.abs()),
        )
    }
}

impl<T: Scalar> Swap for LocalIndexableVector<'_, T> {
    type T = T;
    fn swap(&mut self, other: &mut Self) -> sparse_traits::types::Result<()> {
        if self.len() != other.len() {
            Err(Error::OperationFailed)
        } else {
            for (first, second) in self.iter_mut().zip(other.iter_mut()) {
                std::mem::swap(first, second);
            }
            Ok(())
        }
    }
}

impl<T: Scalar> Fill for LocalIndexableVector<'_, T> {
    type T = T;
    fn fill(&mut self, other: &Self) -> sparse_traits::types::Result<()> {
        if self.len() != other.len() {
            Err(Error::OperationFailed)
        } else {
            for (first, second) in self.iter_mut().zip(other.iter()) {
                *first = *second;
            }
            Ok(())
        }
    }
}

impl<T: Scalar> ScalarMult for LocalIndexableVector<'_, T> {
    type T = T;
    fn scalar_mult(&mut self, scalar: Self::T) {
        for elem in self.iter_mut() {
            *elem *= scalar;
        }
    }
}

impl<T: Scalar> Axpy for LocalIndexableVector<'_, T> {
    type T = T;
    fn axpy(&mut self, other: &Self, scalar: Self::T) -> sparse_traits::types::Result<()> {
        if self.len() != other.len() {
            Err(Error::OperationFailed)
        } else {
            for (first, second) in self.iter_mut().zip(other.iter()) {
                *first += scalar * *second;
            }
            Ok(())
        }
    }
}

impl<'a, T: Scalar> CreateFrom<'a> for LocalIndexableVector<'a, T> {
    type T = T;
    type Ind = super::index_layout::LocalIndexLayout;
    fn create_from<'b>(index_layout: &'b Self::Ind, scalar: Self::T) -> Self
    where
        'b: 'a,
    {
        Self {
            data: vec![scalar; index_layout.number_of_global_indices()],
            index_layout,
        }
    }
}
