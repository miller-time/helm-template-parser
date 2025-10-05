#[derive(Debug)]
pub enum Error {
    LexerError(String),
    ParserError(String),
}
