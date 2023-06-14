use mona_fe::{
    parser::{
        parse, ASTNode,
        Expression::{self, BinaryExpr},
        Function, ParserSettings, Prototype,
    },
    token::{Token::*, OP_PLUS},
};

#[test]
fn parser_test() -> Result<(), String> {
    let toks = vec![
        Func,
        Ident("foo".to_string()),
        OpenParen,
        Ident("a".to_string()),
        CloseParen,
        Int(104),
        Operator(OP_PLUS.to_string()),
        Ident("a".to_string()),
    ];

    let (ast, toks_rest) = parse(&toks, &[], &mut ParserSettings::default())?;

    if !toks_rest.is_empty() {
        panic!("There is a rest.")
    }

    let expected = vec![ASTNode::FunctionNode(Function {
        prototype: Prototype {
            name: "foo".to_string(),
            args: vec!["a".to_string()],
        },
        body: Some(BinaryExpr(
            OP_PLUS.to_string(),
            Box::new(Expression::LiteralExpr(104)),
            Box::new(Expression::VariableExpr("a".to_string())),
        )),
        is_anonymous: false,
    })];

    assert_eq!(ast, expected);

    Ok(())
}
