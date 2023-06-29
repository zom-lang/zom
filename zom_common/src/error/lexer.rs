use std::error::Error;
use std::fmt;

use super::{ErrorKind, Position, ZomError};

#[derive(Debug, PartialEq)]
pub struct IllegalCharError {
    name: String,
    details: String,
    kind: ErrorKind,
    position: Position,
}

impl IllegalCharError {
    pub fn new(position: Position, ch: char) -> IllegalCharError {
        IllegalCharError {
            name: String::from("Illegal Character"),
            details: format!("Unexpected char `{}`.", ch),
            kind: ErrorKind::Lexer,
            position,
        }
    }
}

impl Error for IllegalCharError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl ZomError for IllegalCharError {
    fn details(&self) -> &str {
        &self.details
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn position(&self) -> Option<Position> {
        Some(self.position.clone())
    }
}

impl fmt::Display for IllegalCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        super::print_error(f, self)
    }
}
