#[derive(Debug)]
pub enum TokenType {
    Int(i32),
    Float(f32),
    Plus,
    Minus,
    Mul,
    Div,
    EOF
}