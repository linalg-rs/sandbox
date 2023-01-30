//! Example file for creating vectors.

use mpi::traits::*;
use sparse_core::distributed::index_layout::DistributedIndexLayout;
use sparse_core::distributed::indexable_space::DistributedIndexableVectorSpace;
use sparse_core::local::index_layout::LocalIndexLayout;
use sparse_core::local::indexable_vector::LocalIndexableVector;
use sparse_traits::IndexLayout;
use sparse_traits::linalg::*;
use sparse_traits::Element;
use sparse_traits::LinearSpace;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    let n = 100;

    let index_layout = DistributedIndexLayout::new((0, n), &world);

    let space = DistributedIndexableVectorSpace::<'_, f64, _>::new(&index_layout);

    let mut vec = space.create_element();

    let vec_impl = vec.view_mut();

    // if let Some(val) = vec_impl.view_mut().unwrap().get_mut(0) {
    //     *val = 0.5;
    // }

    let mut local_vec: LocalIndexableVector<'_, f64>;

    if rank == 0 {
        let local_layout = LocalIndexLayout::new((0, n));

        local_vec = LocalIndexableVector::<'_, f64>::new(&local_layout);
        let mut view = local_vec.view_mut().unwrap();

        for index in 0..n {
            *view.get_mut(index).unwrap() = index as f64;
        }

        println!("Local inf norm: {}", local_vec.norm_inf());

        let _ = vec_impl.fill_from_root(&Some(local_vec));

        println!("Root: {}", vec_impl.view().unwrap().data().get(49).unwrap());
        println!("Global dofs {}", vec_impl.index_layout().number_of_global_indices());
        println!("Dofs at root {:#?}", vec_impl.index_layout().index_range(1));
    } else {
        let _ = vec_impl.fill_from_root(&None);
        println!("Proc 1: {}", vec_impl.view().unwrap().data().get(49).unwrap());
        println!("Dofs at root {:#?}", vec_impl.index_layout().index_range(1));
    }
    println!("Inner: {}", vec.view().inner(&vec.view()).unwrap());
    println!("Inf norm: {}", vec.view().norm_inf());

    if world.rank() == 0 {
        // for index in 0..(world.size() as usize) {
        //     println!(
        //         "count: {}, displacement: {}",
        //         index_layout.counts()[index],
        //         index_layout.displacements()[index]
        //     );
        // }
        // println!("Square sum: {}", vec.view().abs_square_sum());
    }
}
