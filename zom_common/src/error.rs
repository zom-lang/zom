//! error module.
//!
//! This used to spawn custom (beautiful) error message when a component of Zom fails.

use std::error::Error;
use std::fmt::{self, Display};

pub mod internal;
pub mod lexer;
pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Lexer,
    Parser,
    Codegen,
    Compiler,
    General,
    Internal,
}

/// This function return spaces * len
/// It is used for implement Display for errors
fn spaces(len: usize) -> String {
    let mut spaces_str = String::new();
    for _ in 0..len {
        spaces_str.push(' ');
    }
    spaces_str
}

fn str_fix_len(string: String, len: usize) -> String {
    let mut num_str = String::with_capacity(len);
    let num_len = string.len();

    if num_len == len {
        return string.to_string();
    }

    let len_diff = len - num_len;
    num_str.push_str(&spaces(len_diff / 2));
    num_str.push_str(&string[..]);
    num_str.push_str(&spaces(len_diff / 2));

    if num_str.len() != len {
        num_str.push(' ');
    }

    num_str
}

/// Safety :
/// We assume that the error in zom_err is knowed, if it's not then it will panic because there is an `unwrap()`.
fn print_error(f: &mut fmt::Formatter<'_>, zom_err: &dyn ZomError) -> fmt::Result {
    let mut margin: usize = 5;
    let num_str_len = zom_err.pos().line.to_string().len();
    if num_str_len > margin {
        margin += (num_str_len - margin) + 2
    }

    writeln!(
        f,
        "error: in file `{}` at line {} :",
        zom_err.pos().filename,
        zom_err.pos().line
    )
    .unwrap();
    writeln!(f, "{}|", str_fix_len("...".to_string(), margin)).unwrap();
    writeln!(
        f,
        "{}| {}",
        str_fix_len(zom_err.pos().line.to_string(), margin),
        zom_err
            .pos()
            .filetext
            .split('\n')
            .nth(zom_err.pos().line - 1)
            .unwrap()
    )
    .unwrap();
    writeln!(
        f,
        "{}| {}^",
        str_fix_len("...".to_string(), margin),
        spaces(zom_err.pos().column)
    )
    .unwrap();
    if !zom_err.details().is_empty() {
        return writeln!(
            f,
            "       {}{}",
            spaces(zom_err.pos().column),
            zom_err.details()
        );
    }
    write!(f, "")
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    /// The index when you iterate over the filetext.
    index: usize,
    line: usize,
    column: usize,
    filename: String,
    filetext: String,
}

impl Position {
    pub fn new(
        index: usize,
        line: usize,
        column: usize,
        filename: String,
        filetext: String,
    ) -> Position {
        Position {
            index,
            line,
            column,
            filename,
            filetext,
        }
    }

    pub fn advance(&mut self, current_char: char) {
        self.index += 1;
        self.column += 1;

        if current_char == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }
}

pub trait ZomError: Error + Display {
    fn name(&self) -> &str;

    fn details(&self) -> &str;

    fn position(&self) -> Option<Position>;

    /// Alias for `.position().unwrap()` so if the error was no position it will panic.
    #[inline]
    fn pos(&self) -> Position {
        self.position().unwrap()
    }

    fn print_error(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    where
        Self: Sized,
    {
        if self.position().is_none() {
            // this is a position less error.
            todo!()
        }
        // it's not a position less error

        print_error(f, self)
    }
}
