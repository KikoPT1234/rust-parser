use crate::interpreter::RuntimeResult;
use crate::error::RuntimeError;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Str(String),
    Boolean(bool),
    Null
}

use self::Value::*;

impl Value {
    pub fn add(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 + n2)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 + n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 + n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 + n2)),
            (Str(s1), Str(s2)) => Ok(Str(String::from(s1) + &s2)),
            (Str(s), other) => Ok(Str(String::from(s) + &other.to_string())),
            (_, other) => Err(RuntimeError::new(String::from("Operator '+' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string()))
        }
    }

    pub fn subtract(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 - n2)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 - n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 - n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 - n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '-' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string()))
        }
    }

    pub fn multiply(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 * n2)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 * n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 * n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 * n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '*' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string()))
        }
    }

    pub fn divide(&self, other: Value) -> RuntimeResult {
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
            (_, other) => Err(RuntimeError::new(String::from("Operator '/' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string()))
        }
    }

    pub fn raise(&self, power_of: Value) -> RuntimeResult {
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
            (_, other) => Err(RuntimeError::new(String::from("Operator '^' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string()))
        }
    }

    pub fn is_greater_than(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 > &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean(*n1 as f32 > n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 > &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 > &n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '>' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string()))
        }
    }

    pub fn is_greater_than_or_equal_to(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 >= &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean(*n1 as f32 >= n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 >= &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 >= &n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '>' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string()))
        }
    }

    pub fn is_less_than(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 < &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean((*n1 as f32) < n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 < &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 < &n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '>' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string()))
        }
    }

    pub fn is_less_than_or_equal_to(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 <= &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean((*n1 as f32) <= n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 <= &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 <= &n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '>' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string()))
        }
    }

    pub fn equals(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Boolean(n1 == &n2)),
            (Int(n1), Float(n2)) => Ok(Boolean((*n1 as f32) == n2)),
            (Float(n1), Int(n2)) => Ok(Boolean(n1 == &(n2 as f32))),
            (Float(n1), Float(n2)) => Ok(Boolean(n1 == &n2)),
            (Boolean(b1), Boolean(b2)) => Ok(Boolean(b1 == &b2)),
            (Str(s1), Str(s2)) => Ok(Boolean(s1 == &s2)),
            (Null, Null) => Ok(Boolean(true)),
            (_, other) => Err(RuntimeError::new(String::from("Cannot compare '") + &self.to_string() + "' with '" + &other.to_string()))
        }
    }

    pub fn is_true(&self) -> bool {
        match self {
            Int(n) => *n != 0,
            Float(n) => *n != 0.0,
            Boolean(b) => *b,
            Str(s) => s != "",
            Null => false
        }
    }

    pub fn bitwise_and(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 & n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '&' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string() + "'."))
        }
    }

    pub fn bitwise_or(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 | n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '|' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string() + "'."))
        }
    }

    pub fn bitwise_xor(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 ^ n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '^^' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string() + "'."))
        }
    }

    pub fn bitwise_not(&self) -> RuntimeResult {
        match self {
            Int(n) => Ok(Int(!n)),
            _ => Err(RuntimeError::new(String::from("Operator '~' cannot be applied to '") + &self.to_string() + "'."))
        }
    }

    pub fn left_shift(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 << n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '<<' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string() + "'."))
        }
    }

    pub fn right_shift(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 >> n2)),
            (_, other) => Err(RuntimeError::new(String::from("Operator '>>' cannot be applied to '") + &self.to_string() + "', '" + &other.to_string() + "'."))
        }
    }

    pub fn logical_or(&self, other: Value) -> RuntimeResult {
        if self.is_true() {
            Ok(self.clone())
        } else {
            Ok(other)
        }
    }

    pub fn logical_and(&self, other: Value) -> RuntimeResult {
        if self.is_true() {
            Ok(other)
        } else {
            Ok(self.clone())
        }
    }

    pub fn logical_not(&self) -> RuntimeResult {
        Ok(Boolean(!self.is_true()))
    }

    pub fn to_string(&self) -> String {
        match self {
            Int(n) => n.to_string(),
            Float(n) => n.to_string(),
            Boolean(b) => b.to_string(),
            Str(s) => String::from(s),
            Null => String::from("null")
        }
    }
}