//! Definition of Index Sets

use crate::{IndexType, SparseLinAlgResult};

pub trait IndexLayout {
    /// The local index range.
    fn local_range(&self) -> (IndexType, IndexType);

    /// Global number of indices.
    fn number_of_global_indices(&self) -> IndexType;

    fn number_of_local_indices(&self) -> IndexType;

    /// Index range on a given process.
    fn index_range(&self, rank: IndexType) -> SparseLinAlgResult<(IndexType, IndexType)>;

    /// Convert continuous (0, n) indices to actual indices.
    ///
    /// Assume that the local range is (30, 40). Then this method
    /// will map (0,10) -> (30, 40).
    /// It returns ```None``` if ```index``` is out of bounds.
    fn local2global(&self, index: IndexType) -> Option<IndexType>;
}
