//! An Indexable Vector is a container whose elements can be 1d indexed.
use crate::local::indexable_vector::{
    LocalIndexableVector, LocalIndexableVectorView, LocalIndexableVectorViewMut,
};
use mpi::datatype::Partition;
use mpi::traits::*;
use num::{Float, Zero};
use sparse_traits::linalg::*;
use sparse_traits::linalg::{Inner, Norm1, Norm2, NormInfty};
use sparse_traits::types::{Error, Result};
use sparse_traits::{linalg::IndexableVectorView, IndexLayout, Scalar};

use super::index_layout::DistributedIndexLayout;

pub struct DistributedIndexableVector<'a, T: Scalar + Equivalence, C: Communicator> {
    index_layout: &'a DistributedIndexLayout<'a, C>,
    local: Option<LocalIndexableVector<'a, T>>,
}

impl<'a, T: Scalar + Equivalence, C: Communicator> DistributedIndexableVector<'a, T, C> {
    pub fn new(index_layout: &'a DistributedIndexLayout<'a, C>) -> Self {
        DistributedIndexableVector {
            index_layout,
            local: match index_layout.local_layout() {
                Some(layout) => Some(LocalIndexableVector::new(layout)),
                None => None,
            },
        }
    }

    fn local(&self) -> Option<&LocalIndexableVector<'a, T>> {
        self.local.as_ref()
    }

    pub fn fill_from_root(&mut self, other: &Option<LocalIndexableVector<T>>) -> Result<()> {
        let comm = self.index_layout().comm().duplicate();
        let counts = self.index_layout().counts().as_slice();
        let displacements = self.index_layout().displacements().as_slice();
        let global_dim = self.index_layout().number_of_global_indices();
        let mut recvbuf = vec![T::zero(); self.index_layout().number_of_local_indices()];

        let root_process = comm.process_at_rank(0);
        if comm.rank() == 0 {
            assert!(other.is_some(), "`other` has a `none` value.");

            let local_vector = other.as_ref().unwrap();

            let local_dim = local_vector.index_layout().number_of_global_indices();

            assert_eq!(
                local_dim, global_dim,
                "Dimension of local vector {} does not match dimension of distributed vector {}",
                local_dim, global_dim
            );

            let view = local_vector.view().unwrap();
            let data = view.data().as_ref();
            let partition = Partition::new(data, counts, displacements);

            root_process.scatter_varcount_into_root(&partition, &mut recvbuf);

        } else {
            assert!(other.is_none(), "`other` has a `Some` value.");
            root_process.scatter_varcount_into(&mut recvbuf);

        }

        if let Some(mut view) = self.view_mut() {
            view.data_mut().clone_from_slice(&recvbuf);
        }

        Ok(())
    }
}

impl<'a, T: Scalar + Equivalence, C: Communicator> IndexableVector
    for DistributedIndexableVector<'a, T, C>
{
    type T = T;
    type View<'b> = LocalIndexableVectorView<'b, T> where Self: 'b;
    type ViewMut<'b> = LocalIndexableVectorViewMut<'b, T> where Self: 'b;
    type Ind = DistributedIndexLayout<'a, C>;

    fn index_layout(&self) -> &Self::Ind {
        &self.index_layout
    }

    fn view<'b>(&'b self) -> Option<Self::View<'b>> {
        match &self.local {
            Some(local_vec) => Some(local_vec.view().unwrap()),
            None => None,
        }
    }

    fn view_mut<'b>(&'b mut self) -> Option<Self::ViewMut<'b>> {
        match &mut self.local {
            Some(local_vec) => Some(local_vec.view_mut().unwrap()),
            None => None,
        }
    }
}

impl<T: Scalar + Equivalence, C: Communicator> Inner for DistributedIndexableVector<'_, T, C> {
    type T = T;
    fn inner(&self, other: &Self) -> Result<Self::T> {
        if !self.index_layout().is_same(other.index_layout()) {
            return Err(Error::OperationFailed);
        }

        let mut local_result = T::zero();

        if let Some(local) = self.local() {
            local_result = local.inner(&other.local().unwrap()).unwrap();
        }

        let comm = self.index_layout.comm();

        let mut global_result = T::zero();
        comm.all_reduce_into(
            &local_result,
            &mut global_result,
            mpi::collective::SystemOperation::sum(),
        );
        Ok(global_result)
    }
}

impl<T: Scalar + Equivalence, C: Communicator> AbsSquareSum for DistributedIndexableVector<'_, T, C>
where
    T::Real: Equivalence,
{
    type T = T;
    fn abs_square_sum(&self) -> <Self::T as Scalar>::Real {
        let comm = self.index_layout.comm();

        let mut local_result = <<Self::T as Scalar>::Real>::zero();

        if let Some(local) = self.local() {
            local_result = local.abs_square_sum();
        }

        let mut global_result = <<Self::T as Scalar>::Real>::zero();
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

        let mut local_result = <<Self::T as Scalar>::Real>::zero();

        if let Some(local) = self.local() {
            local_result = local.norm_1();
        }

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
        Float::sqrt(self.abs_square_sum())
    }
}

impl<T: Scalar + Equivalence, C: Communicator> NormInfty for DistributedIndexableVector<'_, T, C>
where
    T::Real: Equivalence,
{
    type T = T;
    fn norm_infty(&self) -> <Self::T as Scalar>::Real {
        let comm = self.index_layout.comm();

        let mut local_result = <<Self::T as Scalar>::Real>::zero();

        if let Some(local) = self.local() {
            local_result = local.norm_infty();
        }

        let mut global_result = <<Self::T as Scalar>::Real as Zero>::zero();
        comm.all_reduce_into(
            &local_result,
            &mut global_result,
            mpi::collective::SystemOperation::max(),
        );
        global_result
    }
}
