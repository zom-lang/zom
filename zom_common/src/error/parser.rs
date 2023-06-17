use std::error::Error;
use std::fmt;

use super::{ErrorKind, Position, ZomError};

#[derive(Debug, PartialEq)]
pub struct UnexpectedTokenError {
    name: String,
    details: String,
    kind: ErrorKind,
    position: Position,
}

impl UnexpectedTokenError {
    pub fn new(position: Position, details: String) -> UnexpectedTokenError {
        UnexpectedTokenError {
            name: String::from("Unexpected Token Error"),
            details,
            kind: ErrorKind::Parser,
            position,
        }
    }
}

impl Error for UnexpectedTokenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl ZomError for UnexpectedTokenError {}

impl fmt::Display for UnexpectedTokenError {
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
