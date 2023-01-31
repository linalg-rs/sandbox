use super::ElementView;
use super::LinearSpace;
use crate::types::Result;

pub trait InnerProductSpace: LinearSpace {
    fn inner<'a>(&self, x: &ElementView<'a, Self>, other: &ElementView<'a, Self>) -> Result<Self::F> where Self: 'a;
}
