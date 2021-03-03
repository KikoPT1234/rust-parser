use crate::token_type::TokenType;

pub struct Parser {
    tokens: Vec<TokenType>
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) {
        Parser {
            tokens
        }
    }
}