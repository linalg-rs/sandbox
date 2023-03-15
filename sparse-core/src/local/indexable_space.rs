//! An indexable vector space has elements that can be indexed as n-dimensional vectors.

use std::marker::PhantomData;

use super::index_layout::LocalIndexLayout;
use super::indexable_vector::LocalIndexableVector;
use sparse_traits::linalg::{Inner, Norm2};
use sparse_traits::types::{IndexType, Scalar};
use sparse_traits::{Element, IndexLayout, IndexableSpace, InnerProductSpace, NormedSpace};

pub struct LocalIndexableVectorSpace<T: Scalar> {
    index_layout: LocalIndexLayout,
    _phantom: PhantomData<T>,
}

impl<T: Scalar> LocalIndexableVectorSpace<T> {
    pub fn new(n: IndexType) -> Self {
        LocalIndexableVectorSpace {
            index_layout: LocalIndexLayout::new(n),
            _phantom: PhantomData,
        }
    }
}

pub struct LocalIndexableVectorSpaceElement<'a, T: Scalar> {
    space: &'a LocalIndexableVectorSpace<T>,
    data: super::indexable_vector::LocalIndexableVector<T>,
}

impl<'a, T: Scalar> Element for LocalIndexableVectorSpaceElement<'a, T> {
    type Space = LocalIndexableVectorSpace<T>;
    type View<'b> = &'b super::indexable_vector::LocalIndexableVector<T> where Self: 'b;
    type ViewMut<'b> = &'b mut super::indexable_vector::LocalIndexableVector<T> where Self: 'b;

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

impl<T: Scalar> sparse_traits::LinearSpace for LocalIndexableVectorSpace<T> {
    type F = T;
    type E<'a> = LocalIndexableVectorSpaceElement<'a, T> where Self: 'a;

    fn create_element<'a>(&'a self) -> Self::E<'a> {
        LocalIndexableVectorSpaceElement {
            space: &self,
            data: LocalIndexableVector::new(self.index_layout().number_of_global_indices()),
        }
    }
}

impl<T: Scalar> IndexableSpace for LocalIndexableVectorSpace<T> {
    type Ind = LocalIndexLayout;
    fn dimension(&self) -> IndexType {
        self.index_layout().number_of_global_indices()
    }

    fn index_layout(&self) -> &Self::Ind {
        &self.index_layout
    }
}

impl<T: Scalar> InnerProductSpace for LocalIndexableVectorSpace<T> {
    fn inner<'a>(
        &self,
        x: &sparse_traits::ElementView<'a, Self>,
        other: &sparse_traits::ElementView<'a, Self>,
    ) -> sparse_traits::SparseLinAlgResult<Self::F>
    where
        Self: 'a,
    {
        x.inner(other)
    }
}

impl<T: Scalar> NormedSpace for LocalIndexableVectorSpace<T> {
    fn norm<'a>(&'a self, x: &sparse_traits::ElementView<'a, Self>) -> <Self::F as Scalar>::Real {
        x.norm_2()
    }
}
