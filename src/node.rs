use crate::token::TokenType;

#[derive(Debug)]
pub enum Node {
    Int(i32),
    Float(f32),
    BinaryOp(Box<Node>, TokenType, Box<Node>),
    UnaryOp(Box<Node>, TokenType),
    Str(String)
}