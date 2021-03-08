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
            _ => Err(RuntimeError::new(String::from("Illegal operation ADD")))
        }
    }

    pub fn subtract(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 - n2)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 - n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 - n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 - n2)),
            _ => Err(RuntimeError::new(String::from("Illegal operation SUBTRACT")))
        }
    }

    pub fn multiply(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 * n2)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 * n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 * n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 * n2)),
            _ => Err(RuntimeError::new(String::from("Illegal operation MULTIPLY")))
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
            _ => Err(RuntimeError::new(String::from("Illegal operation DIVIDE")))
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
            _ => Err(RuntimeError::new(String::from("Illegal operation RAISE")))
        }
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