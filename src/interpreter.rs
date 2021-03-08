use crate::node::*;
use crate::value::*;
use crate::error::RuntimeError;
use crate::token::TokenType;

use std::collections::HashMap;

pub type RuntimeResult = Result<Value, RuntimeError>;

pub struct Interpreter {
    node: Node
}

impl Interpreter {
    pub fn new(node: Node) -> Interpreter {
        Interpreter {
            node
        }
    }

    pub fn execute(&self) -> RuntimeResult {
        let context = Context::new(None);

        self.visit(&self.node, &context)
    }

    fn visit(&self, node: &Node, context: &Context) -> RuntimeResult {
        match node {
            Node::Int(_) => self.visit_int_node(node, context),
            Node::Float(_) => self.visit_float_node(node, context),
            Node::BinaryOp(_, _, _) => self.visit_binary_op_node(node, context)
        }
    }

    fn visit_int_node(&self, node: &Node, context: &Context) -> RuntimeResult {
        match node {
            Node::Int(n) => Ok(Value::Int(*n)),
            _ => Err(RuntimeError::new(String::from("Integer expected")))
        }
    }

    fn visit_float_node(&self, node: &Node, context: &Context) -> RuntimeResult {
        match node {
            Node::Float(n) => Ok(Value::Float(*n)),
            _ => Err(RuntimeError::new(String::from("Float expected")))
        }
    }

    fn visit_binary_op_node(&self, node: &Node, context: &Context) -> RuntimeResult {
        match node {
            Node::BinaryOp(left_node, token, right_node) => {
                let left = self.visit(left_node, context)?;
                let right = self.visit(right_node, context)?;
                match token {
                    &TokenType::Plus => left.add(right),
                    &TokenType::Minus => left.subtract(right),
                    &TokenType::Mul => left.multiply(right),
                    &TokenType::Div => left.divide(right),
                    _ => Err(RuntimeError::new(String::from("Illegal token '") + &token.to_string() + ")"))
                }
            },
            _ => Err(RuntimeError::new(String::from("Binary operation expected")))
        }
    }
}

pub struct Context<'a> {
    parent: Option<&'a Context<'a>>,
    symbolTable: SymbolTable
}

impl<'a> Context<'a> {
    pub fn new(parent: Option<&'a Context>) -> Context<'a> {
        Context {
            parent,
            symbolTable: SymbolTable::new()
        }
    }
}

pub struct SymbolTable {
    symbols: HashMap<String, Value>
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new()
        }
    }
}