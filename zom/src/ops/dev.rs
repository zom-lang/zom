use anyhow::anyhow;
use zom_common::{reverse_lexer::reverse_lexe, token::*, error::parser::UnexpectedTokenError};

use crate::ExitStatus;

pub fn dev() -> Result<ExitStatus, anyhow::Error> {
    println!("Development command.");

    let tokens = vec![
        OpenParen,
        Ident("foo".to_owned()),
        Operator("&&".to_string()),
        Operator("+".to_string()),
        Operator("+".to_string()),
        Int(109),
        Ident("baz".to_owned()),
        Func,
        Int(10991),
    ];

    // the source code is very weird but it's normal
    let source_file = String::from(
r#"(foo && + +  109
    baz func 10991"#
    );

    let pos = reverse_lexe(7, tokens, source_file, "<DEV_TEST_REVERSE_LEXING.ZOM>".to_string());
    if pos.is_err() {
        let err = pos.err().unwrap();
        return Err(anyhow!(err));
    }
    let pos = pos.unwrap();

    println!("calculed pos = {:?}\n\n", pos);

    eprintln!("{}", UnexpectedTokenError::new(pos, "Unexpected token.".to_owned()));

    Ok(ExitStatus::Success)
}
