use std::fmt;
use std::error::Error;

use super::{ErrorKind, Position, MonaError};

#[derive(Debug, PartialEq)]
pub struct InvalidSyntaxError {
    name: String,
    details: String,
    kind: ErrorKind,
    position: Position,
}

impl InvalidSyntaxError {
    pub fn new(position: Position, details: String) -> InvalidSyntaxError {
        InvalidSyntaxError {
            name: String::from("Invalid Syntax"),
            details,
            kind: ErrorKind::Lexer,
            position,
        }
    }
}

impl Error for InvalidSyntaxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl MonaError for InvalidSyntaxError {
    
}

impl fmt::Display for InvalidSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        super::print_error(
            f,
            &self.position,
            &self.kind,
            self.name.to_owned(),
            String::new(),
        )
    }
}