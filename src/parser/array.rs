use crate::parser::entry::Entry;

#[derive(Debug, PartialEq)]
pub struct Array {
    pub indentation: u32,
    pub items: Vec<Entry>,
}
