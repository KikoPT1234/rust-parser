pub mod lexer;
pub mod token;
pub mod characters;
pub mod error;
pub mod parser;
pub mod node;
pub mod interpreter;
pub mod value;
pub mod context;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::*;
use crate::error::*;
use crate::value::Value;
use crate::context::*;

use std::io::{self, stdin, Write};

fn main() {
    print!("> ");
    io::stdout().flush().unwrap();

    // let mut context = Context::new(None);
    // context.symbol_table.set("true", Value::Boolean(true));
    // context.symbol_table.set("false", Value::Boolean(false));
    // context.symbol_table.set("null", Value::Null);

    let mut manager = ContextManager::new();

    let id = manager.create_context(None);

    manager.set(id, "true", Value::Boolean(true));
    manager.set(id, "false", Value::Boolean(false));
    manager.set(id, "null", Value::Null);

    loop {
        let mut code = String::new();

        stdin().read_line(&mut code).unwrap();

        manager = run(code.as_str(), manager, id);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}

fn run(code: &str, mut manager: ContextManager, context_id: i32) -> ContextManager {
    

    let mut lexer = Lexer::new(&code);
    let result = lexer.tokenize();

    match result {
        Err(error) => eprintln!("{}", error.to_string()),
        Ok(tokens) => {
            println!("{:?}", tokens);
            let mut parser = Parser::new(tokens);
            let result = parser.parse();
            
            match result {
                Err(error) => eprintln!("{}", error.to_string()),
                Ok(node) => {
                    println!("{:?}", node);
                    let mut interpreter = Interpreter::new(&mut manager);

                    match interpreter.visit(&node, context_id) {
                        Err(error) => eprintln!("{}", error.to_string()),
                        Ok(value) => println!("{}", value.to_string(&manager))
                    }
                }
            }
        }
    }
    
    manager
}
