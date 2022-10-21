use sparse_traits::linear_space::CollectionOfVectors;
use sparse_traits::operator::*;
use sparse_traits::types::*;
use std::marker::PhantomData;

/// A base operator struct.
pub struct OperatorWithMatVec<InputType: GeneralScalar, OutputType: GeneralScalar> {
    _input: std::marker::PhantomData<InputType>,
    _output: std::marker::PhantomData<OutputType>,
}

pub struct OperatorWithoutMatVec<InputType: GeneralScalar, OutputType: GeneralScalar> {
    _input: std::marker::PhantomData<InputType>,
    _output: std::marker::PhantomData<OutputType>,
}

impl<InputType: GeneralScalar, OutputType: GeneralScalar>
    OperatorWithMatVec<InputType, OutputType>
{
    pub fn new() -> Self {
        Self {
            _input: PhantomData,
            _output: PhantomData,
        }
    }

    pub fn to_box(self) -> Box<dyn OperatorBase<InputType, OutputType>> {
        Box::new(self)
    }
}

impl<InputType: GeneralScalar, OutputType: GeneralScalar>
    OperatorWithoutMatVec<InputType, OutputType>
{
    pub fn new() -> Self {
        Self {
            _input: PhantomData,
            _output: PhantomData,
        }
    }

    pub fn to_box(self) -> Box<dyn OperatorBase<InputType, OutputType>> {
        Box::new(self)
    }
}

impl<InputType: GeneralScalar, OutputType: GeneralScalar> OperatorBase<InputType, OutputType>
    for OperatorWithoutMatVec<InputType, OutputType>
{
}

impl<InputType: GeneralScalar, OutputType: GeneralScalar> OperatorBase<InputType, OutputType>
    for OperatorWithMatVec<InputType, OutputType>
{
    fn as_matvec(&self) -> Result<&dyn AsMatVec<InputType, OutputType>, ()> {
        Ok(self as &dyn AsMatVec<InputType, OutputType>)
    }
}

impl<InputType: GeneralScalar, OutputType: GeneralScalar> AsMatVec<InputType, OutputType>
    for OperatorWithMatVec<InputType, OutputType>
{
    fn matvec(&self, _other: &CollectionOfVectors<InputType>) -> CollectionOfVectors<OutputType> {
        println!("I am doing a matvec;");
        CollectionOfVectors::new()
    }
}

fn main() {
    let op_with_matvec = OperatorWithMatVec::<f64, f64>::new().to_box();
    let op_without_matvec = OperatorWithoutMatVec::<f64, f64>::new().to_box();

    let vec = CollectionOfVectors::<f64>::new();
    op_with_matvec.as_ref().as_matvec().unwrap().matvec(&vec);
    op_without_matvec.as_ref().as_matvec().unwrap().matvec(&vec);
}
