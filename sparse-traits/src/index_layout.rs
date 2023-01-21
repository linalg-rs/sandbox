//! Definition of Index Sets

use crate::IndexType;

pub trait IndexLayout {
    /// The local index range.
    fn local_range(&self) -> &Option<(IndexType, IndexType)>;

    /// Global number of indices.
    fn number_of_global_indices(&self) -> IndexType;

    fn number_of_local_indices(&self) -> IndexType;

    /// Index range on a given process.
    fn index_range(&self, rank: IndexType) -> &Option<(IndexType, IndexType)>;

    /// Convert local to global indices.
    fn local2global(&self, index: IndexType) -> Option<IndexType>;
}
