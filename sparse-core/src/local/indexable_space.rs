//! An indexable vector space has elements that can be indexed as n-dimensional vectors.

use std::marker::PhantomData;

use super::index_set::LocalIndexSet;
use super::indexable_vector::LocalIndexableVector;
use sparse_traits::types::{IndexType, Scalar};
use sparse_traits::Inner;
use sparse_traits::{Element, IndexSet, IndexableVectorSpace, InnerProductSpace};

pub struct LocalIndexableVectorSpace<T: Scalar> {
    index_set: LocalIndexSet,
    _phantom: PhantomData<T>,
}

impl<T: Scalar> LocalIndexableVectorSpace<T> {
    pub fn new(n: IndexType) -> Self {
        LocalIndexableVectorSpace {
            index_set: LocalIndexSet::new((0, n)),
            _phantom: PhantomData,
        }
    }
}

pub struct LocalIndexableVectorSpaceElement<'a, T: Scalar> {
    index_set: &'a LocalIndexSet,
    space: &'a LocalIndexableVectorSpace<T>,
    data: super::indexable_vector::LocalIndexableVector<T>,
}

impl<'a, T: Scalar> Element<'a> for LocalIndexableVectorSpaceElement<'a, T> {
    type Space = LocalIndexableVectorSpace<T>;
    type View<'b> = &'b super::indexable_vector::LocalIndexableVector<T> where Self: 'b;
    type ViewMut<'b> = &'b mut super::indexable_vector::LocalIndexableVector<T> where Self: 'b;

    fn space(&self) -> &'a Self::Space {
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
    type E<'a> = LocalIndexableVectorSpaceElement<'a, T>;

    fn create_element<'a>(&'a self) -> Self::E<'a> {
        LocalIndexableVectorSpaceElement {
            index_set: &self.index_set,
            space: &self,
            data: LocalIndexableVector::new(self.index_set.number_of_global_indices()),
        }
    }

    fn norm<'a>(
        _x: sparse_traits::ElementView<'a, 'a, Self>,
        _res: &mut <Self::F as Scalar>::Real,
    ) -> sparse_traits::Result<()> {
        Err(sparse_traits::Error::NotImplemented)
    }
}

impl<T: Scalar> IndexableVectorSpace for LocalIndexableVectorSpace<T> {
    fn dimension(&self) -> IndexType {
        self.index_set().number_of_global_indices()
    }

    fn index_set(&self) -> &dyn IndexSet {
        &self.index_set
    }
}

impl<T: Scalar> InnerProductSpace for LocalIndexableVectorSpace<T> {
    fn inner<'a>(
        &self,
        x: sparse_traits::ElementView<'a, 'a, Self>,
        other: sparse_traits::ElementView<'a, 'a, Self>,
    ) -> sparse_traits::Result<Self::F> {
        x.inner(other)
    }
}
