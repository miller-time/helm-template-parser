use std::collections::VecDeque;

use crate::token::Token;

/// The input text tokenized into a stream the parser can process
#[derive(Default)]
pub struct TokenStream {
    tokens: VecDeque<Token>,
}

impl TokenStream {
    /// create a new stream given some tokens
    pub fn new(tokens: Vec<Token>) -> TokenStream {
        TokenStream {
            tokens: tokens.into(),
        }
    }

    /// return the underlying tokens
    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone().into()
    }

    /// return whether there are no more tokens
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// return `n` upcoming lines as strings
    pub fn peek_lines(&self, n: u32) -> Vec<String> {
        let mut lines_found = 0;
        let mut lines = Vec::new();
        let mut line = String::new();
        for t in &self.tokens {
            if *t == Token::Newline {
                lines_found += 1;
                lines.push(line.clone());
                if lines_found == n {
                    break;
                }
                line = String::new();
            } else {
                let s = t.to_string();
                line.push_str(&s);
            }
        }
        lines.push(line);
        lines
    }

    /// return a copy of the next token, if one exists
    pub fn peek(&self) -> Option<Token> {
        self.tokens.get(0).copied()
    }

    /// remove and return the next token, if one exists
    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    /// remove and skip the next token, if one exists
    pub fn skip(&mut self) {
        let _ = self.tokens.pop_front();
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    #[test]
    fn peek_lines() {
        let stream = tokenize("foo\nbar\nbaz").expect("failed to tokenize");
        let lines = stream.peek_lines(2);
        assert_eq!(lines, vec!["foo", "bar"]);
    }
}
