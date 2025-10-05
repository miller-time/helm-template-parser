use crate::error::Error;

pub struct Template {}

pub fn parse_template(_input: &str) -> Result<Template, Error> {
    Err(Error::ParserError("not implemented".into()))
}
