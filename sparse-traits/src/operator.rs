//! General linear operator.

use std::marker::PhantomData;

use crate::{linear_space::CollectionOfVectors, types::GeneralScalar};

// A base operator trait.
pub trait OperatorBase<InputType: GeneralScalar, OutputType: GeneralScalar> {
    fn as_matvec(&self) -> Result<&dyn AsMatVec<InputType, OutputType>, ()> {
        Err(())
    }
}

/// Matrix vector product $Ax$.
pub trait AsMatVec<InputType: GeneralScalar, OutputType: GeneralScalar>:
    OperatorBase<InputType, OutputType>
{
    fn matvec(&self, other: &CollectionOfVectors<InputType>) -> CollectionOfVectors<OutputType>;
}

/// Matrix vector product $A^Hx$.
pub trait AsHermitianMatVec<InputType: GeneralScalar, OutputType: GeneralScalar>:
    OperatorBase<InputType, OutputType>
{
    fn matvec_h(&self, other: &CollectionOfVectors<InputType>) -> CollectionOfVectors<OutputType>;
}

/// Matrix vector product $A^Tx$.
pub trait AsTransposeMatVec<InputType: GeneralScalar, OutputType: GeneralScalar>:
    OperatorBase<InputType, OutputType>
{
    fn matvec_t(&self, other: &CollectionOfVectors<InputType>) -> CollectionOfVectors<OutputType>;
}

