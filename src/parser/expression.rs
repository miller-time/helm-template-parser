use crate::{error::Error, stream::TokenStream, token::Token};

pub fn parse_expression(input: &mut TokenStream) -> Result<String, Error> {
    let mut expression = String::new();

    // skip over first '{'
    input.skip();

    // check for second '{'
    if let Some(token) = input.peek() {
        if let Token::LeftBrace = token {
            // and skip over it
            input.skip();
        } else {
            let message = format!("expected '{{{{' but got '{{{}'", token);
            return Err(Error::ParserError(message));
        }
    } else {
        return Err(Error::ParserError(
            "unexpected end of input after '{'".into(),
        ));
    }

    loop {
        match input.next() {
            Some(token) => match token {
                // first '}' indicates end of expression
                Token::RightBrace => {
                    // check for second '}'
                    if let Some(token) = input.peek() {
                        if let Token::RightBrace = token {
                            // and skip over it
                            input.skip();
                            // check for following '\n'
                            if let Some(Token::Newline) = input.peek() {
                                // and skip over it
                                input.skip();
                            }
                            return Ok(expression);
                        } else {
                            let message = format!("expected '}}}}' but got '}}{}'", token);
                            return Err(Error::ParserError(message));
                        }
                    } else {
                        return Err(Error::ParserError(
                            "unexpected end of input after '}'".into(),
                        ));
                    }
                }
                t => {
                    let s = t.to_string();
                    expression.push_str(&s);
                }
            },
            None => return Err(Error::ParserError("unterminated '{{'".into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn parse_valid_expression() {
        let mut stream = tokenize("{{ foo }}").expect("failed to tokenize");
        let expression = parse_expression(&mut stream).expect("failed to parse expression");
        assert_eq!(expression, " foo ");
    }

    #[test]
    fn parse_invalid_expression_start() {
        let mut stream = tokenize("{ foo }").expect("failed to tokenize");
        let result = parse_expression(&mut stream);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            Error::ParserError("expected '{{' but got '{ '".into())
        );
    }

    #[test]
    fn parse_unterminated_expression() {
        let mut stream = tokenize("{{ foo").expect("failed to tokenize");
        let result = parse_expression(&mut stream);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            Error::ParserError("unterminated '{{'".into())
        )
    }

    #[test]
    fn parse_invalid_expression_end() {
        let mut stream = tokenize("{{ foo }\n").expect("failed to tokenize");
        let result = parse_expression(&mut stream);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            Error::ParserError("expected '}}' but got '}\n'".into())
        );
    }
}
