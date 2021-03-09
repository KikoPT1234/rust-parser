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

use std::io::{self, stdin, Write};

fn main() {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut context = Context::new(None);
    context.symbol_table.set("true", Value::Boolean(true));
    context.symbol_table.set("false", Value::Boolean(false));
    context.symbol_table.set("null", Value::Null);

    loop {
        let mut code = String::new();

        stdin().read_line(&mut code).unwrap();

        run(code.as_str(), &mut context);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}

fn run(code: &str, context: &mut Context) {
    

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
                    let interpreter = Interpreter::new();

                    match interpreter.visit(&node, context) {
                        Err(error) => eprintln!("{}", error.to_string()),
                        Ok(value) => println!("{:?}", value)
                    }
                }
            }
        }
    }
}
