use anyhow::anyhow;
use std::error::Error;
use std::io::{stdout, Write};
use zom_fe::lexer::Lexer;
use zom_fe::parser::{parse, ParserSettings, ParsingContext};

use crate::{ExitStatus, err};

pub fn dev() -> Result<ExitStatus, Box<dyn Error>> {
    println!("Development command.\n");

    let mut buffer =
        String::from("func foo(bar: i16, baz: str) { foo(test, test); foo = 12; foo ");

    print!("input: ");
    stdout().flush().expect("ERR: Flush the output failed.");
    // match io::stdin().read_line(&mut buffer) {
    //     Ok(_) => {}
    //     Err(err) => return Err(anyhow!(format!("{}", err))),
    // }

    buffer = buffer.replace("\\n", "\n");

    println!("buffer = {:?}\n", buffer);

    let mut lexer = Lexer::new(buffer.as_str(), "<dev_cmd>.zom".to_string());

    let tokens = match lexer.make_tokens() {
        Ok(t) => t,
        Err(err) => return Err(err!(fmt "{}", err)),
    };

    println!("tokens = {tokens:#?}");

    let mut parse_context = ParsingContext::new("<dev_cmd>.zom".to_owned(), buffer, tokens.clone());

    let ast_result = parse(
        tokens.as_slice(),
        &[],
        &mut ParserSettings::default(),
        &mut parse_context,
    );

    match ast_result {
        Ok((ast, rest_toks)) => {
            println!("ast = {:#?}", ast);
            println!("toks_rest = {:?}", rest_toks);
        }
        Err(err) => return Err(err!(fmt "{}", err)),
    }

    Ok(ExitStatus::Success)
}
