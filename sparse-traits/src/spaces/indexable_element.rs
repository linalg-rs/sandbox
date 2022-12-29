//! An indexable element has an associated index set.
use crate::index_set::IndexSet;

pub trait IndexableElement<'a>: super::element::Element<'a> {
    fn index_set(&self) -> &dyn IndexSet;
}
