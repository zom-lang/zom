use std::error::Error;

use lexer::Lexer;
use parser::Parser;
use token::Token;

pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flags {
    pub lexer: bool,
    pub parser: bool,
    pub interpreter: bool,
    pub verbose: bool,
}

impl Flags {
    pub fn new(lexer: bool, parser: bool, interpreter: bool, verbose: bool) -> Flags {
        Flags {
            lexer,
            parser,
            interpreter,
            verbose,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunnerResult {
    lex_res: Vec<Token>,
}

impl RunnerResult {
    pub fn new(lex_res: Vec<Token>) -> RunnerResult {
        RunnerResult { lex_res }
    }

    pub fn print_res(&self, flags: Flags) {
        if flags.lexer {
            println!("{:?}", self.lex_res);
        }
    }
}

pub fn run(filename: String, text: String) -> Result<RunnerResult, Box<dyn Error>> {
    let mut lexer = Lexer::new(&text, filename);

    let tokens = lexer.make_tokens()?;

    let mut parser = Parser::new(tokens.clone()); // TODO: Try removing this .clone()
    println!("{:?}", parser);

    Ok(RunnerResult::new(tokens))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    index: u32,
    line: u32,
    column: u32,
    filename: String,
    filetext: String,
}

impl Position {
    pub fn new(index: u32, line: u32, column: u32, filename: String, filetext: String) -> Position {
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
