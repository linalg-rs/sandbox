//! General linear operator.

use std::fmt::Debug;
use crate::types::Scalar;

use crate::*;

#[derive(Debug)]
pub enum Error {
    NotImplemented,
    OperationFailed,
}

type Result = std::result::Result<(), Error>;

// A base operator trait.
pub trait OperatorBase: Debug {
    type In: Scalar;
    type Out: Scalar;

    /// Returns a reference to trait object that supports matvec.
    ///
    /// By default it returns an `Err`. But for concrete types
    /// that support matvecs it is specialised to return
    /// a dynamic reference.
    fn as_matvec(&self) -> Option<&dyn AsMatVec<In=Self::In, Out=Self::Out>> {
        None
    }

    fn as_matvec_h(&self) -> Option<&dyn AsHermitianMatVec<In=Self::In, Out=Self::Out>> {
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
    fn matvec(&self, x: &dyn Vector<Item=Self::In>, y: &mut dyn Vector<Item=Self::Out>) -> Result;
}

/// Matrix vector product $A^Hx$.
pub trait AsHermitianMatVec: OperatorBase {
    fn matvec_h(&self, x: &dyn Vector<Item=Self::Out>, y: &mut dyn Vector<Item=Self::In>) -> Result;
}

/// Matrix vector product $A^Tx$.
pub trait AsTransposeMatVec: OperatorBase {
    fn matvec_t(&self, x: &dyn Vector<Item=Self::Out>, y: &mut dyn Vector<Item=Self::In>) -> Result;
}

impl<In: Scalar, Out: Scalar> AsMatVec for dyn OperatorBase<In=In, Out=Out> {
    fn matvec(&self, x: &dyn Vector<Item=Self::In>, y: &mut dyn Vector<Item=Self::Out>) -> Result {
        if let Some(op) = self.as_matvec() {
            op.matvec(x, y)
        } else {
            Err(Error::NotImplemented)
        }
    }
}

impl<In: Scalar, Out: Scalar>  AsHermitianMatVec for dyn OperatorBase<In=In, Out=Out> {
    fn matvec_h(&self, x: &dyn Vector<Item=Self::Out>, y: &mut dyn Vector<Item=Self::In>) -> Result {
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
    struct SimpleVector;
    impl Vector for SimpleVector {type Item = f64;}

    #[derive(Debug)]
    struct SparseMatrix;
    impl OperatorBase for SparseMatrix {
        type In = f64;
        type Out= f64;

        fn as_matvec(&self) -> Option<&dyn AsMatVec<In=Self::In, Out=Self::Out>> {
            Some(self)
        }
        fn as_matvec_h(&self) -> Option<&dyn AsHermitianMatVec<In=Self::In, Out=Self::Out>> {
            Some(self)
        }
    }
    impl AsMatVec for SparseMatrix {
        fn matvec(&self, _x: &dyn Vector<Item=Self::In>, _y: &mut dyn Vector<Item=Self::Out>) -> Result {
            println!("{self:?} matvec");
            Ok(())
        }
    }
    impl AsHermitianMatVec for SparseMatrix {
        fn matvec_h(&self, _x: &dyn Vector<Item=Self::Out>, _y: &mut dyn Vector<Item=Self::In>) -> Result {
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
        type In = f64;
        type Out = f64;
        fn as_matvec(&self) -> Option<&dyn AsMatVec<In=Self::In, Out=Self::Out>> {
            Some(self)
        }
    }
    impl AsMatVec for FiniteDifference {
        fn matvec(&self, _x: &dyn Vector<Item=Self::In>, _y: &mut dyn Vector<Item=Self::Out>) -> Result {
            println!("{self:?} matvec");
            Ok(())
        }
    }

    /// A fallible matrix
    #[derive(Debug)]
    struct SketchyMatrix;
    impl OperatorBase for SketchyMatrix {
        type In = f64;
        type Out = f64;
        fn as_matvec(&self) -> Option<&dyn AsMatVec<In=Self::In, Out=Self::Out>> {
            Some(self)
        }
    }
    impl AsMatVec for SketchyMatrix {
        fn matvec(&self, _x: &dyn Vector<Item=Self::In>, _y: &mut dyn Vector<Item=Self::Out>) -> Result {
            println!("{self:?} matvec");
            Err(Error::OperationFailed)
        }
    }
    #[test]
    fn test_mult_dyn() -> Result {
        let x = SimpleVector;
        let mut y = SimpleVector;
        let ops: Vec<Box<dyn OperatorBase<In=f64, Out=f64>>> =
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
