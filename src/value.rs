use crate::interpreter::RuntimeResult;
use crate::error::RuntimeError;
use crate::node::Node;
use crate::ContextManager;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Str(String),
    Boolean(bool),
    Func(String, Vec<String>, Box<Node>, i32),
    List(Vec<Value>),
    Pointer(i32, String),
    Null
}

use self::Value::*;

impl Value {

    pub fn deref<'a>(value: &'a Value, manager: &'a ContextManager) -> Option<&'a Value> {
        match value {
            Value::Pointer(id, name) => {
                manager.get(*id, name)
            },
            _ => Some(value)
        }
    }

    pub fn add(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 + n2)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 + n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 + n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 + n2)),
            (Str(s1), Str(s2)) => Ok(Str(String::from(s1) + &s2)),
            (Str(s), other) => Ok(Str(String::from(s) + &other.to_string(manager))),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().add(other, manager),
            (_, Pointer(_, _)) => self.add(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '+' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn subtract(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 - n2)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 - n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 - n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 - n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().subtract(other, manager),
            (_, Pointer(_, _)) => self.subtract(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '-' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn multiply(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 * n2)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 * n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 * n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 * n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().multiply(other, manager),
            (_, Pointer(_, _)) => self.multiply(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '*' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn divide(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => {
                let r = *n1 as f32 / n2 as f32;
                if r == r.floor() {
                    Ok(Int(r as i32))
                } else {
                    Ok(Float(r))
                }
            },
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 / n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 / n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 / n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().divide(other, manager),
            (_, Pointer(_, _)) => self.divide(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '/' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn raise(&self, power_of: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, power_of) {
            (Int(n1), Int(n2)) => {
                if n2 >= 0 {
                    Ok(Int(n1.pow(n2 as u32)))
                } else {
                    let n1 = *n1 as f32;
                    Ok(Float(n1.powi(n2)))
                }
            },
            (Int(n1), Float(n2)) => Ok(Float((*n1 as f32).powf(n2))),
            (Float(n1), Int(n2)) => Ok(Float(n1.powi(n2))),
            (Float(n1), Float(n2)) => Ok(Float(n1.powf(n2))),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().raise(other, manager),
            (_, Pointer(_, _)) => self.raise(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '^' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn is_greater_than(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 > &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean(*n1 as f32 > n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 > &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 > &n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().is_greater_than(other, manager),
            (_, Pointer(_, _)) => self.is_greater_than(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '>' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn is_greater_than_or_equal_to(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 >= &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean(*n1 as f32 >= n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 >= &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 >= &n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().is_greater_than_or_equal_to(other, manager),
            (_, Pointer(_, _)) => self.is_greater_than_or_equal_to(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '>=' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn is_less_than(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 < &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean((*n1 as f32) < n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 < &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 < &n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().is_less_than(other, manager),
            (_, Pointer(_, _)) => self.is_less_than(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '<' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn is_less_than_or_equal_to(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 <= &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean((*n1 as f32) <= n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 <= &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 <= &n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().is_less_than_or_equal_to(other, manager),
            (_, Pointer(_, _)) => self.is_less_than_or_equal_to(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '<=' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn equals(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 == &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean((*n1 as f32) == n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 == &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 == &n2)),
            (Boolean(b1), Boolean(b2)) => Ok(Boolean(b1 == &b2)),
            (Str(s1), Str(s2)) => Ok(Boolean(s1 == &s2)),
            (Null, Null) => Ok(Boolean(true)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().equals(other, manager),
            (_, Pointer(id, name)) => self.equals(Value::deref(&Value::Pointer(id, name), manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Cannot compare '") + &self.to_string(manager) + "' with '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn is_true(&self, manager: &ContextManager) -> bool {
        match self {
            Int(n) => *n != 0,
            Float(n) => *n != 0.0,
            Boolean(b) => *b,
            Str(s) => s != "",
            Func(..) => true,
            List(vec) => !vec.is_empty(),
            Pointer(_, _) => Value::deref(self, manager).unwrap().is_true(manager),
            Null => false
        }
    }

    pub fn bitwise_and(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 & n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().bitwise_and(other, manager),
            (_, Pointer(_, _)) => self.bitwise_and(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '&' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn bitwise_or(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 | n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().bitwise_or(other, manager),
            (_, Pointer(_, _)) => self.bitwise_or(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '|' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn bitwise_xor(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 ^ n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().bitwise_xor(other, manager),
            (_, Pointer(_, _)) => self.bitwise_xor(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '^^' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn bitwise_not(&self, manager: &ContextManager) -> RuntimeResult {
        match self {
            Int(n) => Ok(Int(!n)),
            Pointer(_, _) => Value::deref(self, manager).unwrap().bitwise_not(manager),
            _ => Err(RuntimeError::new(String::from("Operator '~' cannot be applied to '") + &self.to_string(manager) + "'."))
        }
    }

    pub fn left_shift(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 << n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().left_shift(other, manager),
            (_, Pointer(_, _)) => self.left_shift(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '<<' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn right_shift(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 >> n2)),
            (Pointer(_, _), other) => Value::deref(self, manager).unwrap().right_shift(other, manager),
            (_, Pointer(_, _)) => self.right_shift(Value::deref(self, manager).unwrap().clone(), manager),
            (_, other) => Err(RuntimeError::new(String::from("Operator '>>' cannot be applied to '") + &self.to_string(manager) + "', '" + &other.to_string(manager) + "'."))
        }
    }

    pub fn logical_or(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        if self.is_true(manager) {
            Ok(self.clone())
        } else {
            Ok(other)
        }
    }

    pub fn logical_and(&self, other: Value, manager: &ContextManager) -> RuntimeResult {
        if self.is_true(manager) {
            Ok(other)
        } else {
            Ok(self.clone())
        }
    }

    pub fn logical_not(&self, manager: &ContextManager) -> RuntimeResult {
        Ok(Boolean(!self.is_true(manager)))
    }

    pub fn to_string(&self, manager: &ContextManager) -> String {
        match self {
            Int(n) => n.to_string(),
            Float(n) => n.to_string(),
            Boolean(b) => b.to_string(),
            Str(s) => String::from(s),
            Func(name, args, _, _) => name.clone() + "(" + &args.join(", ") + ")",
            List(vec) => {
                let mut string = String::from("[");

                for item in 0..vec.len() {
                    match &vec.get(item) {
                        Some(val) => {
                            string += &val.to_string(manager);
                            if item < vec.len() - 1 {
                                string += ", "
                            }
                        },
                        None => break
                    }
                }

                string += "]";

                string
            },
            Pointer(context_id, name) => match manager.get(*context_id, name) {
                Some(value) => value.to_string(manager),
                None => String::from("null")
            },
            Null => String::from("null")
        }
    }
}