// use std::error::Error;
use std::fmt;

pub trait Error {
    fn msg(&self) -> &str;

    fn new(msg: String) -> Self;
}

pub struct LexError {
    msg: String
}

impl fmt::Debug for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl<'a> fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Error for LexError {
    fn msg(&self) -> &str {
        &self.msg
    }

    fn new(msg: String) -> Self {
        let lexer: LexError = LexError {
            msg
        };
        lexer
    }
}

// impl Error for LexError {

// }