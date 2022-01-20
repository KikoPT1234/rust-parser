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
        let result = self.statements(false)?;

        if self.current_token() != TokenType::EOF {
            Err(ParseError::new(String::from("Unexpected token '") + &self.current_token().to_string() + "'"))
        } else {
            Ok(result)
        }
    }

    fn statements(&mut self, contained: bool) -> ParseResult {
        let mut nodes = vec![];

        loop {

            if self.current_token() == TokenType::RightBracket && contained {
                return Ok(Node::Empty);
            }

            nodes.push(Box::new(self.expression()?));

            if self.current_token() == TokenType::Semicolon {
                
                while self.current_token() == TokenType::Semicolon {
                    self.next();
                }

                if self.current_token() == TokenType::EOF/* || (self.current_token() == TokenType::RightBracket && contained)*/ {
                    return Ok(Node::Statements(nodes, false));
                }
            } else {
                return Ok(Node::Statements(nodes, true));
            }
        }
    }

    fn expression(&mut self) -> ParseResult {
        match self.current_token() {
            TokenType::Keyword(string) => {
                if string == "let" {
                    self.var_def()
                } else if string == "function" {
                    self.function_def()
                } else if string == "if" {
                    self.if_expression()
                } else if string == "while" {
                    self.while_expression()
                } else {
                    self.logical_bitwise_comparison()
                }
            }
            _ => self.logical_bitwise_comparison()
        }
    }

    fn logical_bitwise_comparison(&mut self) -> ParseResult {
        self.binary_operation(&mut |this: &mut Self| this.numeric_comparison(), &[TokenType::BitwiseAnd, TokenType::BitwiseOr, TokenType::BitwiseXOr, TokenType::And, TokenType::Or], false)
    }

    fn numeric_comparison(&mut self) -> ParseResult {
        self.binary_operation(&mut |this: &mut Self| this.bitwise_shifting(), &[TokenType::EE, TokenType::NE, TokenType::GT, TokenType::GTE, TokenType::LT, TokenType::LTE], false)
    }

    fn bitwise_shifting(&mut self) -> ParseResult {
        self.binary_operation(&mut |this: &mut Self| this.not(), &[TokenType::BitwiseRightShift, TokenType::BitwiseLeftShift], false)
    }
    
    fn not(&mut self) -> ParseResult {
        if self.current_token() == TokenType::Not || self.current_token() == TokenType::BitwiseNot {
            let op_token = self.current_token();
            
            self.next();

            let node = self.not()?;

            return match node {
                Node::Empty => Err(ParseError::new(String::from("Unexpected end of file."))),
                _ => Ok(Node::UnaryOp(Box::new(node), op_token))
            }
        }

        self.term()
    }

    fn term(&mut self) -> ParseResult {
        self.binary_operation(&mut |this: &mut Self| this.factor(), &[TokenType::Plus, TokenType::Minus], false)
    }

    fn factor(&mut self) -> ParseResult {
        self.binary_operation(&mut |this: &mut Self| this.power(), &[TokenType::Mul, TokenType::Div], false)
    }

    fn power(&mut self) -> ParseResult {
        self.binary_operation(&mut |this: &mut Self| this.unary(), &[TokenType::Pow], true)
    }

    fn unary(&mut self) -> ParseResult {
        let current_token = self.current_token();
        if current_token == TokenType::Plus || current_token == TokenType::Minus {
            self.next();

            let node = self.call()?;

            return match node {
                Node::Empty => Err(ParseError::new(String::from("Unexpected end of file."))),
                _ => Ok(Node::UnaryOp(Box::new(node), current_token))
            }
        }

        self.call()
    }

    fn call(&mut self) -> ParseResult {
        let node = self.listing()?;

        if self.current_token() == TokenType::LeftParen {
            self.next();

            let mut args = vec![];

            while self.current_token() != TokenType::RightParen {
                args.push(Box::new(self.expression()?));

                if self.current_token() != TokenType::Comma && self.current_token() != TokenType::RightParen {
                    return Err(ParseError::new(String::from("Expected ',' or ')'")));
                }

                if self.current_token() != TokenType::RightParen {
                    self.next();
                }
            }

            self.next();

            return Ok(Node::FuncCall(Box::new(node), args));
        }

        Ok(node)
    }

    fn listing(&mut self) -> ParseResult {
        if self.current_token() == TokenType::LeftSquare {
            let mut list_nodes = vec![];
            self.next();

            while self.current_token() != TokenType::RightSquare {
                list_nodes.push(Box::new(self.expression()?));

                if self.current_token() != TokenType::Comma && self.current_token() != TokenType::RightSquare {
                    return Err(ParseError::new(String::from("Expected ',' or ']'")));
                }

                if self.current_token() != TokenType::RightSquare {
                    self.next();
                }
            }

            self.next();

            return Ok(Node::ListDef(list_nodes));
        }

        self.grouping()
    }

    fn grouping(&mut self) -> ParseResult {
        if self.current_token() == TokenType::LeftParen {
            self.next();

            let expression = self.expression()?;

            if self.current_token() != TokenType::RightParen {
                return Err(ParseError::new(String::from("Expected ')'")));
            }

            self.next();

            return Ok(expression);
        }

        self.atom()
    }

    fn atom(&mut self) -> ParseResult {
        let result = match self.current_token() {
            TokenType::Int(number) => Ok(Node::Int(number)),
            TokenType::Float(number) => Ok(Node::Float(number)),
            TokenType::Str(string) => Ok(Node::Str(string)),
            TokenType::Identifier(string) => Ok(Node::VarAcc(string)),
            TokenType::EOF => Ok(Node::EOF),
            _ => Err(ParseError::new(String::from("Unexpected token '") + &self.current_token().to_string() + "'"))
        };
        self.next();
        result
    }

    pub fn binary_operation<T: FnMut(&mut Self) -> ParseResult>(&mut self, func: &mut T, token_types: &[TokenType], right_to_left: bool) -> ParseResult {
        let mut left = func(self)?;

        while token_types.contains(&self.current_token()) {
            let op_token = self.current_token();

            self.next();

            let right;

            if right_to_left {
                right = self.binary_operation(func, token_types, right_to_left)?;
            } else {          
                right = func(self)?;
            }

            left = Node::BinaryOp(Box::new(left), op_token, Box::new(right));
        }

        Ok(left)
    }

    fn var_def(&mut self) -> ParseResult {
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
    }

    fn function_def(&mut self) -> ParseResult {
        self.next();

        match self.current_token() {
            TokenType::Identifier(function_name) => {
                self.next();

                if self.current_token() != TokenType::LeftParen {
                    return Err(ParseError::new(String::from("Expected '('")));
                }

                self.next();

                let mut args = vec![];

                while self.current_token() != TokenType::RightParen {
                    match self.current_token() {
                        TokenType::Identifier(arg) => {
                            args.push(arg);

                            self.next();

                            if self.current_token() != TokenType::Comma && self.current_token() != TokenType::RightParen {
                                return Err(ParseError::new(String::from("Expected ',' or ')'")));
                            }

                            if self.current_token() != TokenType::RightParen {
                                self.next();
                            }
                        },
                        _ => return Err(ParseError::new(String::from("Identifier expected")))
                    }
                }

                self.next();

                let statements = self.block()?;

                Ok(Node::FuncDef(function_name, args, Box::new(statements)))
            },
            _ => Err(ParseError::new(String::from("Expected identifier")))
        }
    }

    fn if_expression(&mut self) -> ParseResult {
        self.next();

        if self.current_token() != TokenType::LeftParen {
            return Err(ParseError::new(String::from("Expected '('")))
        }

        let condition = self.grouping()?;

        let body;

        if self.current_token() == TokenType::LeftBracket {
            body = self.block()?;
        } else {
            body = self.expression()?;

            match body {
                Node::EOF => {
                    return Err(ParseError::new(String::from("Unexpected end of file")))
                },
                _ => {}
            }
        }

        match self.current_token() {
            TokenType::Keyword(string) => {
                if string == "else" {
                    self.next();

                    let else_body;

                    if self.current_token() == TokenType::LeftBracket {
                        else_body = self.block()?;
                    } else {
                        else_body = self.expression()?;
            
                        match else_body {
                            Node::EOF => {
                                return Err(ParseError::new(String::from("Unexpected end of file")))
                            },
                            _ => {}
                        }
                    }

                    Ok(Node::If(Box::new(condition), Box::new(body), Some(Box::new(else_body))))
                } else {
                    Ok(Node::If(Box::new(condition), Box::new(body), None))
                }
            },
            _ => Ok(Node::If(Box::new(condition), Box::new(body), None))
        }
    }

    fn while_expression(&mut self) -> ParseResult {
        self.next();

        if self.current_token() != TokenType::LeftParen {
            return Err(ParseError::new(String::from("Expected '('")));
        }

        let condition = self.grouping()?;

        let body;

        if self.current_token() == TokenType::LeftBracket {
            body = self.block()?;
        } else {
            body = self.expression()?;

            match body {
                Node::EOF => {
                    return Err(ParseError::new(String::from("Unexpected end of file")))
                },
                _ => {}
            }
        }

        Ok(Node::WhileLoop(Box::new(condition), Box::new(body)))
    }

    fn block(&mut self) -> ParseResult {
        if self.current_token() != TokenType::LeftBracket {
            return Err(ParseError::new(String::from("Expected '{'")));
        }

        self.next();

        let statements = self.statements(true)?;

        if self.current_token() != TokenType::RightBracket {
            return Err(ParseError::new(String::from("Expected '}'")))
        }

        self.next();

        Ok(statements)
    }

    fn next(&mut self) -> TokenType {
        if self.token_index + 1 < self.tokens.len() {
            self.token_index += 1;
        }
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