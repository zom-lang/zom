use std::error::Error;
use std::fmt;

use crate::token::Token;

use super::{ErrorKind, Position, ZomError};

#[derive(Debug, PartialEq)]
pub struct UnexpectedTokenError {
    name: String,
    details: String,
    kind: ErrorKind,
    position: Position,
    token: Token,
}

impl UnexpectedTokenError {
    pub fn new(position: Position, details: String, token: Token) -> UnexpectedTokenError {
        UnexpectedTokenError {
            name: String::from("Unexpected Token Error"),
            details,
            kind: ErrorKind::Parser,
            position,
            token,
        }
    }

    pub fn from_pos(
        _tok_pos: usize,
        _tokens: Vec<Token>,
        _source_file: &mut String,
        _filename: &mut String,
        details: String,
        _token: Token,
    ) -> UnexpectedTokenError {
        UnexpectedTokenError {
            name: String::from("Unexpected Token Error"),
            details,
            kind: ErrorKind::Parser,
            position: todo!(),
            #[allow(unreachable_code)]
            token: _token,
        }
    }
}

impl Error for UnexpectedTokenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl ZomError for UnexpectedTokenError {
    fn details(&self) -> &str {
        self.details.as_str()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn position(&self) -> Option<Position> {
        Some(self.position.clone())
    }
}

impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        super::print_error(f, self)
    }
}
