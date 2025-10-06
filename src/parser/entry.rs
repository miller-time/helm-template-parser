use crate::parser::{array::Array, expression::Expression, map::Map};

#[derive(Debug, PartialEq)]
pub enum Entry {
    Expression(Expression),
    Array(Array),
    Map(Map),
}
