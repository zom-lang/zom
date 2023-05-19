use std::error::Error;

use lexer::Lexer;
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
    lexer_result: Vec<Token>,
}

impl RunnerResult {
    pub fn new(lexer_result: Vec<Token>) -> RunnerResult {
        RunnerResult {
            lexer_result,
        }
    }

    pub fn print_res(&self, flags: Flags) {
        flags.lexer.then(|| println!(" Lexer : \n{:?}\n", self.lexer_result));
        flags.parser.then(|| {
            // things here
        });
    }
}

pub fn run(filename: String, text: String) -> Result<RunnerResult, Box<dyn Error>> {
    let mut lexer = Lexer::new(&text, filename);

    let tokens = lexer.make_tokens()?;

    Ok(RunnerResult::new(tokens))
}
