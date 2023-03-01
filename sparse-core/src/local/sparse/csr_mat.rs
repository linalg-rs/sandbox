//! Definition of CSR matrices.

use crate::local::sparse::SparseMatType;
use sparse_traits::types::SparseLinAlgResult;

use sparse_traits::{
    linalg::IndexableVectorView,
    linalg::IndexableVectorViewMut,
    types::{IndexType, Scalar},
};

pub struct CsrMatrix<T: Scalar> {
    mat_type: SparseMatType,
    shape: (IndexType, IndexType),
    indices: Vec<IndexType>,
    indptr: Vec<IndexType>,
    data: Vec<T>,
}

impl<T: Scalar> CsrMatrix<T> {
    pub fn new(
        shape: (IndexType, IndexType),
        indices: Vec<IndexType>,
        indptr: Vec<IndexType>,
        data: Vec<T>,
    ) -> Self {
        Self {
            mat_type: SparseMatType::Csr,
            shape,
            indices,
            indptr,
            data,
        }
    }

    pub fn mat_type(&self) -> &SparseMatType {
        &self.mat_type
    }

    pub fn shape(&self) -> (IndexType, IndexType) {
        self.shape
    }

    pub fn indices(&self) -> &[IndexType] {
        &self.indices
    }

    pub fn indptr(&self) -> &[IndexType] {
        &self.indptr
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn matmul<V: IndexableVectorView<T = T>, VM: IndexableVectorViewMut<T = T>>(
        &self,
        alpha: T,
        x: &V,
        beta: T,
        y: &mut VM,
    ) {
        for (row, out) in y.iter_mut().enumerate() {
            *out = beta * *out
                + alpha * {
                    let c1 = self.indptr()[row];
                    let c2 = self.indptr()[1 + row];
                    let mut acc = T::zero();

                    for index in c1..c2 {
                        unsafe {
                            let col = *self.indices().get_unchecked(index);
                            acc += *self.data().get_unchecked(index) * *x.get_unchecked(col);
                        }
                    }
                    acc
                }
        }
    }

    pub fn from_aij(
        shape: (IndexType, IndexType),
        rows: &[IndexType],
        cols: &[IndexType],
        data: &[T],
    ) -> SparseLinAlgResult<Self> {
        let mut sorted: Vec<IndexType> = (0..rows.len()).collect();
        sorted.sort_by_key(|&idx| rows[idx]);

        let nelems = data.len();

        let mut indptr = Vec::<IndexType>::with_capacity(1 + shape.0);
        let mut indices = Vec::<IndexType>::with_capacity(nelems);
        let mut new_data = Vec::<T>::with_capacity(nelems);

        let mut count: IndexType = 0;

        for row in 0..(shape.0) {
            indptr.push(count);
            while count < nelems && row == rows[sorted[count]] {
                count += 1;
            }
        }
        indptr.push(count);

        for index in 0..nelems {
            indices.push(cols[sorted[index]]);
            new_data.push(data[sorted[index]]);
        }

        Ok(Self::new(shape, indices, indptr, new_data))
    }
}

#[cfg(test)]
mod test {

    use sparse_traits::linalg::IndexableVector;

    use crate::local::index_layout::LocalIndexLayout;
    use crate::local::indexable_vector::LocalIndexableVector;

    use super::*;

    #[test]
    fn test_csr_from_aij() {
        // Test the matrix [[1, 2], [3, 4]]
        let rows = vec![0, 0, 1, 1];
        let cols = vec![0, 1, 0, 1];
        let data = vec![1.0, 2.0, 3.0, 4.0];

        let csr = CsrMatrix::from_aij((2, 2), &rows, &cols, &data).unwrap();

        assert_eq!(csr.data().len(), 4);
        assert_eq!(csr.indices().len(), 4);
        assert_eq!(csr.indptr().len(), 3);

        //Test the matrix [[0, 0, 0], [2.0, 0, 0], [0, 0, 0]]
        let rows = vec![1];
        let cols = vec![0];
        let data = vec![2.0];

        let csr = CsrMatrix::from_aij((3, 3), &rows, &cols, &data).unwrap();

        assert_eq!(csr.indptr()[0], 0);
        assert_eq!(csr.indptr()[1], 0);
        assert_eq!(csr.indptr()[2], 1);
        assert_eq!(csr.indptr()[3], 1);
    }

    #[test]
    fn test_csr_matmul() {
        // Test the matrix [[1, 2], [3, 4]]
        let rows = vec![0, 0, 1, 1];
        let cols = vec![0, 1, 0, 1];
        let data = vec![1.0, 2.0, 3.0, 4.0];

        let csr = CsrMatrix::from_aij((2, 2), &rows, &cols, &data).unwrap();

        // Execute 2 * [1, 2] + 3 * A*x with x = [3, 4];
        // Expected result is [35, 79].

        let index_layout = LocalIndexLayout::new((0, 2));
        let mut res = LocalIndexableVector::<f64>::new(&index_layout);

        let mut x = LocalIndexableVector::<f64>::new(&index_layout);

        *x.view_mut().unwrap().get_mut(0).unwrap() = 3.0;
        *x.view_mut().unwrap().get_mut(1).unwrap() = 4.0;

        *res.view_mut().unwrap().get_mut(0).unwrap() = 1.0;
        *res.view_mut().unwrap().get_mut(1).unwrap() = 2.0;

        csr.matmul(3.0, &x.view().unwrap(), 2.0, &mut res.view_mut().unwrap());

        assert_eq!(*res.view().unwrap().get(0).unwrap(), 35.0);
        assert_eq!(*res.view().unwrap().get(1).unwrap(), 79.0);
    }
}
