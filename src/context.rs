use crate::value::Value;
use std::collections::HashMap;

use rand;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Context {
    pub id: i32,
    pub parent: Option<i32>,
    symbols: HashMap<String, Value>
}

impl Context {
    pub fn new(manager: &ContextManager, parent: Option<i32>) -> Context {

        let mut id: i32 = 0;

        let mut rng = rand::thread_rng();

        loop {
            id = rng.gen_range(0..0xFFFF);

            if !manager.has_id(id) { break; };
        }

        Context {
            id,
            parent,
            symbols: HashMap::new()
        }
    }

    fn set(&mut self, name: &str, value: Value) {
        self.symbols.insert(String::from(name), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        match self.symbols.get(name) {
            Some(value) => Some(&value),
            None => None
        }
    }
}

pub struct ContextManager {
    contexts: HashMap<i32, Context>
}

impl ContextManager {

    pub fn new() -> ContextManager {
        ContextManager {
            contexts: HashMap::new()
        }
    }

    pub fn get(&self, context_id: i32, name: &str) -> Option<&Value> {
        let context = self.contexts.get(&context_id)?;

        match context.get(name) {
            Some(value) => Some(value),
            None => {
                match context.parent {
                    Some(id) => self.get(id, name),
                    None => None
                }
            }
        }
    }

    pub fn set(&mut self, context_id: i32, name: &str, value: Value) -> Option<&Value> {
        let context = self.contexts.get_mut(&context_id)?;

        context.set(name, value);

        self.get(context_id, name)
    }

    pub fn add_context(&mut self, context: Context) {
        self.contexts.insert(context.id, context);
    }

    pub fn create_context(&mut self, parent: Option<i32>) -> i32 {
        let mut context = Context::new(self, parent);

        let id = context.id;

        self.add_context(context);

        return id;
    }

    pub fn has_id(&self, id: i32) -> bool {
        self.contexts.contains_key(&id)
    }
}