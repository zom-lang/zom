use std::{
    error::Error,
    fs,
    // mem,
    path::PathBuf,
};

use inkwell::{context::Context, passes::PassManager};
// use zom_codegen::gen::CodeGen;
// use zom_compiler::compiler::Compiler;
use zom_fe::{
    lexer::Lexer,
    parser::{parse, ParserSettings, ParsingContext},
};

use crate::{err, ExitStatus};

#[derive(clap::Args, Debug, Clone)]
pub struct Args {
    /// Path to the Zom source file
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

pub fn build(mut args: Args) -> Result<ExitStatus, Box<dyn Error>> {
    // default ouput_file to `output.o`, it's where because with `default_value_t`, that doesn't work.
    if args.output_file.is_none() {
        args.output_file = if args.emit_ir {
            Some(PathBuf::from(r"output.ll"))
        } else {
            Some(PathBuf::from(r"output.o"))
        };
    }

    let source = match fs::read_to_string(&mut args.source_file) {
        Ok(src) => src,
        Err(_) => return err!("Error while trying to read the source file."),
    };

    let mut lexer = Lexer::new(
        source.as_str(),
        args.source_file.to_str().unwrap().to_owned(),
    );

    let tokens = match lexer.make_tokens() {
        Ok(src) => src,
        Err(err) => return err!(fmt "\n{:?}\n", err),
    };

    args.verbose.then(|| {
        println!("[+] Successfully lexes the input.");
    });

    let parse_context = ParsingContext::new(args.source_file.to_str().unwrap().to_owned(), source);

    let parse_result = parse(
        tokens.as_slice(),
        &[],
        &mut ParserSettings::default(),
        parse_context,
    );

    let _ast;

    match parse_result {
        Ok((parsed_ast, rest)) => {
            if rest.is_empty() {
                _ast = parsed_ast;
            } else {
                return err!("There is rest after parsing.");
            }
        }
        Err(err) => return err!(fmt "\n{:?}\n", err),
    }

    args.verbose.then(|| {
        println!("[+] Successfully parsed the tokens.");
    });

    let context = Context::create();
    let module = context.create_module(args.source_file.to_str().unwrap());
    let _builder = context.create_builder();

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

    todo!()
    // let module = context.create_module(mem::take(
    //     &mut args.source_file.as_mut_os_str().to_str().unwrap(),
    // ));

    // let gen_res = CodeGen::compile_ast(&context, &builder, &fpm, &module, &ast);

    // match gen_res {
    //     Ok(_) => {
    //         args.verbose.then(|| {
    //             println!("[+] Successfully generate the code.");
    //         });
    //         if args.emit_ir {
    //             match module.print_to_file(args.output_file.clone().unwrap().as_path()) {
    //                 Ok(_) => {}
    //                 Err(err) => return Err(anyhow!(format!("{}", err))),
    //             }
    //             args.verbose.then(|| {
    //                 println!("Wrote the result to {:?}!", args.output_file.unwrap());
    //             });
    //             return Ok(ExitStatus::Success);
    //         }
    //         match args.output_file {
    //             Some(ref path) => {
    //                 Compiler::compile_default(module, path)
    //                     .expect("Couldn't compile to object file");
    //                 args.verbose.then(|| {
    //                     println!("[+] Successfully compile the code.");
    //                 });
    //                 println!("Wrote result to {:?}!", path);
    //                 Ok(ExitStatus::Success)
    //             }
    //             None => Err(anyhow!("Couldn't unwrap the file path")),
    //         }
    //     }
    //     Err(err) => Err(anyhow!(format!("{}", err))),
    // }
}
