//! Definition of Index Sets

use crate::IndexType;

pub trait IndexSet {
    /// The local index range.
    fn local_range(&self) -> (IndexType, IndexType);

    /// Global number of indices.
    fn number_of_indices(&self) -> IndexType;

    /// Index range on a given process.
    fn index_range(&self, proc: IndexType) -> Option<IndexType>;
}
