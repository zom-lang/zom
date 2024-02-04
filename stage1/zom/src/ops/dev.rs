use std::error::Error;
use std::fs;
use std::io::{self, stdout, Write};
use std::path::PathBuf;
use termcolor::ColorChoice;
use zom_lexer::Lexer;
use zom_parser::{parse, ParserSettings, ParsingContext};

use zom_errors::prelude::*;

use crate::{err, ExitStatus};

pub fn dev() -> Result<ExitStatus, Box<dyn Error>> {
    println!("Development command.\n");

    #[allow(unused_assignments)]
    let mut path =
        // String::from("func foo(bar: i16, baz: str) void { foo(test, test); foo = 999 + 9 / 4; foo } extern foo_c(boobar: u32) void;");
        PathBuf::new();

    print!("path: ");
    stdout().flush().expect("ERR: Flush the output failed.");
    let mut path_buf = "".to_owned();
    match io::stdin().read_line(&mut path_buf) {
        Ok(_) => path = PathBuf::from(path_buf),
        Err(err) => return err!(fmt "{}", err),
    }

    if path.to_str().unwrap().trim() == "" {
        path = PathBuf::from("example/test.zom");
    }

    let buffer = fs::read_to_string(&path).expect("Should have been able to read the file");

    println!("file path = {}", path.display());
    println!("buffer = \\\n{}\n\\-> {}\n", buffer, buffer.len());

    let mut lctx = LogContext::new(buffer.clone(), path.clone(), ColorChoice::Always);

    let mut lexer = Lexer::new(&buffer, &path, &mut lctx);

    let tokens = match lexer.lex() {
        FinalRes::Ok(t, _) => t,
        FinalRes::Err(_) => {
            lctx.print();
            return err!(fmt "");
        }
    };

    for t in &tokens {
        print!("{:?}", t);
        println!(" -> {:?}", buffer.get(t.span.clone()));
    }

    println!("\n~~~  SEPARTOR  ~~~");

    let parse_context = ParsingContext::new("<dev_cmd>.zom".to_owned(), buffer);

    let ast_result = parse(
        tokens.as_slice(),
        &[],
        &mut ParserSettings::default(),
        parse_context,
    );

    match ast_result {
        Ok((ast, rest_toks)) => {
            println!("ast = {:#?}", ast);
            println!("toks_rest = {:?}", rest_toks);
        }
        Err(errs) => {
            let mut err = "".to_owned();
            for error in errs {
                err += format!("{}\n", error).as_str();
            }
            return err!(fmt "{}", err);
        }
    }

    lctx.print();
    Ok(ExitStatus::Success)
}
