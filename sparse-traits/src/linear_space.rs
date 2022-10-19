//! Stubs for linear spaces.

use crate::types::GeneralScalar;

pub struct CollectionOfVectors<ItemType: GeneralScalar> {
    _marker: std::marker::PhantomData<ItemType>,
}