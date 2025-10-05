use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Space,
    Letter(char),
    Number(u32),
    Dash,
    Colon,
    Slash,
    Pipe,
    Dot,
    Comma,
    SingleQuote,
    DoubleQuote,
    Asterisk,
    Equals,
    Percent,
    Newline,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Dollar,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Token::Space => " ".into(),
            Token::Letter(c) => c.to_string(),
            Token::Number(n) => n.to_string(),
            Token::Dash => "-".into(),
            Token::Colon => ":".into(),
            Token::Slash => "/".into(),
            Token::Pipe => "|".into(),
            Token::Dot => ".".into(),
            Token::Comma => ",".into(),
            Token::SingleQuote => "'".into(),
            Token::DoubleQuote => "\"".into(),
            Token::Asterisk => "*".into(),
            Token::Equals => "=".into(),
            Token::Percent => "%".into(),
            Token::Newline => "\n".into(),
            Token::LeftParen => "(".into(),
            Token::RightParen => ")".into(),
            Token::LeftBracket => "[".into(),
            Token::RightBracket => "]".into(),
            Token::LeftBrace => "{".into(),
            Token::RightBrace => "}".into(),
            Token::Dollar => "$".into(),
        };
        write!(f, "{}", s)
    }
}
