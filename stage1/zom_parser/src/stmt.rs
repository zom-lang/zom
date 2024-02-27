//! Module responsible for parsing statement.
use crate::{block::Block, expr::Expression, prelude::*};

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
    BlockStmt(Block),
}

impl Parse for Stmt {
    type Output = Statement;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        match &parser.last().tt {
            T::If => parse_if_else_stmt(parser),
            T::OpenBrace => parse_block_stmt(parser),
            _ => parse_expr_stmt(parser),
        }
    }
}

pub fn parse_expr_stmt(parser: &mut Parser) -> ParsingResult<Statement> {
    let mut parsed_tokens = Vec::new();

    let expr = parse_try!(parser => Expression, parsed_tokens);
    let span = expr.span.clone();

    Good(
        Statement {
            stmt: Stmt::ExprStmt(expr),
            span,
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

    let block = parse_try!(parser => Block, parsed_tokens);
    let span = block.span.clone();

    Good(
        Statement {
            stmt: Stmt::BlockStmt(block),
            span,
        },
        parsed_tokens,
    )
}
