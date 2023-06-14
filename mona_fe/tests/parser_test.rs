use mona_fe::{
    parser::{
        parse, ASTNode,
        Expression::{self, BinaryExpr},
        Function, ParserSettings, Prototype,
    },
    token::{Token::*, OP_PLUS},
};

#[test]
fn short_parser_test() -> Result<(), String> {
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

#[test]
fn long_parser_test() -> Result<(), String> {
    let toks = vec![
        Func,
        Ident("foo".to_owned()),
        OpenParen,
        Ident("a".to_owned()),
        Comma,
        Ident("b".to_owned()),
        Comma,
        Ident("c".to_owned()),
        Comma,
        Ident("d".to_owned()),
        Comma,
        Ident("e".to_owned()),
        Comma,
        Ident("f".to_owned()),
        Comma,
        Ident("g".to_owned()),
        Comma,
        Ident("h".to_owned()),
        Comma,
        Ident("i".to_owned()),
        Comma,
        Ident("j".to_owned()),
        Comma,
        Ident("k".to_owned()),
        Comma,
        Ident("l".to_owned()),
        Comma,
        Ident("m".to_owned()),
        Comma,
        Ident("n".to_owned()),
        Comma,
        Ident("o".to_owned()),
        Comma,
        Ident("p".to_owned()),
        Comma,
        Ident("q".to_owned()),
        Comma,
        Ident("r".to_owned()),
        Comma,
        Ident("s".to_owned()),
        Comma,
        Ident("t".to_owned()),
        Comma,
        Ident("u".to_owned()),
        Comma,
        Ident("v".to_owned()),
        Comma,
        Ident("w".to_owned()),
        Comma,
        Ident("x".to_owned()),
        Comma,
        Ident("y".to_owned()),
        Comma,
        Ident("z".to_owned()),
        CloseParen,
        Ident("a".to_owned()),
        Operator("+".to_owned()),
        Ident("b".to_owned()),
        Operator("+".to_owned()),
        Ident("c".to_owned()),
        Operator("+".to_owned()),
        Ident("d".to_owned()),
        Operator("+".to_owned()),
        Ident("e".to_owned()),
        Operator("+".to_owned()),
        Ident("f".to_owned()),
        Operator("+".to_owned()),
        Ident("g".to_owned()),
        Operator("+".to_owned()),
        Ident("h".to_owned()),
        Operator("+".to_owned()),
        Ident("i".to_owned()),
        Operator("+".to_owned()),
        Ident("j".to_owned()),
        Operator("+".to_owned()),
        Ident("k".to_owned()),
        Operator("+".to_owned()),
        Ident("l".to_owned()),
        Operator("+".to_owned()),
        Ident("m".to_owned()),
        Operator("+".to_owned()),
        Ident("n".to_owned()),
        Operator("+".to_owned()),
        Ident("o".to_owned()),
        Operator("+".to_owned()),
        Ident("p".to_owned()),
        Operator("+".to_owned()),
        Ident("q".to_owned()),
        Operator("+".to_owned()),
        Ident("r".to_owned()),
        Operator("+".to_owned()),
        Ident("s".to_owned()),
        Operator("+".to_owned()),
        Ident("t".to_owned()),
        Operator("+".to_owned()),
        Ident("u".to_owned()),
        Operator("+".to_owned()),
        Ident("v".to_owned()),
        Operator("+".to_owned()),
        Ident("w".to_owned()),
        Operator("+".to_owned()),
        Ident("x".to_owned()),
        Operator("+".to_owned()),
        Ident("y".to_owned()),
        Operator("+".to_owned()),
        Ident("z".to_owned()),
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
        body: Some(
            BinaryExpr(
                "+".to_owned(),
                BinaryExpr(
                    "+".to_owned(),
                    BinaryExpr(
                        "+".to_owned(),
                        BinaryExpr(
                            "+".to_owned(),
                            BinaryExpr(
                                "+".to_owned(),
                                BinaryExpr(
                                    "+".to_owned(),
                                    BinaryExpr(
                                        "+".to_owned(),
                                        BinaryExpr(
                                            "+".to_owned(),
                                            BinaryExpr(
                                                "+".to_owned(),
                                                BinaryExpr(
                                                    "+".to_owned(),
                                                    BinaryExpr(
                                                        "+".to_owned(),
                                                        BinaryExpr(
                                                            "+".to_owned(),
                                                            BinaryExpr(
                                                                "+".to_owned(),
                                                                BinaryExpr(
                                                                    "+".to_owned(),
                                                                    BinaryExpr(
                                                                        "+".to_owned(),
                                                                        BinaryExpr(
                                                                            "+".to_owned(),
                                                                            BinaryExpr(
                                                                                "+".to_owned(),
                                                                                BinaryExpr(
                                                                                    "+".to_owned(),
                                                                                    BinaryExpr(
                                                                                        "+".to_owned(),
                                                                                        BinaryExpr(
                                                                                            "+".to_owned(),
                                                                                            BinaryExpr(
                                                                                                "+".to_owned(),
                                                                                                BinaryExpr(
                                                                                                    "+".to_owned(),
                                                                                                    BinaryExpr(
                                                                                                        "+".to_owned(),
                                                                                                        BinaryExpr(
                                                                                                            "+".to_owned(),
                                                                                                            BinaryExpr(
                                                                                                                "+".to_owned(),
                                                                                                                VariableExpr(
                                                                                                                    "a".to_owned(),
                                                                                                                ),
                                                                                                                VariableExpr(
                                                                                                                    "b".to_owned(),
                                                                                                                ),
                                                                                                            ),
                                                                                                            VariableExpr(
                                                                                                                "c".to_owned(),
                                                                                                            ),
                                                                                                        ),
                                                                                                        VariableExpr(
                                                                                                            "d".to_owned(),
                                                                                                        ),
                                                                                                    ),
                                                                                                    VariableExpr(
                                                                                                        "e".to_owned(),
                                                                                                    ),
                                                                                                ),
                                                                                                VariableExpr(
                                                                                                    "f".to_owned(),
                                                                                                ),
                                                                                            ),
                                                                                            VariableExpr(
                                                                                                "g".to_owned(),
                                                                                            ),
                                                                                        ),
                                                                                        VariableExpr(
                                                                                            "h".to_owned(),
                                                                                        ),
                                                                                    ),
                                                                                    VariableExpr(
                                                                                        "i".to_owned(),
                                                                                    ),
                                                                                ),
                                                                                VariableExpr(
                                                                                    "j".to_owned(),
                                                                                ),
                                                                            ),
                                                                            VariableExpr(
                                                                                "k".to_owned(),
                                                                            ),
                                                                        ),
                                                                        VariableExpr(
                                                                            "l".to_owned(),
                                                                        ),
                                                                    ),
                                                                    VariableExpr(
                                                                        "m".to_owned(),
                                                                    ),
                                                                ),
                                                                VariableExpr(
                                                                    "n".to_owned(),
                                                                ),
                                                            ),
                                                            VariableExpr(
                                                                "o".to_owned(),
                                                            ),
                                                        ),
                                                        VariableExpr(
                                                            "p".to_owned(),
                                                        ),
                                                    ),
                                                    VariableExpr(
                                                        "q".to_owned(),
                                                    ),
                                                ),
                                                VariableExpr(
                                                    "r".to_owned(),
                                                ),
                                            ),
                                            VariableExpr(
                                                "s".to_owned(),
                                            ),
                                        ),
                                        VariableExpr(
                                            "t".to_owned(),
                                        ),
                                    ),
                                    VariableExpr(
                                        "u".to_owned(),
                                    ),
                                ),
                                VariableExpr(
                                    "v".to_owned(),
                                ),
                            ),
                            VariableExpr(
                                "w".to_owned(),
                            ),
                        ),
                        VariableExpr(
                            "x".to_owned(),
                        ),
                    ),
                    VariableExpr(
                        "y".to_owned(),
                    ),
                ),
                VariableExpr(
                    "z".to_owned(),
                ),
            ),
        ),
,
        is_anonymous: false,
    })];

    println!("ast = {ast:#?}");

    assert_eq!(ast, expected);

    Ok(())
}
