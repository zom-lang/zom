use std::fmt;
use std::error::Error;

pub mod lexer;
pub mod token;


#[derive(Debug)]
pub struct IllegalCharError {
    name: String,
    details: String,
}

impl IllegalCharError {
    pub fn new(details: String) -> IllegalCharError{
        IllegalCharError { name: String::from("Illegal Character"), details}
    }
}

impl Error for IllegalCharError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for IllegalCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.details)
    }
}