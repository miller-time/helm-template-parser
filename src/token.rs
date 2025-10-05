#[derive(Debug, PartialEq)]
pub enum Token {
    Space,
    Letter(char),
    Number(u8),
    Dash,
    Colon,
    Slash,
    Dot,
    DoubleQuote,
    LeftBrace,
    RightBrace,
}
