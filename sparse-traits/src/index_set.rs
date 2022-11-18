//! Definition of Index Sets

use crate::IndexType;

pub trait IndexSet {
    /// The local index range.
    fn local_range(&self) -> (IndexType, IndexType);

    /// Global number of indices.
    fn number_of_global_indices(&self) -> IndexType;

    fn number_of_local_indices(&self) -> IndexType {
        self.local_range().1 - self.local_range().0
    }

    /// Index range on a given process.
    fn index_range(&self, rank: IndexType) -> Option<(IndexType, IndexType)>;
}
