//! Linear spaces and their elements.

use crate::IndexSet;
use crate::{Error, Result};
use crate::{IndexType, Scalar};

/// Definition of a linear space
///
/// Linear spaces are basic objects that can create
/// elements of the space.
pub trait LinearSpace {
    /// Field Type.
    type F: Scalar;

    /// Type associated with elements of the space.
    type E: Element<Space = Self>;

    /// Create a new vector from the space.
    fn create_element(&self) -> Self::E {
        std::unimplemented!();
    }

    /// Norm of a vector.
    fn norm(_x: &Self::E, _res: &mut <Self::F as cauchy::Scalar>::Real) -> Result<()> {
        Err(Error::NotImplemented)
    }
}

pub trait DualSpace: LinearSpace {
    type Space: LinearSpace<F = Self::F>;

    fn dual_pairing(&self, x: &Self::E, other: &<Self::Space as LinearSpace>::E)
        -> Result<Self::F>;
}

pub trait InnerProductSpace: LinearSpace {
    fn inner(&self, x: &Self::E, other: &Self::E) -> Result<Self::F>;
}

pub trait IndexableVectorSpace: InnerProductSpace {
    fn dimension(&self) -> IndexType {
        self.index_set().number_of_global_indices()
    }

    fn index_set(&self) -> &dyn IndexSet;
}

/// Elements of linear spaces.
pub trait Element {
    /// Item type of the vector.
    type Space: LinearSpace;
<<<<<<< HEAD
    type View;
=======
    type View<'a>
    where
        Self: 'a;
    type ViewMut<'a>
    where
        Self: 'a;
>>>>>>> main

    /// Return the underlying space.
    fn space(&self) -> &Self::Space {
        std::unimplemented!();
    }

<<<<<<< HEAD
    fn view(&self) -> &Self::View;

    fn view_mut(&mut self) -> &mut Self::View;
=======
    fn view<'a>(&'a self) -> Self::View<'a>;

    fn view_mut<'a>(&'a mut self) -> Self::ViewMut<'a>;
>>>>>>> main
}

/// A finite dimensional indexable type.
pub trait IndexableVector: Element {
    type View: IndexableVectorView;
    fn dimension(&self) -> IndexType;
}

/// A vector view allows access and iteration for vector data.
pub trait IndexableVectorView {}

// The view type associated with elements of linear spaces.
<<<<<<< HEAD
pub type ElementView<Space> = <<Space as LinearSpace>::E as Element>::View;
=======
pub type ElementView<'a, Space> = <<Space as LinearSpace>::E as Element>::View<'a>;
pub type ElementViewMut<'a, Space> = <<Space as LinearSpace>::E as Element>::ViewMut<'a>;
>>>>>>> main
