use crate::token::TokenType;

#[derive(Debug, Clone)]
pub enum Node {
    Int(i32),
    Float(f32),
    Str(String),
    BinaryOp(Box<Node>, TokenType, Box<Node>),
    UnaryOp(Box<Node>, TokenType),
    VarDef(String, Box<Node>),
    VarAcc(String),
    FuncDef(String, Vec<String>, Box<Node>),
    FuncCall(Box<Node>, Vec<Box<Node>>),
    Statements(Vec<Box<Node>>, bool),
    Empty
}