//! Linear spaces and their elements.

use super::{Element, ElementView};
use crate::types::{Error, Result};
use crate::Scalar;

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
    fn norm(_x: ElementView<Self>, _res: &mut <Self::F as Scalar>::Real) -> Result<()> {
        Err(Error::NotImplemented)
    }
}
