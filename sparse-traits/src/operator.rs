//! General linear operator.

use crate::LinearSpace;
use std::fmt::Debug;

use crate::*;

// A base operator trait.
pub trait OperatorBase: Debug {
    type Domain: LinearSpace;
    type Range: LinearSpace;

    /// Returns a reference to trait object that supports application of the operator.
    ///
    /// By default it returns an `Err`. But for concrete types
    /// that support matvecs it is specialised to return
    /// a dynamic reference.
    fn as_apply(&self) -> Option<&dyn AsApply<Domain = Self::Domain, Range = Self::Range>> {
        None
    }

    // fn as_matvec_h(
    //     &self,
    // ) -> Option<&dyn AsHermitianMatVec<Domain = Self::Domain, Range = Self::Range>> {
    //     None
    // }

    // The following convenience routine returns true if an operator
    // supports `apply`. Applied to trait objects it provides a runtime
    // check about supported traits.
    fn has_apply(&self) -> bool {
        self.as_apply().is_some()
    }

    // Check if a given vector allows type conversion to the native type
    // of the operator.
    //fn is_compatible(&self, vec: &dyn Vector);
}

/// Apply an operator.
pub trait AsApply: OperatorBase {
    fn apply(&self, x: ElementView<Self::Domain>, y: ElementViewMut<Self::Range>) -> Result<()>;
}

// /// Matrix vector product $A^Hx$.
// pub trait AsHermitianMatVec: OperatorBase {
//     fn matvec_h(
//         &self,
//         x: &<<<Self as OperatorBase>::Range as spaces::LinearSpace>::E as Element>::View,
//         y: &mut <Self::Domain as LinearSpace>::E,
<<<<<<< HEAD
//     ) -> Result;
=======
//     ) -> Result<()>;
>>>>>>> main
// }

// /// Matrix vector product $A^Tx$.
// pub trait AsTransposeMatVec: OperatorBase {
//     fn matvec_t(
//         &self,
//         x: &<Self::Range as LinearSpace>::E,
//         y: &mut <Self::Domain as LinearSpace>::E,
<<<<<<< HEAD
//     ) -> Result;
// }

impl<In: LinearSpace, Out: LinearSpace> AsApply for dyn OperatorBase<Domain = In, Range = Out> {
    fn apply(&self, x: &ElementView<Self::Domain>, y: &mut ElementView<Self::Range>) -> Result {
=======
//     ) -> Result<()>;
// }

impl<In: LinearSpace, Out: LinearSpace> AsApply for dyn OperatorBase<Domain = In, Range = Out> {
    fn apply(&self, x: ElementView<Self::Domain>, y: ElementViewMut<Self::Range>) -> Result<()> {
>>>>>>> main
        if let Some(op) = self.as_apply() {
            op.apply(x, y)
        } else {
            Err(Error::NotImplemented)
        }
    }
}

// impl<In: LinearSpace, Out: LinearSpace> AsHermitianMatVec
//     for dyn OperatorBase<Domain = In, Range = Out>
// {
//     fn matvec_h(
//         &self,
//         x: &<Self::Range as LinearSpace>::E,
//         y: &mut <Self::Domain as LinearSpace>::E,
<<<<<<< HEAD
//     ) -> Result {
=======
//     ) -> Result<()> {
>>>>>>> main
//         if let Some(op) = self.as_matvec_h() {
//             op.matvec_h(x, y)
//         } else {
//             Err(Error::NotImplemented)
//         }
//     }
// }

#[cfg(test)]
mod tests {

<<<<<<< HEAD
=======
    use std::marker::PhantomData;

>>>>>>> main
    use super::*;

    #[derive(Debug)]
    struct SimpleSpace;
    impl LinearSpace for SimpleSpace {
        type F = f64;
        type E = SimpleVector;
    }

    #[derive(Debug)]
<<<<<<< HEAD
    struct SimpleVector {
        view: View,
    }

    #[derive(Debug)]
    struct View;

    impl Element for SimpleVector {
        type Space = SimpleSpace;
        type View = View;

        fn view(&self) -> &Self::View {
            &self.view
        }

        fn view_mut(&mut self) -> &mut Self::View {
            &mut self.view
=======
    struct SimpleVector {}

    #[derive(Debug)]
    struct View<'a> {
        marker: PhantomData<&'a ()>,
    }

    impl<'a> View<'a> {
        fn new() -> Self {
            Self {
                marker: PhantomData,
            }
        }
    }

    impl Element for SimpleVector {
        type Space = SimpleSpace;
        type View<'a> = View<'a> where Self: 'a;
        type ViewMut<'a> = View<'a> where Self: 'a;

        fn view<'a>(&'a self) -> Self::View<'a> {
            View::new()
        }

        fn view_mut<'a>(&'a mut self) -> Self::View<'a> {
            View::new()
>>>>>>> main
        }
    }

    #[derive(Debug)]
    struct SparseMatrix;
    impl OperatorBase for SparseMatrix {
        type Domain = SimpleSpace;
        type Range = SimpleSpace;

        fn as_apply(&self) -> Option<&dyn AsApply<Domain = Self::Domain, Range = Self::Range>> {
            Some(self)
        }
        // fn as_matvec_h(
        //     &self,
        // ) -> Option<&dyn AsHermitianMatVec<Domain = Self::Domain, Range = Self::Range>> {
        //     Some(self)
        // }
    }
    impl AsApply for SparseMatrix {
        fn apply(
            &self,
<<<<<<< HEAD
            _x: &ElementView<Self::Domain>,
            _y: &mut ElementView<Self::Range>,
        ) -> Result {
=======
            _x: ElementView<Self::Domain>,
            _y: ElementViewMut<Self::Range>,
        ) -> Result<()> {
>>>>>>> main
            println!("{self:?} matvec");
            Ok(())
        }
    }
    // impl AsHermitianMatVec for SparseMatrix {
    //     fn matvec_h(
    //         &self,
    //         _x: &<Self::Range as LinearSpace>::E,
    //         _y: &mut <Self::Domain as LinearSpace>::E,
<<<<<<< HEAD
    //     ) -> Result {
=======
    //     ) -> Result<()> {
>>>>>>> main
    //         println!("{self:?} matvec_h");
    //         Ok(())
    //     }
    // }

    // Finite difference matrices use the following formula where f is a
    // nonlinear function and x is a vector that we linearize around. It is not
    // tractable to apply the transpose or Hermitian adjoint without access to
    // the code that computes f.
    //
    // A y = (f(x + hy) - f(x)) / h
    #[derive(Debug)]
    struct FiniteDifference;
    impl OperatorBase for FiniteDifference {
        type Domain = SimpleSpace;
        type Range = SimpleSpace;
        fn as_apply(&self) -> Option<&dyn AsApply<Domain = Self::Domain, Range = Self::Range>> {
            Some(self)
        }
    }
    impl AsApply for FiniteDifference {
        fn apply(
            &self,
<<<<<<< HEAD
            _x: &ElementView<Self::Domain>,
            _y: &mut ElementView<Self::Range>,
        ) -> Result {
=======
            _x: ElementView<Self::Domain>,
            _y: ElementViewMut<Self::Range>,
        ) -> Result<()> {
>>>>>>> main
            println!("{self:?} matvec");
            Ok(())
        }
    }

    /// A fallible matrix
    #[derive(Debug)]
    struct SketchyMatrix;
    impl OperatorBase for SketchyMatrix {
        type Domain = SimpleSpace;
        type Range = SimpleSpace;
        fn as_apply(&self) -> Option<&dyn AsApply<Domain = Self::Domain, Range = Self::Range>> {
            Some(self)
        }
    }
    impl AsApply for SketchyMatrix {
        fn apply(
            &self,
<<<<<<< HEAD
            _x: &ElementView<Self::Domain>,
            _y: &mut ElementView<Self::Range>,
        ) -> Result {
=======
            _x: ElementView<Self::Domain>,
            _y: ElementViewMut<Self::Range>,
        ) -> Result<()> {
>>>>>>> main
            println!("{self:?} matvec");
            Err(Error::OperationFailed)
        }
    }
    #[test]
<<<<<<< HEAD
    fn test_mult_dyn() -> Result {
        let x = SimpleVector { view: View {} };
        let mut y = SimpleVector { view: View {} };
=======
    fn test_mult_dyn() -> Result<()> {
        let x = SimpleVector {};
        let mut y = SimpleVector {};
>>>>>>> main
        let ops: Vec<Box<dyn OperatorBase<Domain = SimpleSpace, Range = SimpleSpace>>> =
            vec![Box::new(SparseMatrix), Box::new(FiniteDifference)];
        for op in ops {
            op.apply(x.view(), y.view_mut())?;
        }
        Ok(())
    }

    #[test]
<<<<<<< HEAD
    fn test_mult() -> Result {
        let x = SimpleVector { view: View {} };
        let mut y = SimpleVector { view: View {} };
=======
    fn test_mult() -> Result<()> {
        let x = SimpleVector {};
        let mut y = SimpleVector {};
>>>>>>> main
        let a = SparseMatrix;
        // Static dispatch because we're using a struct that implements AsMatVec
        a.apply(x.view(), y.view_mut())?;
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_mult_sketchy() {
<<<<<<< HEAD
        let x = SimpleVector { view: View {} };
        let mut y = SimpleVector { view: View {} };
=======
        let x = SimpleVector {};
        let mut y = SimpleVector {};
>>>>>>> main
        let a = SketchyMatrix;
        // Static dispatch because we're using a struct that implements AsMatVec
        a.apply(x.view(), y.view_mut()).unwrap();
    }
}
