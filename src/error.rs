use std::error::Error;
use std::fmt;

use crate::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Lexer,
    Parser,
    Interpreter,
    General,
}

#[derive(Debug, PartialEq)]
pub struct IllegalCharError {
    name: String,
    _details: String,
    kind: ErrorKind,
    position: Position,
}

impl IllegalCharError {
    pub fn new(details: String, position: Position) -> IllegalCharError {
        IllegalCharError {
            name: String::from("Illegal Character"),
            _details: details,
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

/// This function return spaces * len
/// It is used for implement Display for errors
fn spaces(len: usize) -> String {
    let mut spaces_str = String::new();
    for _ in 0..len {
        spaces_str.push(' ');
    }
    spaces_str
}

fn num_str_fix_len(num: u32, len: usize) -> String {
    let mut num_str = String::with_capacity(len);
    let num_len = num.to_string().len();

    if num_len == len {
        return num.to_string();
    }

    let len_diff = len - num_len;
    num_str.push_str(&spaces(len_diff / 2));
    num_str.push_str(&num.to_string()[..]);
    num_str.push_str(&spaces(len_diff / 2));

    if num_str.len() != len {
        num_str.push(' ');
    }

    num_str
}

pub fn print_error(
    f: &mut fmt::Formatter<'_>,
    position: &Position,
    kind: &ErrorKind,
    name: String,
    details: String,
) -> fmt::Result {
    //TODO: Support error messages with line digits bigger than 5 characters.
    writeln!(
        f,
        "Err: {:?}, in file `{}` at line {} :",
        kind, position.filename, position.line
    )
    .unwrap();
    writeln!(f, " ... |").unwrap();
    writeln!(
        f,
        "{}| {}",
        num_str_fix_len(position.line, 5),
        position
            .filetext
            .split('\n')
            .nth((position.line - 1) as usize)
            .unwrap()
    )
    .unwrap();
    writeln!(f, " ... | {}^", spaces(position.column as usize)).unwrap();
    write!(f, "       {}{}", spaces(position.column as usize), name).unwrap();
    if !details.is_empty() {
        println!();
        return write!(f, "        {}{}", spaces(position.column as usize), details);
    }
    write!(f, "")
}

impl fmt::Display for IllegalCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_error(
            f,
            &self.position,
            &self.kind,
            self.name.to_owned(),
            String::new(),
        )
    }
}

// General Lexer Error

#[derive(Debug, PartialEq)]
pub struct GeneralError {
    name: String,
    details: String,
    kind: ErrorKind,
    position: Position,
}

impl GeneralError {
    pub fn new(name: String, kind: ErrorKind, details: String, position: Position) -> GeneralError {
        GeneralError {
            name, // String::from("Parse Litteral Error")
            details,
            kind,
            position,
        }
    }
}

impl Error for GeneralError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_error(
            f,
            &self.position,
            &self.kind,
            self.name.to_string(),
            self.details.to_string(),
        )
    }
}
