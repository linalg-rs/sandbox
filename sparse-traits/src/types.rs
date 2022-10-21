//! Basic types

use cauchy::Scalar;

pub trait GeneralScalar: cauchy::Scalar {}

impl<T: Scalar> GeneralScalar for T {}
