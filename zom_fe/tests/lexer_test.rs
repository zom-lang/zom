use std::error::Error;
use zom_fe::lexer::Lexer;

#[test]
fn test_operators_lexing() -> Result<(), Box<dyn Error>> {
    use zom_common::token::Token::Operator;
    use zom_common::token::*;
    let mut lexer = Lexer::new(
        "= + - * / % ^ == != > < => =< || &&",
        "test_operators_parsing".to_string(),
    );
    let toks = lexer.make_tokens()?;

    let expected = vec![
        Operator(OP_EQ.to_string()),
        Operator(OP_PLUS.to_string()),
        Operator(OP_MINUS.to_string()),
        Operator(OP_MUL.to_string()),
        Operator(OP_DIV.to_string()),
        Operator(OP_MOD.to_string()),
        Operator(OP_COMP_EQ.to_string()),
        Operator(OP_COMP_NE.to_string()),
        Operator(OP_COMP_GT.to_string()),
        Operator(OP_COMP_LT.to_string()),
        Operator(OP_COMP_GTE.to_string()),
        Operator(OP_COMP_LTE.to_string()),
        Operator(OP_OR.to_string()),
        Operator(OP_AND.to_string()),
    ];

    assert_eq!(toks, expected);

    Ok(())
}

#[test]
fn test_lexing() -> Result<(), Box<dyn Error>> {
    use zom_common::token::*;
    let mut lexer = Lexer::new(
        "func extern let () [] {} : ; , 123",
        "test_operators_parsing".to_string(),
    );
    let toks = lexer.make_tokens()?; // Float aren't currently tested because there is a bug

    let expected = vec![
        Func,
        Extern,
        Var,
        OpenParen,
        CloseParen,
        OpenBracket,
        CloseBracket,
        OpenBrace,
        CloseBrace,
        Colon,
        SemiColon,
        Comma,
        Int(123),
    ];

    assert_eq!(toks, expected);

    Ok(())
}
