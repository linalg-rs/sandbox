//! Stubs for linear spaces.
use crate::{views::VectorTypedView, GeneralScalar};

/// Definition of a linear space
pub trait LinearSpace {

    fn create_vector(&self) -> Box<&dyn Vector>;

}

/// A vector is an element of a linear space.
pub trait Vector {

}