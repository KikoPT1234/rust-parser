pub trait Error {
    fn msg(&self) -> &str;

    fn name(&self) -> &str;

    fn to_string(&self) -> String {
        String::from(self.name()) + ": " + self.msg()
    }
}

#[derive(Debug)]
pub struct LexError {
    msg: String,
    name: String
}

impl LexError {
    pub fn new(msg: String) -> Self {
        LexError {
            msg,
            name: String::from("Lex Error")
        }
    }
}

impl Error for LexError {
    fn msg(&self) -> &str {
        &self.msg
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct ParseError {
    msg: String,
    name: String
}

impl ParseError {
    pub fn new(msg: String) -> Self {
        ParseError {
            msg,
            name: String::from("Syntax Error")
        }
    }
}

impl Error for ParseError {
    fn msg(&self) -> &str {
        &self.msg
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    msg: String,
    name: String
}

impl RuntimeError {
    pub fn new(msg: String) -> Self {
        RuntimeError {
            msg,
            name: String::from("Runtime Error")
        }
    }
}

impl Error for RuntimeError {
    fn msg(&self) -> &str {
        &self.msg
    }

    fn name(&self) -> &str {
        &self.name
    }
}