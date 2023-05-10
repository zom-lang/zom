use std::error::Error;
use std::fmt;

use crate::Position;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Lexer,
    Parser,
    Interpreter,
    General,
}

#[derive(Debug)]
pub struct IllegalCharError {
    name: String,
    details: String,
    kind: ErrorKind,
    position: Position,
}

impl IllegalCharError {
    pub fn new(details: String, position: Position) -> IllegalCharError {
        IllegalCharError {
            name: String::from("Illegal Character"),
            details,
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
    let mut num_str = String::with_capacity(num as usize);
    let num_len = num.to_string().len();

    if num_len == len {
        return num.to_string();
    }

    let len_diff = len - num_len;
    num_str.push_str(&spaces(len_diff / 2));
    num_str.push_str(&num.to_string()[..]);
    num_str.push_str(&spaces(len_diff / 2));
    
    num_str
}

impl fmt::Display for IllegalCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Err: {:?}, in file `{}` at line {} :", self.kind, self.position.filename, self.position.line).unwrap();
        writeln!(f, " ... |").unwrap();
        writeln!(
            f,
            "{}| {}", 
            num_str_fix_len(self.position.line, 5), 
            //self.position.filetext.split('\n').nth((self.position.line - 1) as usize).unwrap()
            self.position.filetext.split('\n').nth(0).unwrap()
        ).unwrap();
        writeln!(f, " ... | {}^", spaces(self.position.column as usize))
    }
}