pub mod lexer;
pub mod token_type;
pub mod characters;
pub mod error;

use lexer::Lexer;

fn main() {
    const CODE: &'static str = "1 + 1 + +434.5.6";

    run(CODE);
}

fn run(code: &'static str) {
    let mut lexer = Lexer::new(&code);
    let result = lexer.tokenize();

    match result {
        Ok(tokens) => println!("{:?}", tokens),
        Err(error) => println!("{:?}", error)
    }
}
