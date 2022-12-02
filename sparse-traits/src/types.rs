//! Basic types

// The scalar type used in the library.
pub use cauchy::Scalar;

// The `IndexType` is used whenever we use an integer counting type.
//
// By default it should be `usize`.
pub type IndexType = usize;

#[derive(Debug)]
pub enum Error {
    NotImplemented,
    OperationFailed,
}

pub type Result<T> = std::result::Result<T, Error>;
