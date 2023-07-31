//! error module.
//!
//! This used to spawn custom (beautiful) error message when a component of Zom fails.

use std::error::Error;
use std::fmt::{self, Display};
use std::ops::RangeInclusive;

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
    let mut str = String::with_capacity(len);
    let str_len = string.len();

    if str_len == len {
        return string;
    }

    let len_diff = len - str_len;
    str.push_str(&spaces(len_diff / 2));
    str.push_str(&string);
    str.push_str(&spaces(len_diff / 2));

    if str.len() != len {
        str.push(' ');
    }

    str
}

/// Safety :
/// We assume that the error in zom_err is knowed, if it's not then it will panic because there is an `unwrap()`.
fn print_error(f: &mut fmt::Formatter<'_>, zom_err: ZomError) -> fmt::Result {
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

    /// Return a position from the index, the range, the filetext and the filename.
    /// Can return `None` if the range's start position is greater that its end position.
    pub fn try_from_range(
        index: usize,
        range: RangeInclusive<usize>,
        filetext: String,
        filename: String,
    ) -> Option<Position> {
        let start_position = *range.start();
        let end_position = *range.end();
        let file_content = filetext.clone();

        // Ensure positions are positive and start is less than or equal to end
        if start_position > end_position {
            return None;
        }

        let mut line_number = 1;
        let mut column_number = 1;
        let mut current_position = 0;

        for line in file_content.lines() {
            let line_length = line.len();

            if current_position <= start_position
                && start_position <= current_position + line_length
            {
                column_number = start_position - current_position + 1;
                break;
            }

            current_position += line_length + 1; // Add 1 for the newline character
            line_number += 1;
        }

        Some(Position {
            index,
            line: line_number,
            column: column_number,
            filename,
            filetext,
        })
    }
}

#[derive(Debug)]
pub struct ZomError {
    location: Option<Position>,
    details: String,
    is_warning: bool,
}

impl ZomError {
    pub fn new(location: Option<Position>, details: String, is_warning: bool) -> ZomError {
        ZomError {
            location,
            details,
            is_warning,
        }
    }

    /// Return the position of the error, if it's none, it will panic.
    pub fn pos(&self) -> &Position {
        self.try_pos().unwrap()
    }

    /// Return the position of the error, if it's none, it will panic.
    pub fn try_pos(&self) -> Option<&Position> {
        self.location.as_ref()
    }

    /// Return a ref to the details of the error.
    pub fn details(&self) -> &str {
        &self.details
    }

    /// Return true if the error has a position or false if it doesn't.
    pub fn has_pos(&self) -> bool {
        self.location.is_some()
    }
}

impl Display for ZomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_warning {
            writeln!(f, "warning: {}", self.details)?;
        } else {
            writeln!(f, "error: {}", self.details)?;
        }

        if self.has_pos() {
            writeln!(
                f,
                "  --> {}:{}:{}",
                self.pos().filename,
                self.pos().line,
                self.pos().column
            )?;
            let mut margin = 3;
            let mut line_str = self.pos().line.to_string();
            if line_str.len() > margin {
                margin += line_str.len() - margin + 1;
            }
            writeln!(f, "{}|", spaces(margin))?;
            writeln!(
                f,
                "{}| {:?}",
                str_fix_len(line_str, margin),
                self.pos().filetext.lines().nth(self.pos().line)
            )?;
            writeln!(f, "{}|", spaces(margin))?;
        }
        Ok(())
    }
}
