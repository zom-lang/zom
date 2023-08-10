use std::error::Error;
use std::io::{stdout, Write};
use zom_fe::lexer::Lexer;
use zom_fe::parser::{parse, ParserSettings, ParsingContext};

use crate::{err, ExitStatus};

pub fn dev() -> Result<ExitStatus, Box<dyn Error>> {
    println!("Development command.\n");

    let buffer =
        // String::from("func foo(bar: i16, baz: str) void { foo(test, test); foo = 999 + 9 / 4; foo } extern foo_c(boobar: u32) void;");
        String::from(r#" "test " 't' "#);

    print!("input: ");
    stdout().flush().expect("ERR: Flush the output failed.");
    // match io::stdin().read_line(&mut buffer) {
    //     Ok(_) => {}
    //     Err(err) => return Err(err!("{}", err)),
    // }

    println!("buffer = {:?}\n", buffer);

    let mut lexer = Lexer::new(buffer.as_str(), "<dev_cmd>.zom".to_string());

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
