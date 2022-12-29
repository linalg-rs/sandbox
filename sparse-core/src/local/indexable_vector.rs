//! An Indexable Vector is a container whose elements can be 1d indexed.
use num::{Float, Zero};
use sparse_traits::types::{Error, Result};
use sparse_traits::IndexableVector;
use sparse_traits::Scalar;
use sparse_traits::{IndexSet, IndexType};
use sparse_traits::{Inner, Norm1, Norm2, NormInf, SquareSum};

use super::index_set::LocalIndexSet;

pub struct LocalIndexableVector<T: Scalar> {
    data: Vec<T>,
    index_set: LocalIndexSet,
}

impl<T: Scalar> LocalIndexableVector<T> {
    pub fn new(n: IndexType) -> LocalIndexableVector<T> {
        LocalIndexableVector {
            data: vec![T::zero(); n],
            index_set: LocalIndexSet::new((0, n)),
        }
    }
}

impl<T: Scalar> IndexableVector for LocalIndexableVector<T> {
    type Ind = LocalIndexSet;
    type Iter<'a> = std::slice::Iter<'a, T>;

    type IterMut<'a> = std::slice::IterMut<'a, T>;
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

    fn index_set(&self) -> &Self::Ind {
        &self.index_set
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.data.as_slice().iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.data.as_mut_slice().iter_mut()
    }

    fn len(&self) -> IndexType {
        self.index_set.number_of_global_indices()
    }
}

impl<T: Scalar> Inner for LocalIndexableVector<T> {
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

impl<T: Scalar> SquareSum for LocalIndexableVector<T> {
    type T = T;
    fn square_sum(&self) -> <Self::T as Scalar>::Real {
        self.iter()
            .fold(<<Self::T as Scalar>::Real>::zero(), |acc, &elem| {
                acc + elem.square()
            })
    }
}

impl<T: Scalar> Norm1 for LocalIndexableVector<T> {
    type T = T;
    fn norm_1(&self) -> <Self::T as Scalar>::Real {
        self.iter()
            .fold(<<Self::T as Scalar>::Real>::zero(), |acc, &elem| {
                acc + elem.abs()
            })
    }
}

impl<T: Scalar> Norm2 for LocalIndexableVector<T> {
    type T = T;
    fn norm_2(&self) -> <Self::T as Scalar>::Real {
        <<Self::T as Scalar>::Real as Float>::sqrt(self.square_sum())
    }
}

impl<T: Scalar> NormInf for LocalIndexableVector<T> {
    type T = T;
    fn norm_inf(&self) -> <Self::T as Scalar>::Real {
        self.iter().fold(
            <<Self::T as Scalar>::Real as Float>::neg_infinity(),
            |acc, &elem| <<Self::T as Scalar>::Real as Float>::max(acc, elem.abs()),
        )
    }
}
