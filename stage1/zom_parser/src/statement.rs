//! This module contains parsing for statements.

use crate::prelude::*;

use crate::{expr::{parse_expr, Expression}, symbol::{parse_symbol, Symbol}};

#[derive(PartialEq, Clone, Debug)]
pub struct Statement {
    pub stmt: Stmt,
    pub span: Range<usize>,
}

impl_span!(Statement);

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Expr(Expression),
    Symbol(Symbol),
}

impl Stmt {
    pub fn is_semi_need(&self) -> bool {
        match self {
            Self::Expr(e) => e.is_semicolon_needed(),
            _ => true,
        }
    }
}

impl Statement {
    pub fn is_semi_need(&self) -> bool {
        self.stmt.is_semi_need()
    }
}

pub fn parse_statement(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Statement> {
    let mut parsed_tokens = vec![];
    match tokens.last() {
        Some(Token {
            tt: Var | Const, ..
        }) => {
            let symbol = parse_try!(parse_symbol, tokens, settings, context, parsed_tokens);
            let syb_span = symbol.span.clone();

            Good(
                Statement {
                    stmt: Stmt::Symbol(symbol),
                    span: syb_span,
                },
                parsed_tokens,
            )
        }
        None => NotComplete,
        _ => {
            let expr = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);
            let expr_span = expr.span.clone();

            Good(
                Statement {
                    stmt: Stmt::Expr(expr),
                    span: expr_span,
                },
                parsed_tokens,
            )
        }
    }
}
