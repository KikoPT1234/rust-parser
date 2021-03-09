use crate::token::TokenType;
use crate::characters::*;
use crate::error::*;

pub type LexResultAll = Result<Vec<TokenType>, LexError>;
pub type LexResult = Result<TokenType, LexError>;

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

    pub fn tokenize(&mut self) -> LexResultAll {
        let mut tokens = vec![];
        loop {
            let token_type;

            match self.current_char {
                Some(current_char) => {
                    if DIGITS.contains(current_char) {
                        token_type = self.make_number()?;
                    } else if LETTERS_AND_DIGITS.contains(current_char) {
                        token_type = self.make_identifier()?;
                    } else {
                        token_type = match current_char {
                            '+' => TokenType::Plus,
                            '-' => TokenType::Minus,
                            '*' => TokenType::Mul,
                            '/' => TokenType::Div,
                            '^' => TokenType::Pow,
                            ';' => TokenType::Semicolon,
                            '"' => self.make_string()?,
                            '=' => self.make_equals()?,
                            '!' => self.make_not_equals()?,
                            '>' => self.make_greater_than()?,
                            '<' => self.make_less_than()?,
                            '|' => self.make_or()?,
                            '&' => self.make_and()?,
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

    fn make_number(&mut self) -> LexResult {
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
            Ok(TokenType::Float(number_string.parse::<f32>().unwrap()))
        } else {
            Ok(TokenType::Int(number_string.parse::<i32>().unwrap()))
        }
    }

    fn make_string(&mut self) -> LexResult {
        let mut string = String::new();
        let mut special = false;

        if let Some(current_char) = self.current_char {
            let character = current_char;
            self.next();
            loop {
                if let Some(current_char) = self.current_char {
                    if current_char == character {
                        break;
                    } else if current_char == '\\' {
                        special = true;
                    } else {
                        if special {
                            special = false;
                            let mut found = false;
                            for i in &SPECIAL_CHARACTERS {
                                if i[0] == current_char {
                                    string.push(i[1]);
                                    found = true;
                                    break;
                                }
                            }
                            if !found {
                                string.push(current_char);
                            }
                        } else {
                            string.push(current_char);
                        }
                    }
                    self.next();
                } else {
                    return Err(LexError::new(String::from("Expected '\"'")))
                }
            }
        };

        Ok(TokenType::Str(string))
    }

    fn make_identifier(&mut self) -> LexResult {
        let mut identifier_string = String::new();

        loop {
            if let Some(current_char) = self.current_char {
                if !LETTERS_AND_DIGITS.contains(current_char) {
                    break;
                }
                identifier_string.push(current_char);
                self.next();
            } else {
                break;
            }
        }

        if KEYWORDS.contains(&identifier_string.as_str()) {
            Ok(TokenType::Keyword(identifier_string))
        } else {
            Ok(TokenType::Identifier(identifier_string))
        }
    }

    fn make_equals(&mut self) -> LexResult {
        self.next();
        
        match self.current_char {
            Some(current_char) => {
                if current_char == '=' {
                    self.next();
                    return Ok(TokenType::EE);
                } else {
                    return Ok(TokenType::Eq);
                }
            },
            None => Err(LexError::new(String::from("Expected '='")))
        }
    }

    fn make_not_equals(&mut self) -> LexResult {
        self.next();

        match self.current_char {
            Some(current_char) => {
                if current_char == '=' {
                    self.next();
                    return Ok(TokenType::NE);
                } else {
                    return Ok(TokenType::Not);
                }
            },
            None => Err(LexError::new(String::from("Expected '='")))
        }
    }

    fn make_greater_than(&mut self) -> LexResult {
        self.next();
        
        match self.current_char {
            Some(current_char) => {
                if current_char == '=' {
                    self.next();
                    Ok(TokenType::GTE)
                } else if current_char == '>' {
                    self.next();
                    Ok(TokenType::BitwiseRightShift)
                } else {
                    Ok(TokenType::GT)
                }
            },
            None => Err(LexError::new(String::from("Expected '>'")))
        }
    }

    fn make_less_than(&mut self) -> LexResult {
        self.next();
        
        match self.current_char {
            Some(current_char) => {
                if current_char == '=' {
                    self.next();
                    Ok(TokenType::LTE)
                } else if current_char == '<' {
                    self.next();
                    Ok(TokenType::BitwiseLeftShift)
                } else {
                    Ok(TokenType::LT)
                }
            },
            None => Err(LexError::new(String::from("Expected '<'")))
        }
    }

    fn make_or(&mut self) -> LexResult {
        self.next();
        
        match self.current_char {
            Some(current_char) => {
                if current_char == '|' {
                    self.next();
                    return Ok(TokenType::Or);
                } else {
                    return Ok(TokenType::BitwiseOr);
                }
            },
            None => Err(LexError::new(String::from("Expected '|'")))
        }
    }

    fn make_and(&mut self) -> LexResult {
        self.next();
        
        match self.current_char {
            Some(current_char) => {
                if current_char == '&' {
                    self.next();
                    return Ok(TokenType::And);
                } else {
                    return Ok(TokenType::BitwiseAnd);
                }
            },
            None => Err(LexError::new(String::from("Expected '&'")))
        }
    }

    fn next(&mut self) {
        self.char_index += 1;
        self.current_char = self.source.chars().nth(self.char_index as usize);
    }
}