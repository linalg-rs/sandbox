//! An indexable element has an associated index set.
use crate::index_set::IndexSet;

pub trait IndexableElement: super::element::Element {
    fn index_set(&self) -> &dyn IndexSet;
}
