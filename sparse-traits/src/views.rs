//! Definition of view traits.
pub use crate::types::*;

pub trait VectorTypedView<'a> {
    type Item: GeneralScalar;

    // We would like to define the iterator just by a trait.
    // But this requires generic associated types (GATs), which luckily
    // become stable in Rust 1.65 scheduled for 3 November. We
    // can then use
    //
    // type I<T: GeneralScalar>: std::iter::Iterator<Item = T>;
    //

    // Put the usual accessor methods here.

    /// Reference to a single element.
    fn get(&self, index: IndexType) -> Option<&'a Self::Item>;

    /// Mutable reference to a single element.
    fn get_mut(&mut self, index: IndexType) -> Option<&'a mut Self::Item>;

    // With GATs we can then write

    // /// Iterator
    // fn iter(&self) -> I<&'a Self::Item>;

    // /// Mutable iterator
    // fn iter_mut(&mut self) -> I<'a mut Self::Item>;
}
