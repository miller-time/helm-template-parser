use std::collections::VecDeque;

use crate::token::Token;

#[derive(Default)]
pub struct TokenStream {
    tokens: VecDeque<Token>,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> TokenStream {
        TokenStream {
            tokens: tokens.into(),
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone().into()
    }

    pub fn peek(&self) -> Option<Token> {
        self.tokens.get(0).copied()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}
