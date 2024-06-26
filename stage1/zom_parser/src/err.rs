use std::fmt::Write;
use zom_common::token::{Token, TokenType};
use zom_errors::prelude::*;

use std::ops::Range;

/// unknown token lexing error
pub struct ExpectedToken<E: ExpectArg> {
    /// token found
    pub found: FmtToken,
    /// expected token / syntax
    pub expected: E,
    /// location of the found token
    pub location: CodeSpan,

    pub other: Vec<LogPart>,
}

impl<E: ExpectArg> ExpectedToken<E> {
    pub fn from(found: &Token, expected: E) -> Self {
        Self {
            found: FmtToken::from_token(found),
            expected,
            location: found.span.clone(),
            other: Vec::new(),
        }
    }

    pub fn with_note(found: &Token, expected: E, note: Box<str>, note_loc: Range<usize>) -> Self {
        Self {
            found: FmtToken::from_token(found),
            expected,
            location: found.span.clone(),
            other: vec![LogPart {
                loc: Some(note_loc),
                lvl: LogLevel::Note,
                msg: note,
            }],
        }
    }
}

impl<E: ExpectArg> Log for ExpectedToken<E> {
    fn location(&self) -> CodeSpan {
        self.location.clone()
    }

    fn level(&self) -> LogLevel {
        LogLevel::Error
    }

    fn msg(&self) -> Box<str> {
        format!("expected {}, found {}", self.expected.fmt(), self.found).into()
    }

    fn other_parts(&self) -> Vec<LogPart> {
        self.other.clone()
    }
}

pub trait ExpectArg {
    fn try_fmt(&self) -> Option<String>;
    fn fmt(&self) -> String {
        self.try_fmt().expect("failed to format ExpectArg")
    }
}

impl ExpectArg for FmtToken {
    fn try_fmt(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl ExpectArg for PartAST {
    fn try_fmt(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<T: ExpectArg, const N: usize> ExpectArg for [T; N] {
    fn try_fmt(&self) -> Option<String> {
        if self.len() == 1 {
            return self.first().map(|t| t.fmt());
        }
        let mut s = String::new();
        for (idx, token) in self.iter().enumerate() {
            if idx == self.len() - 2 {
                write!(s, "{} ", token.fmt()).ok()?;
            } else if idx == self.len() - 1 {
                write!(s, "or {}", token.fmt()).ok()?;
            } else {
                write!(s, "{}, ", token.fmt()).ok()?;
            }
        }
        Some(s)
    }
}

impl ExpectArg for TokenType {
    fn try_fmt(&self) -> Option<String> {
        Some(self.to_string())
    }
}
