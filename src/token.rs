use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Int(i32),
    Float(f32),
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    Eq,
    EE,
    NE,
    GT,
    GTE,
    LT,
    LTE,
    Not,
    And,
    Or,
    BitwiseNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXOr,
    BitwiseRightShift,
    BitwiseLeftShift,
    Keyword(String),
    Identifier(String),
    Str(String),
    Semicolon,
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftSquare,
    RightSquare,
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
            TokenType::Pow => String::from("^"),
            TokenType::Eq => String::from("="),
            TokenType::EE => String::from("=="),
            TokenType::NE => String::from("!="),
            TokenType::GT => String::from(">"),
            TokenType::GTE => String::from(">="),
            TokenType::LT => String::from("<"),
            TokenType::LTE => String::from("<="),
            TokenType::BitwiseNot => String::from("~"),
            TokenType::BitwiseAnd => String::from("&"),
            TokenType::BitwiseOr => String::from("|"),
            TokenType::BitwiseXOr => String::from("^^"),
            TokenType::BitwiseLeftShift => String::from("<<"),
            TokenType::BitwiseRightShift => String::from(">>"),
            TokenType::Not => String::from("!"),
            TokenType::And => String::from("&&"),
            TokenType::Or => String::from("||"),
            TokenType::Semicolon => String::from(";"),
            TokenType::Comma => String::from(","),
            TokenType::LeftParen => String::from("("),
            TokenType::RightParen => String::from(")"),
            TokenType::LeftSquare => String::from("["),
            TokenType::RightSquare => String::from("]"),
            TokenType::LeftBracket => String::from("{"),
            TokenType::RightBracket => String::from("}"),
            TokenType::Keyword(string) => string.clone(),
            TokenType::Identifier(string) => string.clone(),
            TokenType::Str(string) => String::from("\"") + string + "\"",
            _ => string
        };

        string
    }
}