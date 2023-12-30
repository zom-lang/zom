//! This module contains parsing for statements.

use crate::prelude::*;

use crate::{
    expr::{parse_expr, Expression},
    symbol::{parse_symbol, Symbol},
};

#[derive(PartialEq, Clone, Debug)]
pub struct Statement {
    pub stmt: Stmt,
    pub span: Range<usize>,
}

impl_span!(Statement);

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Expr(Expression),
    SymbolDecl(Symbol),
}

pub fn parse_symbol_decl_stmt(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Statement> {
    let mut parsed_tokens = vec![];

    let symbol = parse_try!(parse_symbol, tokens, settings, context, parsed_tokens);
    let start = symbol.span.start;

    let tts = tokens.clone();
    let t = tts.last().unwrap();

    expect_token!(
        context,
        [SemiColon, SemiColon, ()] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![SemiColon], t.tt)
    );

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Statement {
            stmt: Stmt::SymbolDecl(symbol),
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_expression_stmt(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Statement> {
    let mut parsed_tokens = vec![];

    let expr = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);
    let start = expr.span.start;
    let mut end = expr.span.end;

    if expr.semicolon() || token_parteq!(tokens.last(), SemiColon) {
        let tts = tokens.clone();
        let t = tts.last().unwrap();

        expect_token!(
            context,
            [SemiColon, SemiColon, ()] <= tokens,
            parsed_tokens,
            err_et!(context, t, vec![SemiColon], t.tt)
        );
        end = parsed_tokens.last().unwrap().span.end;
    }

    Good(
        Statement {
            stmt: Stmt::Expr(expr),
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_statement(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Statement> {
    match tokens.last() {
        Some(Token {
            tt: Var | Const, ..
        }) => parse_symbol_decl_stmt(tokens, settings, context),
        None => NotComplete,
        _ => parse_expression_stmt(tokens, settings, context),
    }
}
