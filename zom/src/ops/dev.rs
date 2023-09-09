use std::error::Error;
use std::fs;
use std::io::{self, stdout, Write};
use std::path::PathBuf;
use zom_fe::lexer::Lexer;
use zom_fe::parser::{parse, ParserSettings, ParsingContext};

use crate::{err, ExitStatus};

pub fn dev() -> Result<ExitStatus, Box<dyn Error>> {
    println!("Development command.\n");

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

    let buffer = r#"func "test\Ftest\Ftest""#.to_string();
    // fs::read_to_string(&path).expect("Should have been able to read the file");

    println!("file path = {}", path.display());
    println!("buffer = \\\n{}\n\\-> {}\n", buffer, buffer.len());

    let mut lexer = Lexer::new(&buffer, &path);

    let tokens = match lexer.make_tokens() {
        Ok(t) => t,
        Err(errs) => {
            let mut err = "".to_owned();
            for error in errs {
                err += format!("{}\n", error).as_str();
            }
            return err!(fmt "{}", err);
        }
    };

    println!("tokens = {tokens:#?}");

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

    Ok(ExitStatus::Success)
}
