use std::path::Path;

use zom_common::error::ZomError;
use zom_fe::lexer::Lexer;

#[test]
fn test_from_range() {
    let buffer = "^ ^ ^ ^ ^ 987654321234567891234 ^";

    let mut lexer = Lexer::new(buffer, Path::new("tests.zom"));

    let res: Vec<ZomError> = lexer.make_tokens().unwrap_err();

    let mut err = "".to_owned();
    for error in res {
        err += format!("{}\n", error).as_str();
    }
    println!("{}", err);
}
