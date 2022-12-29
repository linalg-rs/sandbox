//! Example file for creating vectors.

use sparse_core::local::indexable_space::LocalIndexableVectorSpace;
use sparse_traits::Element;
use sparse_traits::IndexableVector;
use sparse_traits::LinearSpace;
use sparse_traits::Norm2;

fn main() {
    let space = LocalIndexableVectorSpace::<f64>::new(10);
    let mut vec = space.create_element();

    *vec.view_mut().get_mut(0).unwrap() = 2.0;

    let n = vec.view().len();
    println!("The dimension of the vector is {}", n);
    println!("The norm of the vector is {}", vec.view().norm_2());
}
