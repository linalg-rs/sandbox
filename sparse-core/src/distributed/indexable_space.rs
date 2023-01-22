//! An indexable vector space has elements that can be indexed as n-dimensional vectors.

use mpi::traits::*;
use std::marker::PhantomData;

use super::index_layout::DistributedIndexLayout;
use super::indexable_vector::DistributedIndexableVector;
use sparse_traits::types::{IndexType, Scalar};
use sparse_traits::{Element, IndexLayout, IndexableVectorSpace, InnerProductSpace};
use sparse_traits::{Inner, Norm2};

pub struct DistributedIndexableVectorSpace<'comm, T: Scalar + Equivalence, C: Communicator> {
    index_layout: &'comm DistributedIndexLayout<'comm, C>,
    _phantom: PhantomData<T>,
}

impl<'comm, T: Scalar + Equivalence, C: Communicator> DistributedIndexableVectorSpace<'comm, T, C> {
    pub fn new(index_layout: &'comm DistributedIndexLayout<'comm, C>) -> Self {
        DistributedIndexableVectorSpace {
            index_layout,
            _phantom: PhantomData,
        }
    }
}

pub struct DistributedIndexableVectorSpaceElement<'space, 'comm, T: Scalar + Equivalence, C: Communicator> {
    space: &'space DistributedIndexableVectorSpace<'comm, T, C>,
    data: super::indexable_vector::DistributedIndexableVector<'comm, T, C>,
}

impl<'space, 'comm, T: Scalar + Equivalence, C: Communicator> Element
    for DistributedIndexableVectorSpaceElement<'space, 'comm, T, C> where T::Real: Equivalence
{
    type Space = DistributedIndexableVectorSpace<'comm, T, C>;
    type View<'b> = &'b super::indexable_vector::DistributedIndexableVector<'comm, T, C> where Self: 'b;
    type ViewMut<'b> = &'b mut super::indexable_vector::DistributedIndexableVector<'comm, T, C> where Self: 'b;

    fn space(&self) -> &Self::Space {
        self.space
    }

    fn view<'b>(&'b self) -> Self::View<'b> {
        &self.data
    }

    fn view_mut<'b>(&'b mut self) -> Self::ViewMut<'b> {
        &mut self.data
    }
}

impl<'comm, T: Scalar + Equivalence, C: Communicator> sparse_traits::LinearSpace
    for DistributedIndexableVectorSpace<'comm, T, C>
where
    T::Real: Equivalence,
{
    type F = T;
    type E<'space> = DistributedIndexableVectorSpaceElement<'space, 'comm, T, C> where Self: 'space;

    fn create_element<'space>(&'space self) -> Self::E<'space>
    {
        DistributedIndexableVectorSpaceElement {
            space: &self,
            data: DistributedIndexableVector::<'comm, T, C>::new(&self.index_layout),
        }
    }

    fn norm<'c>(
        &'c self,
        x: sparse_traits::ElementView<'c, 'c, Self>,
        res: &mut <Self::F as Scalar>::Real,
    ) -> sparse_traits::Result<()>
    {
        *res = x.norm_2();
        Ok(())
    }
}

impl<'a, T: Scalar + Equivalence, C: Communicator> IndexableVectorSpace
    for DistributedIndexableVectorSpace<'a, T, C>
where
    T::Real: Equivalence,
{
    type Ind = DistributedIndexLayout<'a, C>;
    fn dimension(&self) -> IndexType {
        self.index_layout().number_of_global_indices()
    }

    fn index_layout(&self) -> &Self::Ind {
        &self.index_layout
    }
}

impl<'a, T: Scalar + Equivalence, C: Communicator> InnerProductSpace
    for DistributedIndexableVectorSpace<'a, T, C>
where
    T::Real: Equivalence,
{
    fn inner<'b>(
        &self,
        x: sparse_traits::ElementView<'b, 'b, Self>,
        other: sparse_traits::ElementView<'b, 'b, Self>,
    ) -> sparse_traits::Result<Self::F>
    where
        Self: 'b,
    {
        x.inner(other)
    }
}
