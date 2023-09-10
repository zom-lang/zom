//! error module.
//!
//! This used to spawn custom (beautiful) error message when a component of Zom fails.

use super::{build_date, build_target_triple, commit_hash};
use std::error::Error;
use std::fmt::{self, Display};
use std::ops::Range;
use std::path::PathBuf;

/// This function return spaces * len
/// It is used for implement Display for errors
fn spaces(len: usize) -> String {
    let mut spaces_str = String::new();
    for _ in 0..len {
        spaces_str.push(' ');
    }
    spaces_str
}

/// Pad a string until it reaches the desired lenght ("len") with spaces.
fn pad_string(string: String, len: usize) -> String {
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

/// Takes a string and an index into that string and iterate over it,
/// and return the line and column corresponding to that index in the file.
/// The line starts at 1 and column starts at 1.
pub fn line_col(text: &str, index: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (idx, ch) in text.char_indices() {
        if index == idx {
            break;
        }
        match ch {
            '\n' => {
                col = 1;
                line += 1;
            }
            _ => col += 1,
        }
    }

    (line, col)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    /// The index when you iterate over the filetext.
    index: usize,
    line_start: usize,
    col_start: usize,
    line_end: usize,
    col_end: usize,
    filename: PathBuf,
    filetext: String,
}

impl Position {
    pub fn new(
        index: usize,
        line_start: usize,
        col_start: usize,
        line_end: usize,
        col_end: usize,
        filename: PathBuf,
        filetext: String,
    ) -> Position {
        Position {
            index,
            line_start,
            col_start,
            line_end,
            col_end,
            filename,
            filetext,
        }
    }

    /// Return a position from the index, the range, the filetext and the filename.
    /// Can return `None` if the range's start position is greater that its end position.
    pub fn try_from_range(
        index: usize,
        range: Range<usize>,
        filetext: String,
        filename: PathBuf,
    ) -> Option<Position> {
        assert!(range.start < range.end); // TODO: add an error message for this assertion
        let (line_start, col_start) = line_col(&filetext, range.start);
        let (line_end, col_end) = line_col(&filetext, range.end);

        Some(Position {
            index,
            line_start,
            col_start,
            line_end,
            col_end,
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
    help: Option<String>,
    notes: Vec<String>,
}

impl ZomError {
    /// If col_start == col_end None will be returned.
    pub fn try_new(
        location: Option<Position>,
        details: String,
        is_warning: bool,
        help: Option<String>,
        notes: Vec<String>,
    ) -> Option<ZomError> {
        if let Some(pos) = &location {
            if pos.col_start == pos.col_end {
                return None;
            }
        }

        Some(ZomError {
            location,
            details,
            is_warning,
            help,
            notes,
        })
    }

    /// If col_start == col_end it will panic.
    pub fn new(
        location: Option<Position>,
        details: String,
        is_warning: bool,
        help: Option<String>,
        notes: Vec<String>,
    ) -> ZomError {
        Self::try_new(location, details, is_warning, help, notes).unwrap()
    }

    /// Create a new Internal Compiler Error
    pub fn ice_error(details: String) -> ZomError {
        ZomError {
            location: None,
            details: "internal compiler error: ".to_owned() + &details,
            is_warning: false,
            help: None,
            notes: [
            "the compiler unexpectedly panicked. this is a bug.",
            "we would appreciate a bug report: https://github.com/zom-lang/zom/issues/new?assignees=&labels=C-bug%2C+I-ICE%2C+A-compiler&projects=&template=ice.md",
            format!("zomc {} ({} {}) running on {}",
                    env!("CARGO_PKG_VERSION"),
                    &commit_hash()[..7],
                    build_date(),
                    build_target_triple()).as_str()
            ]
            .iter()
            .map(|v| v.to_string())
            .collect(),
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

    fn write_help(&self, f: &mut fmt::Formatter<'_>, prefix: &str) -> fmt::Result {
        if let Some(help) = self.help.clone() {
            writeln!(f, "{}help: {}", prefix, help)?;
        }
        Ok(())
    }

    fn write_notes(&self, f: &mut fmt::Formatter<'_>, prefix: &str) -> fmt::Result {
        for note in &self.notes {
            writeln!(f, "{}note: {}", prefix, note)?;
        }
        Ok(())
    }

    /// Add a position to an error if it doesn't have one. If it already have a position,
    /// the function will panic.
    pub fn add_position(&mut self, pos: Position) {
        assert!(self.location.is_none(), "The error has already a position.");
        self.location = Some(pos);
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
            // Where the error is
            writeln!(
                f,
                "  --> {}:{}:{}",
                self.pos().filename.display(),
                self.pos().line_start,
                self.pos().col_start
            )?;

            // Calculate the margin
            let mut margin = 3;

            let line_start_str = self.pos().line_start.to_string();
            if line_start_str.len() > margin {
                margin += line_start_str.len() - margin + 1;
            }

            let line_end_str = self.pos().line_end.to_string();
            if line_end_str.len() > margin {
                margin += line_end_str.len() - margin + 1;
            }
            // Write the start line
            let line_start = self
                .pos()
                .filetext
                .lines()
                .nth(self.pos().line_start - 1) // minus one because the line number starts at 1 and an the nth function "starts" at 0
                .unwrap();

            writeln!(f, "{}|", spaces(margin))?;
            writeln!(f, "{}| {}", pad_string(line_start_str, margin), line_start)?;
            // Write either the end line or the error
            if self.pos().line_start == self.pos().line_end {
                writeln!(
                    f,
                    "{}|{}{}",
                    spaces(margin),
                    spaces(self.pos().col_start),
                    "^".repeat(self.pos().col_end - self.pos().col_start)
                )?;
            } else {
                writeln!(
                    f,
                    "{}|{}{}",
                    spaces(margin),
                    spaces(self.pos().col_start),
                    "~".repeat(line_start.len() - self.pos().col_start)
                )?;
                if (self.pos().line_end - self.pos().line_start) != 1 {
                    writeln!(f, "{}| ...", spaces(margin))?;
                }
                let line_end = self
                    .pos()
                    .filetext
                    .lines()
                    .nth(self.pos().line_end - 1) // minus 1, same as line_start
                    .unwrap();
                writeln!(f, "{}|{}", pad_string(line_end_str, margin), line_end)?;
                writeln!(
                    f,
                    "{}|{}^",
                    spaces(margin),
                    "~".repeat(self.pos().col_end - 1)
                )?;
            }
            // Write an help message if there is one
            self.write_help(f, format!("{}= ", spaces(margin)).as_str())?;
            // Write note(s)
            self.write_notes(f, format!("{}= ", spaces(margin)).as_str())?;
        } else {
            // Write an help message if there is one
            self.write_help(f, "")?;
            // Write a note if there is one
            self.write_notes(f, "")?;
        }
        Ok(())
    }
}

impl Error for ZomError {}
