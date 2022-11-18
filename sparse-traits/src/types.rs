//! Basic types

use num::traits::{One, Zero};
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

// pub trait Scalar:
//     Add<Self, Output = Self>
//     + Sub<Self, Output = Self>
//     + Mul<Self, Output = Self>
//     + PartialEq
//     + Zero
//     + One
//     + Sized
// {
// }

pub trait Scalar: cauchy::Scalar {}
impl<T: cauchy::Scalar> Scalar for T {}

// impl<
//         T: Add<T, Output = T>
//             + Sub<T, Output = T>
//             + Mul<T, Output = T>
//             + Div<T, Output = T>
//             + Sized
//             + PartialEq
//             + Zero
//             + One
//             + Sized,
//     > Scalar for T
// {
// }

pub type IndexType = usize;

//pub trait RealType: num::traits::Float {}

//impl<T: num::traits::Float> RealType for T {}

#[derive(Debug)]
pub enum Error {
    NotImplemented,
    OperationFailed,
}

pub type Result = std::result::Result<(), Error>;
