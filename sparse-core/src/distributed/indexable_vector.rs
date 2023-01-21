//! An Indexable Vector is a container whose elements can be 1d indexed.
use mpi::traits::*;
use num::{Float, Zero};
use sparse_traits::types::{Error, Result};
use sparse_traits::IndexableVector;
use sparse_traits::Scalar;
use sparse_traits::{IndexLayout, IndexType};
use sparse_traits::{Inner, Norm1, Norm2, NormInf, SquareSum};

use super::index_layout::DistributedIndexLayout;

pub struct DistributedIndexableVector<'a, T: Scalar + Equivalence, C: Communicator> {
    data: Vec<T>,
    index_layout: &'a DistributedIndexLayout<'a, C>,
}

impl<'a, T: Scalar + Equivalence, C: Communicator> DistributedIndexableVector<'a, T, C> {
    pub fn new(index_layout: &'a DistributedIndexLayout<'a, C>) -> Self {
        DistributedIndexableVector {
            data: vec![T::zero(); index_layout.number_of_local_indices()],
            index_layout,
        }
    }
}

impl<'a, T: Scalar + Equivalence, C: Communicator> IndexableVector
    for DistributedIndexableVector<'a, T, C>
{
    type T = T;
    type Ind = DistributedIndexLayout<'a, C>;
    type Iter<'b> = std::slice::Iter<'b, T> where Self: 'b;

    type IterMut<'b> = std::slice::IterMut<'b, T> where Self: 'b;
    fn get(&self, index: IndexType) -> Option<&Self::T> {
        self.data.get(index)
    }
    fn len(&self) -> IndexType {
        self.data.len()
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.data.iter()
    }

    fn get_mut(&mut self, index: IndexType) -> Option<&mut Self::T> {
        self.data.get_mut(index)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.data.iter_mut()
    }

    fn index_layout(&self) -> &Self::Ind {
        self.index_layout
    }

    unsafe fn get_unchecked(&self, index: IndexType) -> &Self::T {
        self.data.get_unchecked(index)
    }

    unsafe fn get_unchecked_mut(&mut self, index: IndexType) -> &mut Self::T {
        self.data.get_unchecked_mut(index)
    }
}

impl<T: Scalar + Equivalence, C: Communicator> Inner for DistributedIndexableVector<'_, T, C> {
    type T = T;
    fn inner(&self, other: &Self) -> Result<Self::T> {
        if self.len() != other.len() {
            return Err(Error::OperationFailed);
        }

        let comm = self.index_layout.comm();

        let local_result = self
            .iter()
            .zip(other.iter())
            .fold(<Self::T as Zero>::zero(), |acc, (&first, &second)| {
                acc + first * second.conj()
            });

        let mut global_result = T::zero();
        comm.all_reduce_into(
            &local_result,
            &mut global_result,
            mpi::collective::SystemOperation::sum(),
        );
        Ok(global_result)
    }
}

impl<T: Scalar + Equivalence, C: Communicator> SquareSum for DistributedIndexableVector<'_, T, C>
where
    T::Real: Equivalence,
{
    type T = T;
    fn square_sum(&self) -> <Self::T as Scalar>::Real {
        let comm = self.index_layout.comm();

        let local_result =
            self.iter()
                .fold(<<Self::T as Scalar>::Real as Zero>::zero(), |acc, &elem| {
                    acc + elem.square()
                });

        let mut global_result = <<Self::T as Scalar>::Real as Zero>::zero();
        comm.all_reduce_into(
            &local_result,
            &mut global_result,
            mpi::collective::SystemOperation::sum(),
        );
        global_result
    }
}


impl<T: Scalar + Equivalence, C: Communicator> Norm1 for DistributedIndexableVector<'_, T, C>
where
    T::Real: Equivalence,
{
    type T = T;
    fn norm_1(&self) -> <Self::T as Scalar>::Real {
        let comm = self.index_layout.comm();

        let local_result =
            self.iter()
                .fold(<<Self::T as Scalar>::Real as Zero>::zero(), |acc, &elem| {
                    acc + elem.abs()
                });

        let mut global_result = <<Self::T as Scalar>::Real as Zero>::zero();
        comm.all_reduce_into(
            &local_result,
            &mut global_result,
            mpi::collective::SystemOperation::sum(),
        );
        global_result
    }
}


impl<T: Scalar + Equivalence, C: Communicator> Norm2 for DistributedIndexableVector<'_, T, C>
where
    T::Real: Equivalence,
{
    type T = T;
    fn norm_2(&self) -> <Self::T as Scalar>::Real {
        Float::sqrt(self.square_sum())
    }
}

impl<T: Scalar + Equivalence, C: Communicator> NormInf for DistributedIndexableVector<'_, T, C>
where
    T::Real: Equivalence,
{
    type T = T;
    fn norm_inf(&self) -> <Self::T as Scalar>::Real {
        let comm = self.index_layout.comm();

        let local_result =
        self.iter().fold(
            <<Self::T as Scalar>::Real as Float>::neg_infinity(),
            |acc, &elem| <<Self::T as Scalar>::Real as Float>::max(acc, elem.abs()),
        );

        let mut global_result = <<Self::T as Scalar>::Real as Zero>::zero();
        comm.all_reduce_into(
            &local_result,
            &mut global_result,
            mpi::collective::SystemOperation::max(),
        );
        global_result
    }
}


/* impl<T: Scalar> IndexableVector for LocalIndexableVector<'_, T> {
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

    fn index_set(&self) -> &Self::Ind {
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


impl<T: Scalar> SquareSum for LocalIndexableVector<'_, T> {
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
} */
