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

fn print_error(
    f: &mut fmt::Formatter<'_>,
    position: &Position,
    kind: &ErrorKind,
    name: String,
    details: String,
) -> fmt::Result {
    let mut margin: usize = 5;
    let num_str_len = position.line.to_string().len();
    println!("the num str : `{}` with len of {}", position.line.to_string(), num_str_len);
    if num_str_len > margin {
        println!("margin = {margin}");
        margin += (num_str_len - margin) + 2
    }
    println!("margin after = {margin}");

    drop(num_str_len);

    writeln!(
        f,
        "Err: {:?}, in file `{}` at line {} :",
        kind, position.filename, position.line
    )
    .unwrap();
    writeln!(f, "{}|", str_fix_len("...".to_string(), margin)).unwrap();
    writeln!(
        f,
        "{}| {}",
        str_fix_len(position.line.to_string(), margin),
        position
            .filetext
            .split('\n')
            .nth((position.line - 1) as usize)
            .unwrap()
    )
    .unwrap();
    writeln!(f, "{}| {}^", str_fix_len("...".to_string(), margin) ,spaces(position.column as usize)).unwrap();
    write!(f, "  {}{}", spaces(position.column as usize + margin), name).unwrap();
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
