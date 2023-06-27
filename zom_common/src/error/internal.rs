use std::error::Error;
use std::fmt;

use super::{ErrorKind, Position, ZomError};

#[derive(Debug, PartialEq)]
pub struct InternalError {
    name: String,
    kind: ErrorKind,
    details: String,
}

impl InternalError {
    pub fn new(name: String, details: String) -> InternalError {
        InternalError {
            name,
            kind: ErrorKind::Internal,
            details,
        }
    }
}

impl Error for InternalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl ZomError for InternalError {
    fn details(&self) -> &str {
        self.details.as_str()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn position(&self) -> Option<Position> {
        None
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print_error(f)
    }
}
