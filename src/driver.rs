//! This is a REPL implementation for Mona

use std::io::{self, Write};

use inkwell::{context::Context, passes::PassManager, values::FunctionValue};

use crate::{fe::{
    lexer::Lexer,
    parser::{parse, ASTNode, ParserSettings},
    token::Token,
}, me::compiler::Compiler};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flags {
    pub lexer: bool,
    pub parser: bool,
    pub llvm_ir: bool,
    pub verbose: bool,
}

impl Flags {
    pub fn new(lexer: bool, parser: bool, llvm_ir: bool,  verbose: bool) -> Flags {
        Flags {
            lexer,
            parser,
            llvm_ir,
            verbose,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RunnerResult<'ctx> {
    tokens: Vec<Token>,
    ast: Vec<ASTNode>,
    funcs: Vec<FunctionValue<'ctx>>,
}

impl<'ctx> RunnerResult<'ctx> {
    pub fn new(tokens: Vec<Token>, ast: Vec<ASTNode>, funcs: Vec<FunctionValue<'ctx>>) -> RunnerResult<'ctx> {
        RunnerResult { tokens, ast, funcs }
    }

    pub fn print(&self, flags: Flags) {
        flags
            .lexer
            .then(|| println!("> Attempting to lex input : \n{:?}\n", self.tokens));
        flags
            .parser
            .then(|| println!("> Attempting to parse the lexed input : \n{:#?}", self.ast));
        flags
            .llvm_ir
            .then(|| println!("> Attempting to compile the parsed input : \n{:#?}", self.funcs));
    }
}
pub fn main_loop(flags: Flags) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut parser_settings = ParserSettings::default();

    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    // Create FPM
    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    // the constructed AST
    let mut ast: Vec<ASTNode> = Vec::new();

    'main: loop {
        print!("==> ");
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

        let mut res = RunnerResult::default();

        loop {
            let mut buf = String::new();
            print!(" ~> ");
            stdout.flush().unwrap();

            match stdin.read_line(&mut buf) {
                Ok(_len) => {
                    if buf.as_str() == ".eof\n" {
                        break;
                    } else if buf.as_str() == ".quit\n" {
                        break 'main;
                    } else if buf.as_str() == ".full_ast\n" {
                        println!("> Full AST : \n{ast:#?}");
                        continue 'main;
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

        let parsing_result = parse(tokens.as_slice(), &[], &mut parser_settings);

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

        // make module
        let module = context.create_module("tmp");

        println!("> Full AST : \n{ast:#?}");

        // res.funcs = 
        //     Compiler::compile_ast(&context, &builder, &fpm, &module, &ast)
        //     .iter()
        //     .map(|f| f.expect("Expression failed to compile."))
        //     .collect::<Vec<FunctionValue>>();

        stdout.flush().unwrap();
        input.clear();
        res.print(flags);
    }
}
