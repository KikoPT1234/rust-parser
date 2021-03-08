use crate::token::TokenType;
use crate::node::*;
use crate::error::ParseError;

type ParseResult = Result<Node, ParseError>;

pub struct Parser {
    tokens: Vec<TokenType>,
    token_index: usize
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Parser {
        Parser {
            tokens,
            token_index: 0
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        let result = self.expression();

        if self.current_token() != TokenType::EOF {
            Err(ParseError::new(String::from("Unexpected token '") + &self.current_token().to_string() + "'"))
        } else {
            result
        }
    }

    fn expression(&mut self) -> ParseResult {
        self.term()
    }

    fn term(&mut self) -> ParseResult {
        self.binary_operation(&mut |this: &mut Self| this.factor(), &[TokenType::Plus, TokenType::Minus])
    }

    fn factor(&mut self) -> ParseResult {
        self.binary_operation(&mut |this: &mut Self| this.power(), &[TokenType::Mul, TokenType::Div])
    }

    fn power(&mut self) -> ParseResult {
        self.binary_operation(&mut |this: &mut Self| this.unary(), &[TokenType::Pow])
    }

    fn unary(&mut self) -> ParseResult {
        let current_token = self.current_token();
        if current_token == TokenType::Plus || current_token == TokenType::Minus {
            self.next();

            let node = self.atom()?;

            return Ok(Node::UnaryOp(Box::new(node), current_token));
        }

        self.atom()
    }

    fn atom(&mut self) -> ParseResult {
        let result: ParseResult = match self.current_token() {
            TokenType::Int(number) => Ok(Node::Int(number)),
            TokenType::Float(number) => Ok(Node::Float(number)),
            TokenType::Str(string) => Ok(Node::Str(string)),
            _ => Err(ParseError::new(String::from("Unexpected token '") + &self.current_token().to_string() + "'"))
        };
        self.next();
        result
    }

    pub fn binary_operation<T: FnMut(&mut Self) -> ParseResult>(&mut self, func: &mut T, token_types: &[TokenType]) -> ParseResult {
        let mut left = func(self)?;

        while token_types.contains(&self.current_token()) {
            let op_token = self.current_token();

            self.next();

            let right = func(self)?;

            left = Node::BinaryOp(Box::new(left), op_token, Box::new(right));
        }

        Ok(left)
    }

    fn next(&mut self) -> TokenType {
        self.token_index += 1;
        self.tokens[self.token_index].clone()
    }

    // fn previous(&mut self, count: usize) -> TokenType {
    //     self.token_index -= count;
    //     self.tokens[self.token_index].clone()
    // }

    pub fn current_token(&mut self) -> TokenType {
        self.tokens[self.token_index].clone()
    }
}