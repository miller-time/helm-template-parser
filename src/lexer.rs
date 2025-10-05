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
        ' ' => Ok(Token::Space),
        '-' => Ok(Token::Dash),
        ':' => Ok(Token::Colon),
        '/' => Ok(Token::Slash),
        '.' => Ok(Token::Dot),
        '"' => Ok(Token::DoubleQuote),
        '{' => Ok(Token::LeftBrace),
        '}' => Ok(Token::RightBrace),
        _ => Err(Error::LexerError(format!("unrecognized input '{}'", c))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_token() {
        let tokens = tokenize(" -:/.\"{}").expect("failed to tokenize");
        assert_eq!(
            tokens,
            vec![
                Token::Space,
                Token::Dash,
                Token::Colon,
                Token::Slash,
                Token::Dot,
                Token::DoubleQuote,
                Token::LeftBrace,
                Token::RightBrace
            ]
        );
    }
}
