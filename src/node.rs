use crate::token::TokenType;

#[derive(Debug)]
pub enum Node {
    Int(i32),
    Float(f32),
    Str(String),
    BinaryOp(Box<Node>, TokenType, Box<Node>),
    UnaryOp(Box<Node>, TokenType),
    VarDef(String, Box<Node>),
    VarAcc(String),
    Statements(Vec<Box<Node>>, bool),
    Empty
}