use std::{
    fs::{self, File},
    io::Write,
    mem,
    path::PathBuf,
};

use anyhow::anyhow;
use inkwell::{context::Context, passes::PassManager, values::AnyValue};
use mona_codegen::gen::CodeGen;
use mona_compiler::compiler::Compiler;
use mona_fe::{
    lexer::Lexer,
    parser::{parse, ParserSettings},
};

use crate::ExitStatus;

#[derive(clap::Args, Debug, Clone)]
pub struct Args {
    /// Path to the Mona source file
    source_file: PathBuf,

    /// Path to where the object file will go
    #[clap(short, long)]
    output_file: Option<PathBuf>,

    /// LLVM level of optimization (not implemented yet)
    #[clap(short = 'O', long, default_value_t = 2)]
    // TODO: Change this to the actual things later.
    optimization_level: usize,

    /// Emits IR instead of a *.o
    #[clap(long, short, action = clap::ArgAction::SetTrue)]
    emit_ir: bool,

    /// Print verbose ouput if enabled.
    #[clap(long, short = 'V', action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

pub fn build(mut args: Args) -> Result<ExitStatus, anyhow::Error> {
    // default ouput_file to `output.o`, it's where because with `default_value_t`, that doesn't work.
    if args.output_file.is_none() {
        args.output_file = if args.emit_ir {
            Some(PathBuf::from(r"output.ll"))
        } else {
            Some(PathBuf::from(r"output.o"))
        };
    }

    let source = match fs::read_to_string(mem::take(&mut args.source_file)) {
        Ok(src) => src,
        Err(_) => return Err(anyhow!("Error while trying to read the source file.")),
    };

    let mut lexer = Lexer::new(
        source.as_str(),
        args.source_file.to_str().unwrap().to_owned(),
    );

    let tokens = match lexer.make_tokens() {
        Ok(src) => src,
        Err(_) => return Err(anyhow!("Error while trying to read the source file.")),
    };

    args.verbose.then(|| {
        println!("[+] Successfully lexes the input.");
    });

    let parse_result = parse(tokens.as_slice(), &[], &mut ParserSettings::default());

    let ast;

    match parse_result {
        Ok((parsed_ast, rest)) => {
            if rest.is_empty() {
                ast = parsed_ast;
            } else {
                return Err(anyhow!("There is rest after parsing."));
            }
        }
        Err(_) => return Err(anyhow!("Parsing error occurs.")),
    }

    args.verbose.then(|| {
        println!("[+] Successfully parsed the tokens.");
    });

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

    let module = context.create_module(mem::take(
        &mut args.source_file.as_mut_os_str().to_str().unwrap(),
    ));

    let compile_res = CodeGen::compile_ast(&context, &builder, &fpm, &module, &ast);

    match compile_res {
        Ok(funcs) => {
            args.verbose.then(|| {
                println!("[+] Successfully generate the code.");
            });
            if args.emit_ir {
                for fun in funcs {
                    let str = fun.print_to_string();
                    match args.output_file {
                        Some(ref path) => {
                            let mut file = File::create(path).expect("Couldn't open the file");
                            file.write(str.to_bytes()).expect("Could write to the file");

                            println!("Wrote the result to {:?}!", path);
                        }
                        None => return Err(anyhow!("Couldn't unwrap the file path")),
                    }
                }
            } else {
                match args.output_file {
                    Some(ref path) => {
                        Compiler::compile_default(module, path)
                            .expect("Couldn't compile to object file");
                        println!("Wrote result to {:?}!", path);
                    }
                    None => return Err(anyhow!("Couldn't unwrap the file path")),
                }
            }
        }
        Err(_) => return Err(anyhow!("Error was occur when trying to generate the code")),
    }

    Ok(ExitStatus::Success)
}
