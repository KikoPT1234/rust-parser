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
use crate::interpreter::Interpreter;
use crate::error::*;

fn main() {
    const CODE: &'static str = "1 / 1 + 434.56";

    run(CODE);
}

fn run(code: &'static str) {
    let mut lexer = Lexer::new(&code);
    let result = lexer.tokenize();

    match result {
        Err(error) => println!("{}", error.msg()),
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            let result = parser.parse();
            
            match result {
                Err(error) => println!("{}", error.msg()),
                Ok(node) => {
                    let interpreter = Interpreter::new(node);

                    match interpreter.execute() {
                        Err(error) => println!("{}", error.msg()),
                        Ok(value) => println!("{:?}", value)
                    }
                }
            }
        }
    }
}
