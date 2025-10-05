use crate::{error::Error, token::Token};

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();

    let mut iter = input.chars();
    loop {
        match iter.next() {
            Some(c) => {
                let token = to_token(c)?;
                tokens.push(token);
            }
            None => break,
        }
    }

    Ok(tokens)
}

fn to_token(c: char) -> Result<Token, Error> {
    match c {
        letter if letter.is_alphabetic() => Ok(Token::Letter(c)),
        number if number.is_digit(10) => match number.to_digit(10) {
            Some(digit) => Ok(Token::Number(digit)),
            None => Err(Error::LexerError(format!(
                "failed to convert {} to digit",
                c
            ))),
        },
        ' ' => Ok(Token::Space),
        '-' => Ok(Token::Dash),
        ':' => Ok(Token::Colon),
        '/' => Ok(Token::Slash),
        '|' => Ok(Token::Pipe),
        '.' => Ok(Token::Dot),
        ',' => Ok(Token::Comma),
        '\'' => Ok(Token::SingleQuote),
        '"' => Ok(Token::DoubleQuote),
        '*' => Ok(Token::Asterisk),
        '=' => Ok(Token::Equals),
        '%' => Ok(Token::Percent),
        '\n' => Ok(Token::Newline),
        '(' => Ok(Token::LeftParen),
        ')' => Ok(Token::RightParen),
        '[' => Ok(Token::LeftBracket),
        ']' => Ok(Token::RightBracket),
        '{' => Ok(Token::LeftBrace),
        '}' => Ok(Token::RightBrace),
        '$' => Ok(Token::Dollar),
        _ => Err(Error::LexerError(format!("unrecognized input '{}'", c))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_token() {
        let tokens = tokenize("a1 -:/|.,'\"*=%\n()[]{}$").expect("failed to tokenize");
        assert_eq!(
            tokens,
            vec![
                Token::Letter('a'),
                Token::Number(1),
                Token::Space,
                Token::Dash,
                Token::Colon,
                Token::Slash,
                Token::Pipe,
                Token::Dot,
                Token::Comma,
                Token::SingleQuote,
                Token::DoubleQuote,
                Token::Asterisk,
                Token::Equals,
                Token::Percent,
                Token::Newline,
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBracket,
                Token::RightBracket,
                Token::LeftBrace,
                Token::RightBrace,
                Token::Dollar,
            ]
        );
    }
}
