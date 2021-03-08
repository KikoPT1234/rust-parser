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
        let result = self.statements();

        if self.current_token() != TokenType::EOF {
            Err(ParseError::new(String::from("Unexpected token '") + &self.current_token().to_string() + "'"))
        } else {
            result
        }
    }

    fn statements(&mut self) -> ParseResult {
        let mut nodes = vec![];

        loop {
            let mut should_return = true;
            nodes.push(Box::new(self.expression()?));

            if self.current_token() == TokenType::Semicolon {
                should_return = false;
                self.next();
            }

            if self.current_token() == TokenType::EOF {
                return Ok(Node::Statements(nodes, should_return));
            }
        }
    }

    fn expression(&mut self) -> ParseResult {
        match self.current_token() {
            TokenType::Keyword(string) => {
                if string == "let" {
                    self.next();
                    
                    match self.current_token() {
                        TokenType::Identifier(name) => {
                            self.next();

                            if self.current_token() != TokenType::Eq {
                                return Err(ParseError::new(String::from("Expected '='")))
                            }

                            self.next();

                            let value_node = self.expression()?;

                            Ok(Node::VarDef(name, Box::new(value_node)))
                        },
                        _ => Err(ParseError::new(String::from("Expected identifier")))
                    }
                } else {
                    self.term()
                }
            }
            _ => self.term()
        }
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
            TokenType::Identifier(string) => Ok(Node::VarAcc(string)),
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