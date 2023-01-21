use sparse_traits::{IndexLayout, IndexType};

pub struct LocalIndexLayout {
    range: Option<(IndexType, IndexType)>,
    number_of_global_indices: IndexType,
}

impl LocalIndexLayout {
    pub fn new(range: (IndexType, IndexType)) -> Self {
        Self {
            range: Some(range),
            number_of_global_indices: range.1 - range.0,
        }
    }
}

impl IndexLayout for LocalIndexLayout {
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

    fn local2global(&self, index: IndexType) -> Option<IndexType> {
        if index < self.number_of_local_indices() {
            Some(index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_local_index_set() {
        let index_set = LocalIndexLayout::new((3, 14));

        // Test that the range is correct on rank 0
        assert_eq!(index_set.index_range(0).unwrap(), (3, 14));

        // Test that the number of global indices is correct.
        assert_eq!(index_set.number_of_global_indices(), 11);

        // Test that the number of local indices is correct.

        assert!(index_set.index_range(1).is_none());
    }
}
