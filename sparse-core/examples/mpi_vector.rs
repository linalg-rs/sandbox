
//! Example file for creating vectors.

use mpi::topology::SystemCommunicator;
use sparse_core::distributed::index_layout::DistributedIndexLayout;
use sparse_traits::indexable_vector::{IndexableVector, Inner, SquareSum, NormInf};
use sparse_core::distributed::DistributedIndexableVector;

fn main() {

    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let index_layout = DistributedIndexLayout::new((0, 4), &world);

    let mut vec = DistributedIndexableVector::<'_, f64, SystemCommunicator>::new(&index_layout);
    if let Some(val) = vec.get_mut(0) {
        *val = 0.5;
    }


    println!("Inner: {}", vec.inner(&vec).unwrap());
    println!("Square sum: {}", vec.square_sum());
    println!("Inf norm: {}", vec.norm_inf());
}
