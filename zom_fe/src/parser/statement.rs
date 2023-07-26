//! This module contains parsing for statements.

use std::ops::RangeInclusive;

use zom_common::token::{Token, TokenType::Return};

use crate::{parse_try, parser::expr::parse_expr, impl_span};

use super::{expr::Expression, types::Type, ParserSettings, ParsingContext, PartParsingResult};

use crate::parser::PartParsingResult::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Statement {
    pub stmt: Stmt,
    pub span: RangeInclusive<usize>,
}

impl_span!(Statement);

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Expr(Expression),
    Var {
        name: String,
        ty: Option<Type>,
        expr: Expression,
    },
    Const {
        name: String,
        ty: Option<Type>,
        expr: Expression,
    },
    Return,
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

pub(super) fn parse_statement(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Statement> {
    let mut parsed_tokens = vec![];
    match tokens.last() {
        Some(Token {
            tt: Return,
            span: _,
        }) => todo!("Implement the return statement"),
        None => NotComplete,
        _ => {
            let expr = parse_try!(
                parse_expr,
                tokens,
                settings,
                context,
                parsed_tokens
            );
            let expr_span = expr.span.clone();

            Good(
                Statement { stmt: Stmt::Expr(expr), span: expr_span },
                parsed_tokens,
            )
        },
    }
}
