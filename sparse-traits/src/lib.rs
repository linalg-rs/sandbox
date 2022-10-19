//! This is a sandbox to test traits for sparse (and more general) operators.

pub mod operator;
pub mod types;
pub mod linear_space;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
