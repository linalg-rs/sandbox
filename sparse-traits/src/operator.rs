//! General linear operator.

use crate::Space;
use std::fmt::Debug;

use crate::*;

// A base operator trait.
pub trait OperatorBase: Debug {
    type Domain: Space;
    type Range: Space;

    /// Returns a reference to trait object that supports matvec.
    ///
    /// By default it returns an `Err`. But for concrete types
    /// that support matvecs it is specialised to return
    /// a dynamic reference.
    fn as_matvec(&self) -> Option<&dyn AsMatVec<Domain = Self::Domain, Range = Self::Range>> {
        None
    }

    fn as_matvec_h(
        &self,
    ) -> Option<&dyn AsHermitianMatVec<Domain = Self::Domain, Range = Self::Range>> {
        None
    }

    // The following convenience routine returns true if an operator
    // supports matvecs. Applied to trait objects it provides a runtime
    // check about supported traits.
    fn has_matvec(&self) -> bool {
        self.as_matvec().is_some()
    }

    // Check if a given vector allows type conversion to the native type
    // of the operator.
    //fn is_compatible(&self, vec: &dyn Vector);
}

/// Matrix vector product $Ax$.
pub trait AsMatVec: OperatorBase {
    fn matvec(&self, x: &<Self::Domain as Space>::E, y: &mut <Self::Range as Space>::E) -> Result;
}

/// Matrix vector product $A^Hx$.
pub trait AsHermitianMatVec: OperatorBase {
    fn matvec_h(
        &self,
        x: &<Self::Range as Space>::VectorType,
        y: &mut <Self::Domain as Space>::VectorType,
    ) -> Result;
}

/// Matrix vector product $A^Tx$.
pub trait AsTransposeMatVec: OperatorBase {
    fn matvec_t(
        &self,
        x: &<Self::Range as Space>::VectorType,
        y: &mut <Self::Domain as Space>::VectorType,
    ) -> Result;
}

impl<In: Space, Out: Space> AsMatVec for dyn OperatorBase<Domain = In, Range = Out> {
    fn matvec(
        &self,
        x: &<Self::Domain as Space>::VectorType,
        y: &mut <Self::Range as Space>::VectorType,
    ) -> Result {
        if let Some(op) = self.as_matvec() {
            op.matvec(x, y)
        } else {
            Err(Error::NotImplemented)
        }
    }
}

impl<In: Space, Out: Space> AsHermitianMatVec for dyn OperatorBase<Domain = In, Range = Out> {
    fn matvec_h(
        &self,
        x: &<Self::Range as Space>::VectorType,
        y: &mut <Self::Domain as Space>::VectorType,
    ) -> Result {
        if let Some(op) = self.as_matvec_h() {
            op.matvec_h(x, y)
        } else {
            Err(Error::NotImplemented)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug)]
    struct SimpleSpace;
    impl Space for SimpleSpace {
        type Item = f64;
        type VectorType = SimpleVector;
        type Real = f64;
    }

    #[derive(Debug)]
    struct SimpleVector;
    struct View;
    impl FiniteVectorView for View {
        type Item = f64;
    }

    impl Element for SimpleVector {
        type Space = SimpleSpace;
        type View = View;
    }

    #[derive(Debug)]
    struct SparseMatrix;
    impl OperatorBase for SparseMatrix {
        type Domain = SimpleSpace;
        type Range = SimpleSpace;

        fn as_matvec(&self) -> Option<&dyn AsMatVec<Domain = Self::Domain, Range = Self::Range>> {
            Some(self)
        }
        fn as_matvec_h(
            &self,
        ) -> Option<&dyn AsHermitianMatVec<Domain = Self::Domain, Range = Self::Range>> {
            Some(self)
        }
    }
    impl AsMatVec for SparseMatrix {
        fn matvec(
            &self,
            _x: &<Self::Domain as Space>::VectorType,
            _y: &mut <Self::Range as Space>::VectorType,
        ) -> Result {
            println!("{self:?} matvec");
            Ok(())
        }
    }
    impl AsHermitianMatVec for SparseMatrix {
        fn matvec_h(
            &self,
            _x: &<Self::Range as Space>::VectorType,
            _y: &mut <Self::Domain as Space>::VectorType,
        ) -> Result {
            println!("{self:?} matvec_h");
            Ok(())
        }
    }

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
        fn as_matvec(&self) -> Option<&dyn AsMatVec<Domain = Self::Domain, Range = Self::Range>> {
            Some(self)
        }
    }
    impl AsMatVec for FiniteDifference {
        fn matvec(
            &self,
            _x: &<Self::Domain as Space>::VectorType,
            _y: &mut <Self::Range as Space>::VectorType,
        ) -> Result {
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
        fn as_matvec(&self) -> Option<&dyn AsMatVec<Domain = Self::Domain, Range = Self::Range>> {
            Some(self)
        }
    }
    impl AsMatVec for SketchyMatrix {
        fn matvec(
            &self,
            _x: &<Self::Domain as Space>::VectorType,
            _y: &mut <Self::Range as Space>::VectorType,
        ) -> Result {
            println!("{self:?} matvec");
            Err(Error::OperationFailed)
        }
    }
    #[test]
    fn test_mult_dyn() -> Result {
        let x = SimpleVector;
        let mut y = SimpleVector;
        let ops: Vec<Box<dyn OperatorBase<Domain = SimpleSpace, Range = SimpleSpace>>> =
            vec![Box::new(SparseMatrix), Box::new(FiniteDifference)];
        for op in ops {
            op.matvec(&x, &mut y)?;
            op.matvec_h(&x, &mut y).or_else(|e| {
                eprintln!("{op:?} reported {e:?}");
                Ok(())
            })?;
        }
        Ok(())
    }

    #[test]
    fn test_mult() -> Result {
        let x = SimpleVector;
        let mut y = SimpleVector;
        let a = SparseMatrix;
        // Static dispatch because we're using a struct that implements AsMatVec
        a.matvec(&x, &mut y)?;
        a.matvec_h(&x, &mut y)?;
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_mult_sketchy() {
        let x = SimpleVector;
        let mut y = SimpleVector;
        let a = SketchyMatrix;
        // Static dispatch because we're using a struct that implements AsMatVec
        a.matvec(&x, &mut y).unwrap();
    }
}
