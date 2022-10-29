//! Basic types

use num::traits::{One, Zero};
use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

pub trait GeneralScalar:
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
            + Sized
            + PartialEq
            + Zero
            + One
            + Sized,
    > GeneralScalar for T
{
}

pub type IndexType = usize;
