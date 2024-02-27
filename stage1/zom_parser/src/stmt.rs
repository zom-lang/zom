//! Module responsible for parsing statement.
use crate::{
    block::Block,
    expr::{Expression, ExpressionList},
    prelude::*,
};

#[derive(Debug)]
pub struct Statement {
    pub stmt: Stmt,
    pub span: Range<usize>,
}

impl Parse for Statement {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        Stmt::parse(parser)
    }
}

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expression),
    IfElseStmt {
        predicate: Expression,
        stmt_true: Box<Statement>,
        stmt_false: Option<Box<Statement>>,
    },
    BlockStmt {
        label: Option<String>,
        block: Block,
    },
    ReturnStmt(Option<Expression>),
    WhileStmt {
        label: Option<String>,
        ctrling_expr: Expression,
        loop_body: Block,
    },
    BreakStmt {
        label: Option<String>,
        expr: Option<Expression>,
    },
    ContinueStmt {
        label: Option<String>,
    },
    AssignementStmt {
        lhs: ExpressionList,
        rhs: ExpressionList,
    },
    ShortVarDecl {
        names: Vec<String>,
        exprs: Vec<Expression>,
    },
}

impl Parse for Stmt {
    type Output = Statement;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        match &parser.last().tt {
            T::If => parse_if_else_stmt(parser),
            T::OpenBrace => parse_block_stmt(parser),
            T::Return => parse_return_stmt(parser),
            T::Ident(_) if is_labeled_stmt(parser) => parse_labeled_stmt(parser),
            T::While => parse_while_stmt(parser),
            T::Break => parse_break_stmt(parser),
            T::Continue => parse_continue_stmt(parser),
            T::Ident(_) if is_short_var_decl(parser) => parse_short_var_decl(parser),
            _ => parse_expr_stmt(parser),
        }
    }
}

pub fn parse_expr_stmt(parser: &mut Parser) -> ParsingResult<Statement> {
    let mut parsed_tokens = Vec::new();

    let expr = parse_try!(parser => Expression, parsed_tokens);
    let start = span_toks!(start first parsed_tokens);
    if token_parteq!(parser.last(), T::Comma | T::Oper(Operator::Equal)) {
        let expr_list = parse_try!(fn; parser => ExpressionList::parse_with, parsed_tokens, expr);

        return parse_assignement_stmt(parser, expr_list, start);
    }

    let end = span_toks!(end parsed_tokens);

    Good(
        Statement {
            stmt: Stmt::ExprStmt(expr),
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_if_else_stmt(parser: &mut Parser) -> ParsingResult<Statement> {
    let mut parsed_tokens = Vec::new();
    expect_token!(parser => [T::If, ()], If, parsed_tokens);
    let start = span_toks!(start parsed_tokens);

    expect_token!(parser => [T::OpenParen, ()], OpenParen, parsed_tokens);
    let predicate = parse_try!(parser => Expression, parsed_tokens);
    expect_token!(parser => [T::CloseParen, ()], CloseParen, parsed_tokens);

    let stmt_true = Box::new(parse_try!(parser => Statement, parsed_tokens));

    let stmt_false = if token_parteq!(parser.last(), T::Else) {
        expect_token!(parser => [T::Else, ()], Else, parsed_tokens);
        Some(Box::new(parse_try!(parser => Statement, parsed_tokens)))
    } else {
        None
    };
    let end = span_toks!(end parsed_tokens);

    Good(
        Statement {
            stmt: Stmt::IfElseStmt {
                predicate,
                stmt_true,
                stmt_false,
            },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_block_stmt(parser: &mut Parser) -> ParsingResult<Statement> {
    let mut parsed_tokens = Vec::new();

    let label = if token_parteq!(parser.last(), T::Ident(_)) {
        let l = expect_token!(parser => [T::Ident(label), label.clone()], Ident, parsed_tokens);
        expect_token!(parser => [T::Colon, ()], Colon, parsed_tokens);
        Some(l)
    } else {
        None
    };

    let block = parse_try!(parser => Block, parsed_tokens);
    let start = span_toks!(start first parsed_tokens);
    let end = block.span.end;

    Good(
        Statement {
            stmt: Stmt::BlockStmt { label, block },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_return_stmt(parser: &mut Parser) -> ParsingResult<Statement> {
    let mut parsed_tokens = Vec::new();

    expect_token!(parser => [T::Return, ()], Return, parsed_tokens);
    let start = span_toks!(start parsed_tokens);

    let expr = if !parser.expr_end() {
        Some(parse_try!(parser => Expression, parsed_tokens))
    } else {
        None
    };

    let end = span_toks!(end parsed_tokens);

    Good(
        Statement {
            stmt: Stmt::ReturnStmt(expr),
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn is_labeled_stmt(parser: &Parser) -> bool {
    matches!(parser.end_nth(1).tt, T::Ident(_)) && matches!(parser.end_nth(2).tt, T::Colon)
}

pub fn parse_labeled_stmt(parser: &mut Parser) -> ParsingResult<Statement> {
    match &parser.end_nth(3).tt {
        T::While => parse_while_stmt(parser),
        T::OpenBrace => parse_block_stmt(parser),
        _ => Error(Box::new(ExpectedToken::from(
            parser.end_nth(3),
            PartAST::LabeledStmt,
        ))),
    }
}

pub fn parse_while_stmt(parser: &mut Parser) -> ParsingResult<Statement> {
    let mut parsed_tokens = Vec::new();

    let label = if token_parteq!(parser.last(), T::Ident(_)) {
        let l = expect_token!(parser => [T::Ident(label), label.clone()], Ident, parsed_tokens);
        expect_token!(parser => [T::Colon, ()], Colon, parsed_tokens);
        Some(l)
    } else {
        None
    };

    expect_token!(parser => [T::While, ()], While, parsed_tokens);
    let start = span_toks!(start first parsed_tokens);

    expect_token!(parser => [T::OpenParen, ()], OpenParen, parsed_tokens);
    let ctrling_expr = parse_try!(parser => Expression, parsed_tokens);
    expect_token!(parser => [T::CloseParen, ()], CloseParen, parsed_tokens);

    let loop_body = parse_try!(parser => Block, parsed_tokens);

    let end = span_toks!(end parsed_tokens);
    Good(
        Statement {
            stmt: Stmt::WhileStmt {
                label,
                ctrling_expr,
                loop_body,
            },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_break_stmt(parser: &mut Parser) -> ParsingResult<Statement> {
    let mut parsed_tokens = Vec::new();

    expect_token!(parser => [T::Break, ()], Break, parsed_tokens);
    let start = span_toks!(start parsed_tokens);

    let label = if token_parteq!(parser.last(), T::Colon) {
        expect_token!(parser => [T::Colon, ()], Colon, parsed_tokens);
        expect_token!(parser => [T::Ident(lab), Some(lab.clone())], Ident, parsed_tokens)
    } else {
        None
    };

    let expr = if !parser.expr_end() {
        Some(parse_try!(parser => Expression, parsed_tokens))
    } else {
        None
    };
    let end = span_toks!(end parsed_tokens);

    Good(
        Statement {
            stmt: Stmt::BreakStmt { label, expr },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_continue_stmt(parser: &mut Parser) -> ParsingResult<Statement> {
    let mut parsed_tokens = Vec::new();

    expect_token!(parser => [T::Continue, ()], Continue, parsed_tokens);
    let start = span_toks!(start parsed_tokens);

    let label = if token_parteq!(parser.last(), T::Colon) {
        expect_token!(parser => [T::Colon, ()], Colon, parsed_tokens);
        expect_token!(parser => [T::Ident(lab), Some(lab.clone())], Ident, parsed_tokens)
    } else {
        None
    };

    let end = span_toks!(end parsed_tokens);

    Good(
        Statement {
            stmt: Stmt::ContinueStmt { label },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_assignement_stmt(
    parser: &mut Parser,
    lhs: ExpressionList,
    start: usize,
) -> ParsingResult<Statement> {
    // TODO(Larsouille25): Add ability to use +=, etc..
    //                     a += 1
    let mut parsed_tokens = Vec::new();

    expect_token!(parser => [T::Oper(Operator::Equal), ()], T::Oper(Operator::Equal), parsed_tokens);

    let rhs = parse_try!(parser => ExpressionList, parsed_tokens);

    let end = span_toks!(end parsed_tokens);

    Good(
        Statement {
            stmt: Stmt::AssignementStmt { lhs, rhs },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn is_short_var_decl(parser: &Parser) -> bool {
    let mut i = 1;
    loop {
        if !token_parteq!(parser.end_nth(i), T::Ident(_)) {
            return false;
        }
        i += 1;
        match parser.end_nth(i).tt {
            T::Comma => {}
            T::Colon => return true,
            _ => return false,
        }
        i += 1;
    }
}

pub fn parse_short_var_decl(parser: &mut Parser) -> ParsingResult<Statement> {
    let mut parsed_tokens = Vec::new();

    let mut names = Vec::new();
    loop {
        names.push(expect_token!(parser => [T::Ident(name), name.clone()], Ident, parsed_tokens));
        expect_token!(parser => [T::Comma, ()] else { break }, parsed_tokens);
    }
    let start = span_toks!(start first parsed_tokens);

    expect_token!(parser => [T::Colon, ()], Colon, parsed_tokens);
    expect_token!(parser => [T::Oper(Operator::Equal), ()], T::Oper(Operator::Equal), parsed_tokens);
    // Do we wanna make `:=` an operator?

    let mut exprs = Vec::new();
    loop {
        exprs.push(parse_try!(parser => Expression, parsed_tokens));
        expect_token!(parser => [T::Comma, ()] else { break }, parsed_tokens);
    }
    let end = span_toks!(end parsed_tokens);

    Good(
        Statement {
            stmt: Stmt::ShortVarDecl { names, exprs },
            span: start..end,
        },
        parsed_tokens,
    )
}
