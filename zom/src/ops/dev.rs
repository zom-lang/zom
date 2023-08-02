use anyhow::anyhow;
use std::io::{
    stdout,
    Write,
};
use zom_common::error::{Position, ZomError};
use zom_fe::lexer::Lexer;
use zom_fe::parser::{parse, ParserSettings, ParsingContext};

use crate::ExitStatus;

pub fn dev() -> Result<ExitStatus, anyhow::Error> {
    println!("Development command.\n");

//     let filetext = r#"func foo(test: bar) void {
//     @print("test");
// }"#.to_owned();

    // let err = ZomError::new(
    //     Some(Position::try_from_range(
    //         0,
    //         0..=3,
    //         filetext.clone(),
    //         "src/main.zom".to_owned()
    //     ).unwrap()),
    //     "expected `,` found keyword `func`".to_owned(),
    //     false,
    //     Some("You could ...".to_string()),
    //     vec!("Some very usefull notes!".to_owned())
    // );
    // println!("{err}");

    // let err2 = ZomError::new(
    //     Some(Position::try_from_range(
    //         0,
    //         0..=47,
    //         filetext,
    //         "src/main.zom".to_owned(),
    //     ).unwrap()),
    //     "the name `test` is defined multiple times".to_owned(),
    //     false,
    //     None,
    //     vec!("`foo` must be defined only once in the value namespace of this module".to_owned())
    // );
    // println!("{err2}");

    // let err3 = ZomError::new(
    //     None,
    //     "internal compiler error".to_owned(),
    //     false,
    //     None,
    //     vec!(
    //                 "the compiler unexpectedly panicked. this is a bug.",
    //                 "we would appreciate a bug report: https://github.com/zom-lang/zom/issues/new?labels=bug (TODO: change the link to the template.)", // TODO: When issue #46 is finished, replace with the link to the template.
    //                 format!("zomc {} ({} {}) running on {}",
    //                         env!("CARGO_PKG_VERSION"),
    //                         &env!("GIT_HASH")[..7],
    //                         env!("COMPILED_DATE"),
    //                         env!("TARGET_TRIPLE")).as_str())
    //                 .iter()
    //                 .map(|v| v.to_string())
    //                 .collect()
    // );
    // println!("\n*HERE RUST_BACKTRACE*\n\n{err3}");

    let mut buffer = String::from("func foo(bar: i16, baz: str) {(foo(test, test)}");

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
        Err(err) => return Err(anyhow!(format!("{}", err))),
    };

    // println!("tokens = {tokens:#?}");

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
        Err(err) => return Err(anyhow!(format!("{}", err))),
    }

    Ok(ExitStatus::Success)
}
