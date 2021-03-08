pub mod lexer;
pub mod token;
pub mod characters;
pub mod error;
pub mod parser;
pub mod node;
pub mod interpreter;
pub mod value;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::*;
use crate::error::*;
use crate::value::Value;

fn main() {
    const CODE: &'static str = "let helloWorld = \"hey \"+2;helloWorld";

    run(CODE);
}

fn run(code: &'static str) {
    let mut context = Context::new(None);
    context.symbol_table.set("true", Value::Boolean(true));
    context.symbol_table.set("false", Value::Boolean(false));
    context.symbol_table.set("null", Value::Null);

    let mut lexer = Lexer::new(&code);
    let result = lexer.tokenize();

    match result {
        Err(error) => println!("{}", error.to_string()),
        Ok(tokens) => {
            println!("{:?}", tokens);
            let mut parser = Parser::new(tokens);
            let result = parser.parse();
            
            match result {
                Err(error) => println!("{}", error.to_string()),
                Ok(node) => {
                    println!("{:?}", node);
                    let interpreter = Interpreter::new();

                    match interpreter.visit(&node, &mut context) {
                        Err(error) => println!("{}", error.to_string()),
                        Ok(value) => println!("{:?}", value)
                    }
                }
            }
        }
    }
}
