use std::error::Error;
use std::fmt;

use lexer::Lexer;
use token::Token;

pub mod lexer;
pub mod token;

pub fn run(text: String) -> Result<Vec<Token>, Box<dyn Error>> {
    if !text.is_ascii() {
        return Err("mona does not support non-ascii characters.".to_owned())?;
    }
    let mut lexer = Lexer::new(&text);

    lexer.make_tokens()
}

#[derive(Debug, Clone)]
enum ErrorKind {
    Lexer,
    // Parser,
    // Interpreter,
}

#[derive(Debug)]
pub struct IllegalCharError {
    name: String,
    details: String,
    kind: ErrorKind,
}

impl IllegalCharError {
    pub fn new(details: String) -> IllegalCharError {
        IllegalCharError {
            name: String::from("Illegal Character"),
            details,
            kind: ErrorKind::Lexer,
        }
    }
}

impl Error for IllegalCharError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for IllegalCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?}): {}", self.name, self.kind, self.details)
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    index: u32,
    line: u32,
    column: u32,
}

impl Position {
    pub fn new(index: u32, line: u32, column: u32) -> Position {
        Position { index, line, column }
    }

    pub fn advance(&mut self, current_char: char) {
        self.index  += 1;
        self.column += 1;

        if current_char == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }
}
