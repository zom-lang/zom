//! This module contains parsing for statements.

use zom_common::token::{Token, TokenType::Return};

use crate::{parse_try, parser::expr::parse_expr};

use super::{expr::Expression, ParserSettings, ParsingContext, PartParsingResult, types::Type};

use crate::parser::PartParsingResult::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Expr(Expression),
    Var {
        name: String,
        type_: Option<Type>,
        expr: Expression,
    },
    Const {
        name: String,
        type_: Option<Type>,
        expr: Expression,
    },
    Return,
}

impl Statement {
    pub fn is_semi_need(&self) -> bool {
        match self {
            Self::Expr(e) => e.is_semicolon_needed(),
            _ => true,
        }
    }
}

pub(super) fn parse_statement(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Statement> {
    let mut parsed_tokens = vec![];
    match tokens.last() {
        Some(Token { tt: Return, span: _ }) => todo!("Implement the return statement"),
        None => NotComplete,
        _ => Good(Statement::Expr(parse_try!(parse_expr, tokens, settings, context, parsed_tokens)), parsed_tokens),
    }
}
