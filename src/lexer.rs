use crate::token::TokenType;
use crate::characters::*;
use crate::error::*;
// use regex::Regex;

pub struct Lexer {
    source: String,    
    current_char: Option<char>,
    char_index: i32
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        let mut lexer = Lexer {
            source: String::from(source),
            current_char: None,
            char_index: -1
        };
        lexer.next();
        lexer
    }

    pub fn tokenize(&mut self) -> Result<Vec<TokenType>, LexError> {
        let mut tokens = vec![];
        loop {
            let token_type;

            match self.current_char {
                Some(current_char) => {
                    if DIGITS.contains(current_char) {
                        token_type = self.make_number();
                    } else {
                        token_type = match current_char {
                            '+' => TokenType::Plus,
                            '-' => TokenType::Minus,
                            '*' => TokenType::Mul,
                            '/' => TokenType::Div,
                            '^' => TokenType::Pow,
                            ' ' => {
                                self.next();
                                continue;
                            },
                            '\t' => {
                                self.next();
                                continue;
                            },
                            '\n' => {
                                self.next();
                                continue;
                            },
                            '\r' => {
                                self.next();
                                continue;
                            },
                            _ => {
                                let mut error = String::from("Unknown character '");
                                error.push(current_char);
                                error.push_str("'.");
                                return Err(LexError::new(error));
                            }
                        };
                        self.next();
                    }
                },
                None => {
                    token_type = TokenType::EOF
                }
            };

            match token_type {
                TokenType::EOF => {
                    tokens.push(token_type);
                    break;
                },
                _ => tokens.push(token_type)
            };
        }
        Ok(tokens)
    }

    fn next(&mut self) {
        self.char_index += 1;
        self.current_char = self.source.chars().nth(self.char_index as usize);
    }

    fn make_number(&mut self) -> TokenType {
        let mut number_string = String::new();
        let mut has_point = false;
        loop {
            if let Some(current_char) = self.current_char {
                if !DIGITS.contains(current_char) && current_char != '.' {
                    break;
                }
                if current_char == '.' {
                    if has_point {
                        break;
                    }
                    has_point = true;
                }
                number_string.push(current_char);
                self.next();
            } else {
                break;
            }
        }
        if has_point {
            TokenType::Float(number_string.parse::<f32>().unwrap())
        } else {
            TokenType::Int(number_string.parse::<i32>().unwrap())
        }
    }
}