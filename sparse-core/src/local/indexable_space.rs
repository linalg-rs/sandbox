//! An indexable vector space has elements that can be indexed as n-dimensional vectors.

use std::marker::PhantomData;

use super::index_set::LocalIndexSet;
use sparse_traits::spaces::IndexableVectorSpace;
use sparse_traits::types::{IndexType, Scalar};
use sparse_traits::Element;

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

impl<'a, T: Scalar> sparse_traits::LinearSpace for LocalIndexableVectorSpace<T> {
    type F = T;
    type E = LocalIndexableVectorSpaceElement<'a, T>;

    fn create_element(&self) -> Self::E {
        std::unimplemented!();
    }

    fn norm(
        _x: sparse_traits::ElementView<Self>,
        _res: &mut <Self::F as Scalar>::Real,
    ) -> sparse_traits::Result<()> {
        Err(sparse_traits::Error::NotImplemented)
    }
}
