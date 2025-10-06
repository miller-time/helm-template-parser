use crate::{error::Error, stream::TokenStream, token::Token};

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub indentation: u32,
    pub expression: String,
}

/// parse a standalone expression from the input stream
/// preconditions:
///   * aside from spaces, this line must start and end with '{{ }}'
pub fn parse_expression(input: &mut TokenStream) -> Result<Expression, Error> {
    // count indentation
    let mut indentation: u32 = 0;
    loop {
        let token = input.peek().unwrap();
        match token {
            Token::Space => {
                indentation += 1;
                input.skip();
            }
            _ => {
                break;
            }
        }
    }

    // chomp first '{'
    if let Some(Token::LeftBrace) = input.peek() {
        input.skip();
    }
    // chomp second '{'
    if let Some(Token::LeftBrace) = input.peek() {
        input.skip();
    }

    let mut expression = String::new();

    // read expression
    loop {
        let token = input.peek().unwrap();
        match token {
            Token::RightBrace => {
                break;
            }
            _ => {
                let s: String = input.next().unwrap().to_string();
                expression.push_str(&s);
            }
        }
    }

    // chomp first '}'
    if let Some(Token::RightBrace) = input.peek() {
        input.skip();
    }
    // chomp second '}'
    if let Some(Token::RightBrace) = input.peek() {
        input.skip();
    }

    loop {
        match input.peek() {
            Some(token) => {
                match token {
                    // drop newline and return
                    Token::Newline => {
                        input.skip();
                        break;
                    }
                    // drop trailing whitespace
                    Token::Space => {
                        input.skip();
                    }
                    _ => {
                        let message = format!("unexpected symbol '{}' after '}}'", token);
                        return Err(Error::ParserError(message));
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    Ok(Expression {
        indentation,
        expression,
    })
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn parse_valid_expression() {
        let mut stream = tokenize("{{ foo }}").expect("failed to tokenize");
        let expression = parse_expression(&mut stream).expect("failed to parse expression");
        assert_eq!(
            expression,
            Expression {
                indentation: 0,
                expression: " foo ".into()
            }
        );
    }

    #[test]
    fn parse_indented_expression() {
        let mut stream = tokenize("  {{ foo }}").expect("failed to tokenize");
        let expression = parse_expression(&mut stream).expect("failed to parse expression");
        assert_eq!(
            expression,
            Expression {
                indentation: 2,
                expression: " foo ".into()
            }
        );
    }
}
