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

#[derive(Debug)]
pub struct RunnerResult {
    lex_res: Option<Vec<Token>>,
}

impl RunnerResult {
    pub fn new(lex_res: Option<Vec<Token>>) -> RunnerResult {
        RunnerResult { lex_res }
    }

    pub fn print_res(&self, flags: Flags) {
        if flags.lexer {
            if let Some(toks) = &self.lex_res {
                println!("{:?}", toks);
            }
        }
    }
}

pub fn run(filename: String, text: String) -> Result<RunnerResult, Box<dyn Error>> {
    let mut lexer = Lexer::new(&text, filename);
    let mut res = RunnerResult::new(None);

    res.lex_res = Some(lexer.make_tokens()?);

    Ok(res)
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
