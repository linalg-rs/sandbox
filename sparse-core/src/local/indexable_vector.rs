//! An Indexable Vector is a container whose elements can be 1d indexed.
use num::{Float, Zero};
use sparse_traits::linalg::traits::*;
use sparse_traits::linalg::*;
use sparse_traits::types::{Error, Result};
use sparse_traits::Scalar;
use sparse_traits::{IndexLayout, IndexType};

use super::index_layout::LocalIndexLayout;

pub struct LocalIndexableVector<'a, T: Scalar> {
    data: Vec<T>,
    index_layout: &'a LocalIndexLayout,
}

pub struct LocalIndexableVectorView<'a, T: Scalar> {
    data: &'a Vec<T>,
}

pub struct LocalIndexableVectorViewMut<'a, T: Scalar> {
    data: &'a mut Vec<T>,
}

impl<'a, T: Scalar> LocalIndexableVector<'a, T> {
    pub fn new(index_layout: &'a LocalIndexLayout) -> LocalIndexableVector<'a, T> {
        LocalIndexableVector {
            data: vec![T::zero(); index_layout.number_of_global_indices()],
            index_layout,
        }
    }
}

impl<T: Scalar> IndexableVector for LocalIndexableVector<'_, T> {
    type T = T;
    type Ind = LocalIndexLayout;
    type View<'a> = LocalIndexableVectorView<'a, T> where Self: 'a;
    type ViewMut<'a> = LocalIndexableVectorViewMut<'a, T> where Self: 'a;

    fn index_layout(&self) -> &Self::Ind {
        &self.index_layout
    }

    fn view<'a>(&'a self) -> Option<Self::View<'a>> {
        Some(LocalIndexableVectorView { data: &self.data })
    }

    fn view_mut<'a>(&'a mut self) -> Option<Self::ViewMut<'a>> {
        Some(LocalIndexableVectorViewMut {
            data: &mut self.data,
        })
    }
}

macro_rules! implement_view {
    ($ViewType:ident) => {
        impl<T: Scalar> IndexableVectorView for $ViewType<'_, T> {
            type Iter<'b> = std::slice::Iter<'b, T> where Self: 'b;

            type T = T;

            fn get(&self, index: IndexType) -> Option<&Self::T> {
                self.data.get(index)
            }

            unsafe fn get_unchecked(&self, index: IndexType) -> &Self::T {
                self.data.get_unchecked(index)
            }

            fn iter(&self) -> Self::Iter<'_> {
                self.data.as_slice().iter()
            }

            fn len(&self) -> IndexType {
                self.data.len()
            }
        }
    };
}
implement_view!(LocalIndexableVectorView);
implement_view!(LocalIndexableVectorViewMut);

impl<T: Scalar> IndexableVectorViewMut for LocalIndexableVectorViewMut<'_, T> {
    type IterMut<'b> = std::slice::IterMut<'b, T> where Self: 'b;

    fn get_mut(&mut self, index: IndexType) -> Option<&mut Self::T> {
        self.data.get_mut(index)
    }

    unsafe fn get_unchecked_mut(&mut self, index: IndexType) -> &mut Self::T {
        self.data.get_unchecked_mut(index)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.data.as_mut_slice().iter_mut()
    }
}

impl<T: Scalar> Inner for LocalIndexableVector<'_, T> {
    type T = T;
    fn inner(&self, other: &Self) -> Result<Self::T> {
        let my_view = self.view().unwrap();
        let other_view = other.view().unwrap();
        if !self.index_layout().is_same(other.index_layout()) {
            return Err(Error::OperationFailed);
        }
        let result = my_view
            .iter()
            .zip(other_view.iter())
            .fold(<Self::T as Zero>::zero(), |acc, (&first, &second)| {
                acc + first * second.conj()
            });
        Ok(result)
    }
}

impl<T: Scalar> AbsSquareSum for LocalIndexableVector<'_, T> {
    type T = T;
    fn abs_square_sum(&self) -> <Self::T as Scalar>::Real {
        self.view()
            .unwrap()
            .iter()
            .fold(<<Self::T as Scalar>::Real>::zero(), |acc, &elem| {
                acc + elem.square()
            })
    }
}

impl<T: Scalar> Norm1 for LocalIndexableVector<'_, T> {
    type T = T;
    fn norm_1(&self) -> <Self::T as Scalar>::Real {
        self.view()
            .unwrap()
            .iter()
            .fold(<<Self::T as Scalar>::Real>::zero(), |acc, &elem| {
                acc + elem.abs()
            })
    }
}

impl<T: Scalar> Norm2 for LocalIndexableVector<'_, T> {
    type T = T;
    fn norm_2(&self) -> <Self::T as Scalar>::Real {
        <<Self::T as Scalar>::Real as Float>::sqrt(self.abs_square_sum())
    }
}

impl<T: Scalar> NormInf for LocalIndexableVector<'_, T> {
    type T = T;
    fn norm_inf(&self) -> <Self::T as Scalar>::Real {
        self.view().unwrap().iter().fold(
            <<Self::T as Scalar>::Real as Float>::neg_infinity(),
            |acc, &elem| <<Self::T as Scalar>::Real as Float>::max(acc, elem.abs()),
        )
    }
}

impl<T: Scalar> Swap for LocalIndexableVector<'_, T> {
    type T = T;
    fn swap(&mut self, other: &mut Self) -> sparse_traits::types::Result<()> {
        if !self.index_layout().is_same(other.index_layout()) {
            return Err(Error::OperationFailed);
        } else {
            let mut my_view = self.view_mut().unwrap();
            let mut other_view = other.view_mut().unwrap();
            for (first, second) in my_view.iter_mut().zip(other_view.iter_mut()) {
                std::mem::swap(first, second);
            }
            Ok(())
        }
    }
}

impl<T: Scalar> Fill for LocalIndexableVector<'_, T> {
    type T = T;
    fn fill(&mut self, other: &Self) -> sparse_traits::types::Result<()> {
        if !self.index_layout().is_same(other.index_layout()) {
            return Err(Error::OperationFailed);
        } else {
            let mut my_view = self.view_mut().unwrap();
            let other_view = other.view().unwrap();
            for (first, second) in my_view.iter_mut().zip(other_view.iter()) {
                *first = *second;
            }
            Ok(())
        }
    }
}

impl<T: Scalar> ScalarMult for LocalIndexableVector<'_, T> {
    type T = T;
    fn scalar_mult(&mut self, scalar: Self::T) {
        for elem in self.view_mut().unwrap().iter_mut() {
            *elem *= scalar;
        }
    }
}

impl<T: Scalar> MultSumInto for LocalIndexableVector<'_, T> {
    type T = T;
    fn mult_sum_into(&mut self, other: &Self, scalar: Self::T) -> sparse_traits::types::Result<()> {
        if !self.index_layout().is_same(other.index_layout()) {
            return Err(Error::OperationFailed);
        }
        let mut my_view = self.view_mut().unwrap();
        let other_view = other.view().unwrap();
        if scalar == T::zero() {
            return Ok(());
        }
        if scalar == T::one() {
            for (first, second) in my_view.iter_mut().zip(other_view.iter()) {
                *first += *second;
            }
            return Ok(());
        }
        for (first, second) in my_view.iter_mut().zip(other_view.iter()) {
            *first += scalar * *second;
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use cauchy::c64;
    use float_eq;

    const VEC_SIZE: IndexType = 2;

    fn index_layout() -> LocalIndexLayout {
        LocalIndexLayout::new((0, VEC_SIZE))
    }

    fn new_vec<'a, T: Scalar>(index_layout: &'a LocalIndexLayout) -> LocalIndexableVector<'a, T> {
        LocalIndexableVector::<'_, T>::new(index_layout)
    }

    #[test]
    fn test_inner() {
        let index_layout = index_layout();

        let mut vec1 = new_vec::<c64>(&index_layout);
        let mut vec2 = new_vec::<c64>(&index_layout);

        let mut vec1_view = vec1.view_mut().unwrap();
        let mut vec2_view = vec2.view_mut().unwrap();

        *vec1_view.get_mut(0).unwrap() = c64::new(1.0, 2.0);
        *vec1_view.get_mut(1).unwrap() = c64::new(0.5, 1.0);

        *vec2_view.get_mut(0).unwrap() = c64::new(2.0, 3.0);
        *vec2_view.get_mut(1).unwrap() = c64::new(0.4, 1.5);

        let actual = vec1.inner(&vec2).unwrap();

        let expected =
            c64::new(1.0, 2.0) * c64::new(2.0, -3.0) + c64::new(0.5, 1.0) * c64::new(0.4, -1.5);

        assert_eq!(actual, expected);
    }

    #[test]
    fn abs_square_sum() {
        let index_layout = index_layout();

        let mut vec = new_vec::<c64>(&index_layout);

        let mut vec_view = vec.view_mut().unwrap();

        let val1 = c64::new(1.0, 2.0);
        let val2 = c64::new(1.5, 3.0);

        *vec_view.get_mut(0).unwrap() = val1;
        *vec_view.get_mut(1).unwrap() = val2;

        let actual = vec.abs_square_sum();
        let expected = val1.abs() * val1.abs() + val2.abs() * val2.abs();

        float_eq::assert_float_eq!(actual, expected, ulps_all <= 4);
    }

    #[test]
    fn norm_1() {
        let index_layout = index_layout();

        let mut vec = new_vec::<c64>(&index_layout);

        let val1 = c64::new(1.0, 2.0);
        let val2 = c64::new(1.5, 3.0);

        *vec.view_mut().unwrap().get_mut(0).unwrap() = val1;
        *vec.view_mut().unwrap().get_mut(1).unwrap() = val2;

        let actual = vec.norm_1();
        let expected = val1.abs() + val2.abs();

        float_eq::assert_float_eq!(actual, expected, ulps_all <= 4);
    }

    #[test]
    fn norm_2() {
        let index_layout = index_layout();

        let mut vec = new_vec::<c64>(&index_layout);

        let val1 = c64::new(1.0, 2.0);
        let val2 = c64::new(1.5, 3.0);

        *vec.view_mut().unwrap().get_mut(0).unwrap() = val1;
        *vec.view_mut().unwrap().get_mut(1).unwrap() = val2;

        let actual = vec.norm_2();
        let expected = (val1.abs() * val1.abs() + val2.abs() * val2.abs()).sqrt();

        float_eq::assert_float_eq!(actual, expected, ulps_all <= 4);
    }

    #[test]
    fn norm_inf() {
        let index_layout = index_layout();

        let mut vec = new_vec::<c64>(&index_layout);

        let val1 = c64::new(1.0, 2.0);
        let val2 = c64::new(1.5, 3.0);

        *vec.view_mut().unwrap().get_mut(0).unwrap() = val1;
        *vec.view_mut().unwrap().get_mut(1).unwrap() = val2;

        let actual = vec.norm_inf();
        let expected = val2.abs();

        float_eq::assert_float_eq!(actual, expected, ulps_all <= 4);
    }

    #[test]
    fn swap() {
        let index_layout = index_layout();

        let mut vec1 = new_vec::<c64>(&index_layout);
        let mut vec2 = new_vec::<c64>(&index_layout);

        let mut vec1_view = vec1.view_mut().unwrap();
        let mut vec2_view = vec2.view_mut().unwrap();

        *vec1_view.get_mut(0).unwrap() = c64::new(1.0, 2.0);
        *vec1_view.get_mut(1).unwrap() = c64::new(0.5, 1.0);

        *vec2_view.get_mut(0).unwrap() = c64::new(2.0, 3.0);
        *vec2_view.get_mut(1).unwrap() = c64::new(0.4, 1.5);

        vec1.swap(&mut vec2).unwrap();

        assert_eq!(*vec1.view().unwrap().get(0).unwrap(), c64::new(2.0, 3.0));
        assert_eq!(*vec2.view().unwrap().get(1).unwrap(), c64::new(0.5, 1.0));
    }

    #[test]
    fn mult_sum_into() {
        let index_layout = index_layout();

        let mut vec1 = new_vec::<c64>(&index_layout);
        let mut vec2 = new_vec::<c64>(&index_layout);

        let mut vec1_view = vec1.view_mut().unwrap();
        let mut vec2_view = vec2.view_mut().unwrap();

        *vec1_view.get_mut(0).unwrap() = c64::new(1.0, 2.0);
        *vec1_view.get_mut(1).unwrap() = c64::new(0.5, 1.0);

        *vec2_view.get_mut(0).unwrap() = c64::new(2.0, 3.0);
        *vec2_view.get_mut(1).unwrap() = c64::new(0.4, 1.5);
        // Test scalar = 0

        let _ = vec1.mult_sum_into(&vec2, c64::new(0.0, 0.0));

        assert_eq!(*vec1.view().unwrap().get(0).unwrap(), c64::new(1.0, 2.0));
        assert_eq!(*vec1.view().unwrap().get(1).unwrap(), c64::new(0.5, 1.0));

        *vec1.view_mut().unwrap().get_mut(0).unwrap() = c64::new(1.0, 2.0);
        *vec1.view_mut().unwrap().get_mut(1).unwrap() = c64::new(0.5, 1.0);

        // Test scalar = 1
        let _ = vec1.mult_sum_into(&vec2, c64::new(1.0, 0.0));

        assert_eq!(
            *vec1.view().unwrap().get(0).unwrap(),
            c64::new(1.0, 2.0) + c64::new(2.0, 3.0)
        );
        assert_eq!(
            *vec1.view().unwrap().get(1).unwrap(),
            c64::new(0.5, 1.0) + c64::new(0.4, 1.5)
        );

        *vec1.view_mut().unwrap().get_mut(0).unwrap() = c64::new(1.0, 2.0);
        *vec1.view_mut().unwrap().get_mut(1).unwrap() = c64::new(0.5, 1.0);

        // Test scalar = 1.3
        let _ = vec1.mult_sum_into(&vec2, c64::new(1.3, 0.0));

        assert_eq!(
            *vec1.view().unwrap().get(0).unwrap(),
            c64::new(1.0, 2.0) + c64::new(1.3, 0.0) * c64::new(2.0, 3.0)
        );
        assert_eq!(
            *vec1.view().unwrap().get(1).unwrap(),
            c64::new(0.5, 1.0) + c64::new(1.3, 0.0) * c64::new(0.4, 1.5)
        );
    }
    #[test]
    fn scalar_mult() {
        let index_layout = index_layout();

        let mut vec = new_vec::<c64>(&index_layout);

        let val1 = c64::new(1.0, 2.0);
        let val2 = c64::new(1.5, 3.0);

        *vec.view_mut().unwrap().get_mut(0).unwrap() = val1;
        *vec.view_mut().unwrap().get_mut(1).unwrap() = val2;

        vec.scalar_mult(c64::new(2.1, 3.5));

        assert_eq!(
            *vec.view().unwrap().get(0).unwrap(),
            c64::new(2.1, 3.5) * c64::new(1.0, 2.0)
        );
        assert_eq!(
            *vec.view().unwrap().get(1).unwrap(),
            c64::new(2.1, 3.5) * c64::new(1.5, 3.0)
        );
    }
}
