use std::error::Error;
use std::fmt;

use super::{ErrorKind, Position, ZomError};

#[derive(Debug, PartialEq)]
pub struct IllegalCharError {
    name: String,
    kind: ErrorKind,
    position: Position,
}

impl IllegalCharError {
    pub fn new(position: Position) -> IllegalCharError {
        IllegalCharError {
            name: String::from("Illegal Character"),
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

impl ZomError for IllegalCharError {}

impl fmt::Display for IllegalCharError {
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
