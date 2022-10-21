//! Stubs for linear spaces.

use std::marker::PhantomData;

use crate::types::GeneralScalar;

pub struct CollectionOfVectors<ItemType: GeneralScalar> {
    _marker: std::marker::PhantomData<ItemType>,
}

impl<ItemType: GeneralScalar> CollectionOfVectors<ItemType> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
