//! Module responsible for parsing statement.
use crate::{expr::Expression, prelude::*};

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
}

impl Parse for Stmt {
    type Output = Statement;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        match parser.last() {
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
