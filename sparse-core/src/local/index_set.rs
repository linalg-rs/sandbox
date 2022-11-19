use sparse_traits::{IndexSet, IndexType};

pub struct LocalIndexSet {
    range: Option<(IndexType, IndexType)>,
    number_of_global_indices: IndexType,
}

impl LocalIndexSet {
    pub fn new(range: (IndexType, IndexType)) -> Self {
        Self {
            range: Some(range),
            number_of_global_indices: range.1 - range.0,
        }
    }
}

impl IndexSet for LocalIndexSet {
    fn number_of_local_indices(&self) -> IndexType {
        self.number_of_global_indices()
    }

    fn local_range(&self) -> &Option<(IndexType, IndexType)> {
        &self.range
    }

    fn number_of_global_indices(&self) -> IndexType {
        self.number_of_global_indices
    }

    fn index_range(&self, rank: IndexType) -> &Option<(IndexType, IndexType)> {
        match rank {
            0 => &self.range,
            _ => &None,
        }
    }
}
