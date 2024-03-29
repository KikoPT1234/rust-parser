use crate::node::*;
use crate::value::*;
use crate::error::RuntimeError;
use crate::token::TokenType;
use crate::ContextManager;

pub type RuntimeResult = Result<Value, RuntimeError>;

pub struct Interpreter<'a> {
    manager: &'a mut ContextManager
}

impl<'a> Interpreter<'a> {
    pub fn new(manager: &'a mut ContextManager) -> Interpreter {
        Interpreter {
            manager
        }
    }

    pub fn visit(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::Statements(..) => self.visit_statements_node(node, context_id),
            Node::Int(..) => self.visit_int_node(node, context_id),
            Node::Float(..) => self.visit_float_node(node, context_id),
            Node::Str(..) => self.visit_string_node(node, context_id),
            Node::UnaryOp(..) => self.visit_unary_op_node(node, context_id),
            Node::BinaryOp(..) => self.visit_binary_op_node(node, context_id),
            Node::VarDef(..) => self.visit_var_def_node(node, context_id),
            Node::VarAcc(..) => self.visit_var_acc_node(node, context_id),
            Node::ListDef(..) => self.visit_list_def_node(node, context_id),
            Node::FuncDef(..) => self.visit_func_def_node(node, context_id),
            Node::FuncCall(..) => self.visit_func_call_node(node, context_id),
            Node::If(..) => self.visit_if_node(node, context_id),
            Node::WhileLoop(..) => self.visit_while_loop_node(node, context_id),
            _ => Ok(Value::Null)
        }
    }

    fn visit_statements_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::Statements(nodes, should_return_last) => {
                let mut value = Value::Null;

                for node in nodes {
                    value = self.visit(node, context_id)?;
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

    fn visit_int_node(&self, node: &Node, _context_id: i32) -> RuntimeResult {
        match node {
            Node::Int(n) => Ok(Value::Int(*n)),
            _ => Err(RuntimeError::new(String::from("Integer expected")))
        }
    }

    fn visit_float_node(&self, node: &Node, _context_id: i32) -> RuntimeResult {
        match node {
            Node::Float(n) => Ok(Value::Float(*n)),
            _ => Err(RuntimeError::new(String::from("Float expected")))
        }
    }

    fn visit_string_node(&self, node: &Node, _context_id: i32) -> RuntimeResult {
        match node {
            Node::Str(string) => Ok(Value::Str(string.as_str().to_string())),
            _ => Err(RuntimeError::new(String::from("String expected")))
        }
    }

    fn visit_unary_op_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::UnaryOp(node, token) => {
                let value = self.visit(node, context_id)?;
                
                let result = match token {
                    TokenType::Plus => value.multiply(Value::Int(1), &self.manager),
                    TokenType::Minus => value.multiply(Value::Int(-1), &self.manager),
                    TokenType::BitwiseNot => value.bitwise_not(&self.manager),
                    TokenType::Not => value.logical_not(&self.manager),
                    _ => RuntimeResult::Ok(value)
                };

                match result {
                    RuntimeResult::Ok(value) => RuntimeResult::Ok(value),
                    RuntimeResult::Err(err) => RuntimeResult::Err(err)
                }
            },
            _ => Err(RuntimeError::new(String::from("Unary operation expected")))
        }
    }

    fn visit_binary_op_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::BinaryOp(left_node, token, right_node) => {
                let left = self.visit(left_node, context_id)?;
                let right = self.visit(right_node, context_id)?;
                let result = match token {
                    &TokenType::Plus => left.add(right, &self.manager),
                    &TokenType::Minus => left.subtract(right, &self.manager),
                    &TokenType::Mul => left.multiply(right, &self.manager),
                    &TokenType::Div => left.divide(right, &self.manager),
                    &TokenType::Pow => left.raise(right, &self.manager),
                    &TokenType::EE => left.equals(right, &self.manager),
                    &TokenType::NE => {
                        let not_equals = left.equals(right, &self.manager)?;

                        match not_equals {
                            Value::Boolean(b) => Ok(Value::Boolean(!b)),
                            right => Err(RuntimeError::new(String::from("Comparing '") + &left.to_string(&self.manager) + "' with '" + &right.to_string(&self.manager) + "' gave a non-boolean value."))
                        }
                    },
                    &TokenType::GT => left.is_greater_than(right, &self.manager),
                    &TokenType::GTE => left.is_greater_than_or_equal_to(right, &self.manager),
                    &TokenType::LT => left.is_less_than(right, &self.manager),
                    &TokenType::LTE => left.is_less_than_or_equal_to(right, &self.manager),
                    &TokenType::BitwiseAnd => left.bitwise_and(right, &self.manager),
                    &TokenType::BitwiseOr => left.bitwise_or(right, &self.manager),
                    &TokenType::BitwiseXOr => left.bitwise_xor(right, &self.manager),
                    &TokenType::BitwiseLeftShift => left.left_shift(right, &self.manager),
                    &TokenType::BitwiseRightShift => left.right_shift(right, &self.manager),
                    &TokenType::And => left.logical_and(right, &self.manager),
                    &TokenType::Or => left.logical_or(right, &self.manager),
                    _ => Err(RuntimeError::new(String::from("Illegal token '") + &token.to_string() + "'"))
                };

                match result {
                    RuntimeResult::Ok(value) => RuntimeResult::Ok(value),
                    RuntimeResult::Err(err) => RuntimeResult::Err(err)
                }
            },
            _ => Err(RuntimeError::new(String::from("Binary operation expected")))
        }
    }

    fn visit_var_def_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::VarDef(name, value_node) => {
                let value = self.visit(value_node, context_id)?;

                self.manager.set(context_id, name, value);

                Ok(Value::Pointer(context_id, name.to_string()))
            }
            _ => Err(RuntimeError::new(String::from("Var definition expected")))
        }
    }

    fn visit_var_acc_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::VarAcc(name) => {
                match self.manager.get(context_id, name) {
                    Some(value) => {
                        match value {
                            Value::List(..) => Ok(Value::Pointer(context_id, name.to_string())),
                            Value::Func(..) => Ok(Value::Pointer(context_id, name.to_string())),
                            Value::Str(..) => Ok(Value::Pointer(context_id, name.to_string())),
                            _ => Ok(value.clone())
                        }
                    },
                    None => Err(RuntimeError::new(String::from(name) + " is not defined"))
                }
            }
            _ => Err(RuntimeError::new(String::from("Var access expected")))
        }
    }

    fn visit_list_def_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::ListDef(nodes) => {
                let mut values = vec![];

                for node in nodes {
                    values.push(self.visit(node, context_id)?);
                }

                Ok(Value::List(values))
            }
            _ => Err(RuntimeError::new(String::from("List def expected")))
        }
    }

    fn visit_func_def_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::FuncDef(name, args, body) => {

                let value = Value::Func(name.clone(), args.clone(), body.clone(), self.manager.create_context(Some(context_id)));

                self.manager.set(context_id, name, value.clone());

                Ok(value)
            }
            _ => Err(RuntimeError::new(String::from("Func definition expected")))
        }
    }

    fn visit_func_call_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::FuncCall(func, args) => {
                let mut function = self.visit(func, context_id)?;

                loop {
                    match function {
                        Value::Func(_, params, body, func_context) => {
                            let arg_context = self.manager.create_context(Some(func_context));
                            // arg_context.set_parent(context);

                            let call_context = self.manager.create_context(Some(func_context));
                            // call_context.set_parent(func_context.get_mut());

                            if params.len() > 0 {
                                for i in 0..params.len() {
                                    match args.get(i) {
                                        Some(arg) => {
                                            let arg_value = self.visit(arg, arg_context)?;
                                            self.manager.set(call_context, &params[i], arg_value);
                                        },
                                        None => {
                                            self.manager.set(call_context, &params[i], Value::Null);
                                        }
                                    }
                                }
                            }

                            break self.visit(&body, call_context);
                        },
                        Value::Pointer(_, _) => {
                            match Value::deref(&function, self.manager) {
                                Some(value) => {
                                    function = value.clone()
                                },
                                None => {
                                    break Err(RuntimeError::new(function.to_string(&self.manager) + " is not a function"))
                                }
                            };
                        },
                        _ => {
                            break Err(RuntimeError::new(function.to_string(&self.manager) + " is not a function"));
                        }
                    }
                }
            }
            _ => Err(RuntimeError::new(String::from("Func call expected")))
        }
    }

    fn visit_if_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::If(condition, body, else_body) => {
                let condition_value = self.visit(condition, context_id)?;

                if condition_value.is_true(self.manager) {
                    let if_context = self.manager.create_context(Some(context_id));

                    self.visit(body, if_context)
                } else {
                    match else_body {
                        Some(else_node) => {
                            let else_context = self.manager.create_context(Some(context_id));

                            self.visit(else_node, else_context)
                        }
                        None => Ok(Value::Null)
                    }
                }
            },
            _ => Err(RuntimeError::new(String::from("If expected")))
        }
    }

    fn visit_while_loop_node(&mut self, node: &Node, context_id: i32) -> RuntimeResult {
        match node {
            Node::WhileLoop(condition, body) => {
                let while_context = self.manager.create_context(Some(context_id));

                let mut result_value = Value::Null;

                loop {
                    let condition_value = self.visit(condition, context_id)?;

                    if !condition_value.is_true(self.manager) {
                        break;
                    }

                    result_value = self.visit(body, while_context)?;
                }

                Ok(result_value)
            },
            _ => Err(RuntimeError::new(String::from("While loop expected")))
        }
    }
}

