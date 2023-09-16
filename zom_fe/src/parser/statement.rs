//! This module contains parsing for statements.

use std::ops::Range;

use zom_common::token::{Token, TokenType::*};

use crate::{
    impl_span, parse_try,
    parser::{expr::parse_expr, symbol::parse_symbol},
    token_parteq,
};

use super::{expr::Expression, symbol::Symbol, ParserSettings, ParsingContext, PartParsingResult};

use crate::parser::PartParsingResult::*;

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
    Return { expr: Option<Expression> },
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
        Some(Token { tt: Return, .. }) => parse_return(tokens, settings, context),
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

pub fn parse_return(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Statement> {
    // eat Return keyword
    let mut parsed_tokens: Vec<Token> = vec![tokens.last().unwrap().clone()];
    tokens.pop();

    let start = parsed_tokens.last().unwrap().span.start;

    let expr = if token_parteq!(no_opt tokens.last().unwrap(), SemiColon) {
        None
    } else {
        Some(parse_try!(
            parse_expr,
            tokens,
            settings,
            context,
            parsed_tokens
        ))
    };

    let end = parsed_tokens.last().unwrap().span.end;
    Good(
        Statement {
            stmt: Stmt::Return { expr },
            span: start..end,
        },
        parsed_tokens,
    )
}
