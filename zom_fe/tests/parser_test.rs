use zom_common::token::{Token::*, OP_PLUS};
use zom_fe::parser::{
    parse, ASTNode,
    Expression::{self, BinaryExpr, VariableExpr},
    Function, ParserSettings, Prototype, ParsingContext,
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

    let mut parse_context = ParsingContext::new("<tests>.zom".to_string(), "func foo(a) 104 + a".to_string());

    let (ast, toks_rest) = parse(&toks, &[], &mut ParserSettings::default(), &mut parse_context)?;

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
    ];

    let mut parse_context = ParsingContext::new(
        "<tests>.zom".to_string(), 
    "func foo(a, b, c, d, e, f, g) a + b + c + d + e + f + g".to_string()
    );


    let (ast, toks_rest) = parse(&toks, &[], &mut ParserSettings::default(), &mut parse_context)?;

    if !toks_rest.is_empty() {
        panic!("There is a rest.")
    }

    let expected = vec![ASTNode::FunctionNode(Function {
        prototype: Prototype {
            name: "foo".to_string(),
            args: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "f".to_string(),
                "g".to_string(),
            ],
        },
        body: Some(BinaryExpr(
            OP_PLUS.to_owned(),
            Box::new(BinaryExpr(
                OP_PLUS.to_owned(),
                Box::new(BinaryExpr(
                    OP_PLUS.to_owned(),
                    Box::new(BinaryExpr(
                        OP_PLUS.to_owned(),
                        Box::new(BinaryExpr(
                            OP_PLUS.to_owned(),
                            Box::new(BinaryExpr(
                                OP_PLUS.to_owned(),
                                Box::new(VariableExpr("a".to_string())),
                                Box::new(VariableExpr("b".to_string())),
                            )),
                            Box::new(VariableExpr("c".to_string())),
                        )),
                        Box::new(VariableExpr("d".to_string())),
                    )),
                    Box::new(VariableExpr("e".to_string())),
                )),
                Box::new(VariableExpr("f".to_string())),
            )),
            Box::new(VariableExpr("g".to_string())),
        )),
        is_anonymous: false,
    })];

    assert_eq!(ast, expected);

    Ok(())
}

#[test]
#[should_panic]
fn error_parser_test() {
    let toks = vec![
        Func,
        OpenParen,
        Ident("a".to_string()),
        CloseParen,
        Int(104),
        Operator(OP_PLUS.to_string()),
        Ident("a".to_string()),
    ];

    let mut parse_context = ParsingContext::new(
        "<tests>.zom".to_string(), 
    "func foo(a) 104 + a".to_string()
    );

    let (ast, toks_rest) = match parse(&toks, &[], &mut ParserSettings::default(), &mut parse_context) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            panic!()
        }
    };

    if !toks_rest.is_empty() {
        eprintln!("There is a rest.");
        return;
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
}
