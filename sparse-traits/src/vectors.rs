//! Stubs for linear spaces.

use crate::IndexSet;
use crate::{Error, Result};
use crate::{IndexType, Scalar};

/// Definition of a vector space
///
/// Linear Spaces are factory objects that can allocate memory
/// to create new vectors.
pub trait Space {
    /// Field Type.
    type F: Scalar;

    /// Type associated with elements of the space.
    type E: Element<Space = Self>;

    /// Create a new vector from the space.
    fn create_element(&self) -> Self::E {
        std::unimplemented!();
    }

    /// Norm of a vector.
    fn norm(x: &Self::E, res: &mut <Self::F as cauchy::Scalar>::Real) -> Result {
        Err(Error::NotImplemented)

        // Need to fill out more algebra operations...
    }
}

pub trait DualSpace: Space {
    type Space: Space<F = Self::F>;

    fn dual_pairing(
        &self,
        x: &Self::E,
        other: <Self::Space as Space>::E,
        res: &mut Self::F,
    ) -> Result;
}

pub trait InnerProductSpace: Space {
    fn inner(&self, x: &Self::E, other: &Self::E, res: &mut Self::F) -> Result;
}

pub trait FiniteVectorSpace: InnerProductSpace {
    fn dimension(&self) -> IndexType {
        self.index_set().number_of_global_indices()
    }

    fn index_set(&self) -> &dyn IndexSet;
}

/// A vector is an element of a linear space.
pub trait Element {
    /// Item type of the vector.
    type Space: Space;
    type View;

    /// Return the underlying space.
    fn space(&self) -> &Self::Space {
        std::unimplemented!();
    }

    fn view(&self) -> &Self::View {
        std::unimplemented!();
    }

    fn view_mut(&mut self) -> &mut Self::View {
        std::unimplemented!();
    }
}

pub trait FiniteVector: Element {
    type View: FiniteVectorView;
    fn dimension(&self) -> IndexType;
}

pub trait FiniteVectorView {}
