//! General linear operator.

use std::fmt::Debug;

use crate::*;

// A base operator trait.
pub trait OperatorBase: Debug {
    /// Returns a reference to trait object that supports matvec.
    ///
    /// By default it returns an `Err`. But for concrete types
    /// that support matvecs it is specialised to return
    /// a dynamic reference.
    fn as_matvec(&self) -> Option<&dyn AsMatVec> {
        None
    }

    fn as_matvec_h(&self) -> Option<&dyn AsHermitianMatVec> {
        None
    }

    // The following convenience routine returns true if an operator
    // supports matvecs. Applied to trait objects it provides a runtime
    // check about supported traits.
    fn has_matvec(&self) -> bool {
        self.as_matvec().is_some()
    }
}

/// Matrix vector product $Ax$.
pub trait AsMatVec: OperatorBase {
    fn matvec(&self, x: &dyn Vector, y: &mut dyn Vector);
}

/// Matrix vector product $A^Hx$.
pub trait AsHermitianMatVec: OperatorBase {
    fn matvec_h(&self, x: &dyn Vector, y: &mut dyn Vector);
}

/// Matrix vector product $A^Tx$.
pub trait AsTransposeMatVec: OperatorBase {
    fn matvec_t(&self, x: &dyn Vector, y: &mut dyn Vector);
}
