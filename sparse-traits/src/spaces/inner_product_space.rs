use super::ElementView;
use super::LinearSpace;
use crate::types::Result;

pub trait InnerProductSpace: LinearSpace {
    fn inner(&self, x: ElementView<Self>, other: ElementView<Self>) -> Result<Self::F>;
}
