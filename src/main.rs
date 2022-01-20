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

// use std::io::{self, stdin, Write};
use std::fs;

const NAME: &str = "C:\\Users\\carri\\Desktop\\Projects\\rust-parser\\src\\main.txt";

fn main() {
    // print!("> ");
    // io::stdout().flush().unwrap();

    // let mut context = Context::new(None);
    // context.symbol_table.set("true", Value::Boolean(true));
    // context.symbol_table.set("false", Value::Boolean(false));
    // context.symbol_table.set("null", Value::Null);

    let mut manager = ContextManager::new();

    let id = manager.create_context(None);

    manager.set(id, "true", Value::Boolean(true));
    manager.set(id, "false", Value::Boolean(false));
    manager.set(id, "null", Value::Null);

    // let mut name = String::new();

    // stdin().read_line(&mut name).unwrap();

    let code = fs::read_to_string(NAME).expect("Something went wrong reading the file");

    run(&code, &mut manager, id);
}

fn run(code: &str, manager: &mut ContextManager, context_id: i32) {
    
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
                    let mut interpreter = Interpreter::new(manager);

                    match interpreter.visit(&node, context_id) {
                        Err(error) => eprintln!("{}", error.to_string()),
                        Ok(value) => println!("{}", value.to_string(&manager))
                    }
                }
            }
        }
    }
}
