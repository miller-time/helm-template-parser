use crate::parser::entry::Entry;

#[derive(Debug, Default, PartialEq)]
pub struct Map {
    pub indentation: u32,
    pub items: Vec<Entry>,
}
