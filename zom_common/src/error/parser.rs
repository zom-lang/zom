use std::error::Error;
use std::fmt;

use crate::{reverse_lexer::reverse_lexe, token::Token};

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
        tok_pos: usize,
        tokens: Vec<Token>,
        source_file: &mut String,
        filename: &mut String,
        details: String,
        token: Token,
    ) -> UnexpectedTokenError {
        UnexpectedTokenError {
            name: String::from("Unexpected Token Error"),
            details,
            kind: ErrorKind::Parser,
            position: reverse_lexe(
                tok_pos,
                tokens,
                source_file.to_string(),
                filename.to_string(),
            )
            .expect("ERR: Couldn't reverse lexe the error."),
            token,
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
