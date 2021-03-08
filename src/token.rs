use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Int(i32),
    Float(f32),
    Plus,
    Minus,
    Mul,
    Div,
    EOF
}

impl TokenType {
    pub fn to_string(&self) -> String {
        let mut string = format!("{:?}", self);
        
        string = match self {
            TokenType::Int(n) => n.to_string(),
            TokenType::Float(n) => n.to_string(),
            TokenType::Plus => String::from("+"),
            TokenType::Minus => String::from("-"),
            TokenType::Mul => String::from("*"),
            TokenType::Div => String::from("/"),
            _ => string
        };

        string
    }
}