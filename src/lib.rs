use std::error::Error;

use lexer::Lexer;
use token::Token;
use parser::{parse, ParserSettings, ASTNode};

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
    tokens: Vec<Token>,
    ast: Vec<ASTNode>,
}

impl RunnerResult {
    pub fn new(tokens: Vec<Token>, ast: Vec<ASTNode>) -> RunnerResult {
        RunnerResult {
            tokens,
            ast,
        }
    }

    pub fn print_res(&self, flags: Flags) {
        flags.lexer.then(|| println!(" Lexer : \n{:?}\n", self.tokens));
        flags.parser.then(|| {
            println!(" Parser : \n{:#?}", self.ast);
        });
    }
}

pub fn run(filename: String, text: String) -> Result<RunnerResult, Box<dyn Error>> {
    let mut lexer = Lexer::new(&text, filename);

    let tokens = lexer.make_tokens()?;

    let (ast, _tokens) = parse(tokens.as_slice(), Vec::new().as_slice(), &mut ParserSettings::default())?;

    Ok(RunnerResult::new(tokens, ast))
}
