//! General linear operator.

use std::fmt::Debug;

use crate::*;

// A base operator trait.
pub trait OperatorBase: Debug {
    /// Returns a reference to trait object that supports matvec.
    ///
    /// By default it returns an `Err`. But for concrete types
    /// that support matvecs it is specialised to return
    /// a dynamic reference.
    fn as_matvec(&self) -> Option<&dyn AsMatVec> {
        None
    }

    fn as_matvec_h(&self) -> Option<&dyn AsHermitianMatVec> {
        None
    }

    // The following convenience routine returns true if an operator
    // supports matvecs. Applied to trait objects it provides a runtime
    // check about supported traits.
    fn has_matvec(&self) -> bool {
        self.as_matvec().is_some()
    }
}

/// Matrix vector product $Ax$.
pub trait AsMatVec: OperatorBase {
    fn matvec(&self, x: &dyn Vector, y: &mut dyn Vector);
}

/// Matrix vector product $A^Hx$.
pub trait AsHermitianMatVec: OperatorBase {
    fn matvec_h(&self, x: &dyn Vector, y: &mut dyn Vector);
}

/// Matrix vector product $A^Tx$.
pub trait AsTransposeMatVec: OperatorBase {
    fn matvec_t(&self, x: &dyn Vector, y: &mut dyn Vector);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct SimpleVector;
    impl Vector for SimpleVector {}

    #[derive(Debug)]
    struct SparseMatrix;
    impl OperatorBase for SparseMatrix {
        fn as_matvec(&self) -> Option<&dyn AsMatVec> {
            Some(self)
        }
        fn as_matvec_h(&self) -> Option<&dyn AsHermitianMatVec> {
            Some(self)
        }
    }
    impl AsMatVec for SparseMatrix {
        fn matvec(&self, _x: &dyn Vector, _y: &mut dyn Vector) {
            println!("{self:?} matvec");
        }
    }
    impl AsHermitianMatVec for SparseMatrix {
        fn matvec_h(&self, _x: &dyn Vector, _y: &mut dyn Vector) {
            println!("{self:?} matvec_h");
        }
    }

    #[derive(Debug)]
    struct FiniteDifference;
    impl OperatorBase for FiniteDifference {
        fn as_matvec(&self) -> Option<&dyn AsMatVec> {
            Some(self)
        }
    }
    impl AsMatVec for FiniteDifference {
        fn matvec(&self, _x: &dyn Vector, _y: &mut dyn Vector) {
            println!("{self:?} matvec");
        }
    }

    #[test]
    fn test_mult_dyn() {
        let x = SimpleVector;
        let mut y = SimpleVector;
        let ops: Vec<Box<dyn OperatorBase>> =
            vec![Box::new(SparseMatrix), Box::new(FiniteDifference)];
        for op in ops {
            if let Some(mat) = op.as_matvec() {
                mat.matvec(&x, &mut y);
            } else {
                eprintln!("{op:?} has no matvec");
            }
            if let Some(mat) = op.as_matvec_h() {
                mat.matvec_h(&x, &mut y);
            } else {
                eprintln!("{op:?} has no matvec_h");
            }
        }
    }

    #[test]
    fn test_mult() {
        let x = SimpleVector;
        let mut y = SimpleVector;
        let a = SparseMatrix;
        // Static dispatch because we're using a struct that implements AsMatVec
        a.matvec(&x, &mut y);
        a.matvec_h(&x, &mut y);
    }
}
