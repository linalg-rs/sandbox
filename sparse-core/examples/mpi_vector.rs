//! Example file for creating vectors.

use sparse_core::distributed::index_layout::DistributedIndexLayout;
use sparse_core::distributed::indexable_space::DistributedIndexableVectorSpace;
use sparse_traits::indexable_vector::{IndexableVector, Inner, NormInf, SquareSum};
use sparse_traits::Element;
use sparse_traits::LinearSpace;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let index_layout = DistributedIndexLayout::new((0, 100), &world);

    let space = DistributedIndexableVectorSpace::<'_, f64, _>::new(&index_layout);

    let mut vec = space.create_element();

    if let Some(val) = vec.view_mut().get_mut(0) {
        *val = 0.5;
    }

    println!("Inner: {}", vec.view().inner(&vec.view()).unwrap());
    println!("Square sum: {}", vec.view().square_sum());
    println!("Inf norm: {}", vec.view().norm_inf());
}
