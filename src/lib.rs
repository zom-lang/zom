use std::error::Error;

use lexer::Lexer;
use token::Token;

pub mod lexer;
pub mod token;
pub mod error;

pub fn run(filename: String, text: String) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut lexer = Lexer::new(&text, filename);

    lexer.make_tokens()
}

#[derive(Debug, Clone)]
pub struct Position {
    index: u32,
    line: u32,
    column: u32,
    filename: String,
    filetext: String,
}

impl Position {
    pub fn new(index: u32, line: u32, column: u32, filename: String, filetext: String) -> Position {
        Position { index, line, column, filename, filetext }
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
