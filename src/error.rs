pub trait Error {
    fn msg(&self) -> &str;
}

#[derive(Debug)]
pub struct LexError {
    msg: String
}

impl LexError {
    pub fn new(msg: String) -> Self {
        LexError {
            msg
        }
    }
}

impl Error for LexError {
    fn msg(&self) -> &str {
        &self.msg
    }
}

#[derive(Debug)]
pub struct ParseError {
    msg: String
}

impl ParseError {
    pub fn new(msg: String) -> Self {
        ParseError {
            msg
        }
    }
}

impl Error for ParseError {
    fn msg(&self) -> &str {
        &self.msg
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    msg: String
}

impl RuntimeError {
    pub fn new(msg: String) -> Self {
        RuntimeError {
            msg
        }
    }
}

impl Error for RuntimeError {
    fn msg(&self) -> &str {
        &self.msg
    }
}