#[derive(Debug, PartialEq)]
pub enum Error {
    LexerError(String),
    ParserError(String),
}
