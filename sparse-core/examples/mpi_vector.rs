//! Example file for creating vectors.

use sparse_core::distributed::index_layout::DistributedIndexLayout;
use sparse_traits::linalg::*;
use sparse_core::distributed::indexable_space::DistributedIndexableVectorSpace;
use sparse_traits::linalg::{Inner, NormInf, AbsSquareSum};
use sparse_traits::Element;
use sparse_traits::LinearSpace;
use sparse_traits::IndexLayout;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let index_layout = DistributedIndexLayout::new((0, 100), &world);

    let space = DistributedIndexableVectorSpace::<'_, f64, _>::new(&index_layout);

    let mut vec = space.create_element();

    let vec_impl = vec.view_mut();

    if let Some(val) = vec_impl.view_mut().unwrap().get_mut(0) {
        *val = 0.5;
    }

    println!("Range: {:#?}", index_layout.local_range());

    // println!("Inner: {}", vec.view().inner(&vec.view()).unwrap());
    // println!("Square sum: {}", vec.view().abs_square_sum());
    // println!("Inf norm: {}", vec.view().norm_inf());
}
