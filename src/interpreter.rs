use crate::node::*;
use crate::value::*;
use crate::error::RuntimeError;
use crate::token::TokenType;

use std::collections::HashMap;

pub type RuntimeResult = Result<Value, RuntimeError>;

pub struct Interpreter {}

impl<'a> Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn visit(&self, node: &Node, context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::Statements(_, _) => self.visit_statements_node(node, context),
            Node::Int(_) => self.visit_int_node(node, context),
            Node::Float(_) => self.visit_float_node(node, context),
            Node::Str(_) => self.visit_string_node(node, context),
            Node::UnaryOp(_, _) => self.visit_unary_op_node(node, context),
            Node::BinaryOp(_, _, _) => self.visit_binary_op_node(node, context),
            Node::VarDef(_, _) => self.visit_var_def_node(node, context),
            Node::VarAcc(_) => self.visit_var_acc_node(node, context),
            Node::FuncDef(_, _, _) => self.visit_func_def_node(node, context),
            Node::FuncCall(_, _) => self.visit_func_call_node(node, context),
            Node::Empty => Ok(Value::Null)
        }
    }

    fn visit_statements_node(&self, node: &Node, context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::Statements(nodes, should_return_last) => {
                let mut value = Value::Null;

                for node in nodes {
                    value = self.visit(node, context)?;
                };

                if *should_return_last {
                    Ok(value)
                } else {
                    Ok(Value::Null)
                }
            },
            _ => Err(RuntimeError::new(String::from("Statements expected")))
        }
    }

    fn visit_int_node(&self, node: &Node, _context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::Int(n) => Ok(Value::Int(*n)),
            _ => Err(RuntimeError::new(String::from("Integer expected")))
        }
    }

    fn visit_float_node(&self, node: &Node, _context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::Float(n) => Ok(Value::Float(*n)),
            _ => Err(RuntimeError::new(String::from("Float expected")))
        }
    }

    fn visit_string_node(&self, node: &Node, _context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::Str(string) => Ok(Value::Str(string.as_str().to_string())),
            _ => Err(RuntimeError::new(String::from("String expected")))
        }
    }

    fn visit_unary_op_node(&self, node: &Node, context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::UnaryOp(node, token) => {
                let value = self.visit(node, context)?;
                
                match token {
                    TokenType::Plus => Ok(value.multiply(Value::Int(1))?),
                    TokenType::Minus => Ok(value.multiply(Value::Int(-1))?),
                    TokenType::BitwiseNot => Ok(value.bitwise_not()?),
                    TokenType::Not => Ok(value.logical_not()?),
                    _ => Ok(value)
                }
            },
            _ => Err(RuntimeError::new(String::from("Unary operation expected")))
        }
    }

    fn visit_binary_op_node(&self, node: &Node, context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::BinaryOp(left_node, token, right_node) => {
                let left = self.visit(left_node, context)?;
                let right = self.visit(right_node, context)?;
                match token {
                    &TokenType::Plus => left.add(right),
                    &TokenType::Minus => left.subtract(right),
                    &TokenType::Mul => left.multiply(right),
                    &TokenType::Div => left.divide(right),
                    &TokenType::Pow => left.raise(right),
                    &TokenType::EE => left.equals(right),
                    &TokenType::NE => {
                        let not_equals = left.equals(right)?;
                        
                        match not_equals {
                            Value::Boolean(b) => Ok(Value::Boolean(!b)),
                            right => Err(RuntimeError::new(String::from("Comparing '") + &left.to_string() + "' with '" + &right.to_string() + "' gave a non-boolean value."))
                        }
                    },
                    &TokenType::GT => left.is_greater_than(right),
                    &TokenType::GTE => left.is_greater_than_or_equal_to(right),
                    &TokenType::LT => left.is_less_than(right),
                    &TokenType::LTE => left.is_less_than_or_equal_to(right),
                    &TokenType::BitwiseAnd => left.bitwise_and(right),
                    &TokenType::BitwiseOr => left.bitwise_or(right),
                    &TokenType::BitwiseXOr => left.bitwise_xor(right),
                    &TokenType::BitwiseLeftShift => left.left_shift(right),
                    &TokenType::BitwiseRightShift => left.right_shift(right),
                    &TokenType::And => left.logical_and(right),
                    &TokenType::Or => left.logical_or(right),
                    _ => Err(RuntimeError::new(String::from("Illegal token '") + &token.to_string() + "'"))
                }
            },
            _ => Err(RuntimeError::new(String::from("Binary operation expected")))
        }
    }

    fn visit_var_def_node(&self, node: &Node, context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::VarDef(name, value_node) => {
                let value = self.visit(value_node, context)?;

                context.symbol_table.set(name, value);

                let value = context.symbol_table.get(name).unwrap();

                Ok(value)
            }
            _ => Err(RuntimeError::new(String::from("Var definition expected")))
        }
    }

    fn visit_var_acc_node(&self, node: &Node, context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::VarAcc(name) => {
                match context.symbol_table.get(name) {
                    Some(value) => Ok(value),
                    None => Err(RuntimeError::new(String::from(name) + " is not defined"))
                }
            }
            _ => Err(RuntimeError::new(String::from("Var access expected")))
        }
    }

    fn visit_func_def_node(&self, node: &Node, _context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::FuncDef(name, args, body) => {
                Ok(Value::Func(name.clone(), args.clone(), body.clone()))
            }
            _ => Err(RuntimeError::new(String::from("Func definition expected")))
        }
    }

    fn visit_func_call_node(&self, node: &Node, context: &'a mut Context<'a>) -> RuntimeResult {
        match node {
            Node::FuncCall(func, args) => {
                let function = self.visit(func, context)?;

                match function {
                    Value::Func(name, params, body) => {
                        let new_context = Context::new(Some(context));

                        Ok(Value::Null)
                    },
                    _ => Err(RuntimeError::new(function.to_string() + " is not a function"))
                }
            }
            _ => Err(RuntimeError::new(String::from("Func call expected")))
        }
    }
}

#[derive(Debug)]
pub struct Context<'a> {
    pub parent: Option<&'a mut Context<'a>>,
    pub symbol_table: SymbolTable<'a>
}

impl<'a> Context<'a> {
    pub fn new(parent: Option<&'a mut Context<'a>>) -> Context<'a> {
        Context {
            parent,
            symbol_table: SymbolTable::new()
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable<'a> {
    pub symbols: HashMap<String, Value>,
    context: Option<&'a mut Context<'a>>
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            context: None
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.symbols.insert(String::from(name), value);
    }

    pub fn get(&mut self, name: &str) -> Option<Value> {
        match self.symbols.get(name) {
            Some(value) => Some(value.clone()),
            None => {
                match &mut self.context {
                    Some(context) => {
                        context.symbol_table.get(name)
                    },
                    None => None
                }
            }
        }
    }

    pub fn set_context(&mut self, context: &'a mut Context<'a>) {
        self.context = Some(context);
    }
}