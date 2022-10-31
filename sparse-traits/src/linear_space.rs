//! Stubs for linear spaces.
use crate::Scalar;

/// Definition of a linear space
pub trait LinearSpace {
    type Item: Scalar;
    fn create_vector(&self) -> Box<&dyn Vector<Item=Self::Item>>;
}

/// A vector is an element of a linear space.
pub trait Vector {
    type Item: Scalar;
}
