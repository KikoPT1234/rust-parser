use crate::interpreter::RuntimeResult;
use crate::error::RuntimeError;

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    Str(String)
}

use self::Value::*;

impl Value {
    pub fn add(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 + n2)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 + n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 + n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 + n2)),
            _ => Err(RuntimeError::new(String::from("Illegal operation ADD")))
        }
    }

    pub fn subtract(&self, other: Value) -> RuntimeResult {
        match (self, other) {
            (Int(n1), Int(n2)) => Ok(Int(n1 + n2)),
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
            (Int(n1), Int(n2)) => Ok(Float(*n1 as f32 / n2 as f32)),
            (Int(n1), Float(n2)) => Ok(Float(*n1 as f32 / n2)),
            (Float(n1), Int(n2)) => Ok(Float(n1 / n2 as f32)),
            (Float(n1), Float(n2)) => Ok(Float(n1 / n2)),
            _ => Err(RuntimeError::new(String::from("Illegal operation DIVIDE")))
        }
    }
}