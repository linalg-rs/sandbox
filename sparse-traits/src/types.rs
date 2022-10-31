//! Basic types

use num::traits::{One, Zero};
use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub, Div};

pub trait Scalar:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + PartialEq
    + Zero
    + One
    + Sized
{
}

impl<
        T: Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + Sized
            + PartialEq
            + Zero
            + One
            + Sized,
    > Scalar for T
{
}

pub type IndexType = usize;
