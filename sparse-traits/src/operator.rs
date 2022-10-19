//! General linear operator.

use crate::{linear_space::CollectionOfVectors, types::GeneralScalar};

// A base operator trait.
pub trait OperatorBase {}

/// Matrix vector product $Ax$.
pub trait AsMatVec<InputType: GeneralScalar, OutputType: GeneralScalar>: OperatorBase {
    fn matvec(&self, other: &CollectionOfVectors<InputType>) -> CollectionOfVectors<OutputType>;
}

/// Matrix vector product $A^Hx$.
pub trait AsHermitianMatVec<InputType: GeneralScalar, OutputType: GeneralScalar>: OperatorBase {
    fn matvec_h(&self, other: &CollectionOfVectors<InputType>) -> CollectionOfVectors<OutputType>;
}

/// Matrix vector product $A^Tx$.
pub trait AsTransposeMatVec<InputType: GeneralScalar, OutputType: GeneralScalar>: OperatorBase {
    fn matvec_t(&self, other: &CollectionOfVectors<InputType>) -> CollectionOfVectors<OutputType>;
}

/// A base operator struct.
pub struct Operator {
    implementer: Box<dyn OperatorBase>,
}