use super::LinearSpace;
use crate::types::IndexType;
use crate::IndexSet;

pub trait IndexableVectorSpace: LinearSpace {
    fn dimension(&self) -> IndexType {
        self.index_set().number_of_global_indices()
    }

    fn index_set(&self) -> &dyn IndexSet;
}
