use std::error::Error;
use std::fmt;

use lexer::Lexer;
use token::Token;

pub mod lexer;
pub mod token;

pub fn run(text: &String) -> Result<Vec<Token>, IllegalCharError> {
    let mut lexer = Lexer::new(text);
    let tokens = lexer.make_tokens();

    return tokens;
}

#[derive(Debug)]
pub struct IllegalCharError {
    name: String,
    details: String,
}

impl IllegalCharError {
    pub fn new(details: String) -> IllegalCharError {
        IllegalCharError {
            name: String::from("Illegal Character"),
            details,
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
        write!(f, "{}: {}", self.name, self.details)
    }
}
