//! Stubs for linear spaces.
use crate::IndexSet;

use crate::{IndexType, RealType, Scalar};

/// Definition of a linear space
///
/// Linear Spaces are factory objects that can allocate memory
/// to create new vectors.
pub trait LinearSpace {
    /// Item type of the space.
    type Item: Scalar;

    /// Realtype for operations into real numbers.
    type Real: RealType;

    /// Type associated with vectors.
    type VectorType: Vector<Space = Self>;

    /// The associated index set.
    fn index_set(&self) -> &dyn IndexSet {
        std::unimplemented!();
    }

    /// Create a new vector from the space.
    fn create_vector(&self) -> Self::VectorType {
        std::unimplemented!();
    }

    /// The dimension of the linear space.
    fn dimension(&self) -> IndexType {
        std::unimplemented!();
    }

    /// Inner product of two vectors.
    fn inner(x: &Self::VectorType, y: &Self::VectorType) -> Self::Item {
        std::unimplemented!();
    }

    /// Norm of a vector.
    fn norm(x: &Self::VectorType) -> Self::Real {
        std::unimplemented!();
    }

    // Need to fill out more algebra operations...
}

/// A vector is an element of a linear space.
pub trait Vector {
    /// Item type of the vector.
    type Space: LinearSpace;
    type View: VectorView<Item = <Self::Space as LinearSpace>::Item>;

    /// The dimension of the vector.
    fn dimension(&self) -> IndexType {
        self.space().dimension()
    }

    /// Return the underlying space.
    fn space(&self) -> &Self::Space {
        std::unimplemented!();
    }

    /// The index set associated with the vector.
    fn index_set(&self) -> &dyn IndexSet {
        self.space().index_set()
    }

    fn view(&self) -> Self::View {
        std::unimplemented!();
    }
}

pub trait VectorView {
    type Item: Scalar;

    fn get(&self, index: IndexType) -> &Self::Item {
        std::unimplemented!();
    }
    fn get_mut(&self, index: IndexType) -> &mut Self::Item {
        std::unimplemented!();
    }
}
