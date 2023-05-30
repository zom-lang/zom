use std::io::{self, Write};

use crate::{
    lexer::Lexer,
    parser::{parse, ASTNode, ParserSettings},
    token::Token,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flags {
    pub lexer: bool,
    pub parser: bool,
    pub verbose: bool,
}

impl Flags {
    pub fn new(lexer: bool, parser: bool, verbose: bool) -> Flags {
        Flags {
            lexer,
            parser,
            verbose,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RunnerResult {
    tokens: Vec<Token>,
    ast: Vec<ASTNode>,
}

impl RunnerResult {
    pub fn new(tokens: Vec<Token>, ast: Vec<ASTNode>) -> RunnerResult {
        RunnerResult { tokens, ast }
    }

    pub fn print(&self, flags: Flags) {
        flags
            .lexer
            .then(|| println!(" Lexer : \n{:?}\n", self.tokens));
        flags.parser.then(|| println!(" Parser : \n{:#?}", self.ast) );
    }
}
pub fn main_loop(flags: Flags) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut parser_settings = ParserSettings::default();

    'main: loop {
        print!("~> ");
        stdout.flush().unwrap();

        match stdin.read_line(&mut input) {
            Ok(_len) => {}
            Err(err) => {
                println!("ERR: {}", err);
                continue;
            }
        }

        if input.as_str() == ".quit\n" {
            break;
        }

        // the constructed AST
        let mut ast = Vec::new();

        let mut res = RunnerResult::default();

        loop {
            let mut buf = String::new();
            print!(" > ");
            stdout.flush().unwrap();

            match stdin.read_line(&mut buf) {
                Ok(_len) => {
                    if buf.as_str() == ".eof\n" {
                        break;
                    } else if buf.as_str() == ".quit\n" {
                        break 'main;
                    }
                    input.push_str(buf.as_str());
                }
                Err(err) => {
                    println!("ERR: {}", err);
                    continue;
                }
            }
        }

        let mut lexer = Lexer::new(&input, "<stdin>".to_string());
        let lexer_result = lexer.make_tokens();

        input.clear();

        let tokens = match lexer_result {
            Ok(toks) => toks,
            Err(err) => {
                eprintln!("{}", err);
                continue 'main;
            }
        };

        let parsing_result = parse(tokens.as_slice(), ast.as_slice(), &mut parser_settings);

        res.tokens = tokens;

        match parsing_result {
            Ok((parsed_ast, rest)) => {
                ast.extend(parsed_ast.clone().into_iter());
                if rest.is_empty() {
                    res.ast = parsed_ast;
                }
            }
            Err(message) => {
                println!("Err: {}", message);
                continue 'main;
            }
        }

        stdout.flush().unwrap();
        input.clear();
        res.print(flags);
    }
}
