//! This module contains the parsing for expressions.

use zom_common::error::parser::UnexpectedTokenError;
use zom_common::token::Token;
use zom_common::token::*;

use crate::{FromContext, expect_token, parse_try};

use self::Expression::{BinaryExpr, CallExpr, BlockExpr, LiteralExpr, VariableExpr};

use crate::parser::PartParsingResult::{Bad, Good, NotComplete};

use crate::parser::PartParsingResult;

use super::{ParserSettings, ParsingContext, error};

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    LiteralExpr(i32),
    VariableExpr(String),
    BinaryExpr {
        op: String,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    CallExpr(String, Vec<Expression>),
    BlockExpr {
        exprs: Vec<Expression>,
    },
}

impl Expression {
    pub fn is_semicolon_needed(&self) -> bool {
        match self {
            &LiteralExpr(_) => true,
            &VariableExpr(_) => true,
            &BinaryExpr {
                op: _,
                rhs: _,
                lhs: _,
            } => true,
            &CallExpr(_, _) => true,
            &BlockExpr { exprs: _ } => false,
        }
    }
}

pub(super) fn parse_primary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    match tokens.last() {
        Some(&Ident(_)) => parse_ident_expr(tokens, settings, context),
        Some(&Int(_)) => parse_literal_expr(tokens, settings, context),
        Some(&OpenParen) => parse_parenthesis_expr(tokens, settings, context),
        Some(&OpenBrace) => parse_block_expr(tokens, settings, context),
        None => NotComplete,
        tok => error(Box::new(UnexpectedTokenError::from_context(
            context,
            format!("unknow token when expecting an expression, found {:?}", tok),
            tokens.last().unwrap().clone(),
        ))),
    }
}

pub(super) fn parse_ident_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        context,
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        // "identificator expected"
        error(Box::new(UnexpectedTokenError::from_context(
            context,
            "identificator expected".to_owned(),
            tokens.last().unwrap().clone()
        )))
    );

    expect_token!(
        context,
        [OpenParen, OpenParen, ()]
        else {return Good(VariableExpr(name), parsed_tokens)}
        <= tokens, parsed_tokens);

    let mut args = Vec::new();
    loop {
        expect_token!(
            context,
            [CloseParen, CloseParen, break]
            else {
                args.push(parse_try!(parse_expr, tokens, settings, context, parsed_tokens));
            }
            <= tokens, parsed_tokens
        );

        expect_token!(
            context, [
            Comma, Comma, {};
            CloseParen, CloseParen, break
        ] <= tokens,
             parsed_tokens,
            error(
                Box::new(UnexpectedTokenError::from_context(
                    context,
                    "Expected ',' in function call"
                        .to_owned(),
                    tokens.last().unwrap().clone()
                ))
            )
        );
    }

    Good(CallExpr(name, args), parsed_tokens)
}

pub(super) fn parse_literal_expr(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let value = expect_token!(
        context,
        [Int(val), Int(val), val] <= tokens,
        parsed_tokens,
        // "literal expected"
        error(Box::new(UnexpectedTokenError::from_context(
            context,
            "Literal expected".to_owned(),
            tokens.last().unwrap().clone()
        )))
    );

    Good(LiteralExpr(value), parsed_tokens)
}

pub(super) fn parse_parenthesis_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat the opening parenthesis
    tokens.pop();
    let mut parsed_tokens = vec![OpenParen];

    let expr = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);

    expect_token!(
        context,
        [CloseParen, CloseParen, ()] <= tokens,
        parsed_tokens,
        // "')' expected"
        error(Box::new(UnexpectedTokenError::from_context(
            context,
            "Expected ')' in parenthesis expression".to_owned(),
            tokens.last().unwrap().clone()
        )))
    );

    Good(expr, parsed_tokens)
}

pub(super) fn parse_block_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat the opening brace
    tokens.pop();
    let mut parsed_tokens = vec![OpenBrace];

    let mut exprs = vec![];

    while Some(&CloseBrace) != tokens.last() {
        let expr = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);
        let is_semi_needed = expr.is_semicolon_needed();

        exprs.push(expr);

        if is_semi_needed {
            expect_token!(
                context,
                [SemiColon, SemiColon, ()] <= tokens,
                parsed_tokens,
                // "';' expected"
                error(Box::new(UnexpectedTokenError::from_context(
                    context,
                    "Expected ';'".to_owned(),
                    tokens.last().unwrap().clone()
                )))
            );
        }
    }

    expect_token!(
        context,
        [CloseBrace, CloseBrace, ()] <= tokens,
        parsed_tokens,
        error(Box::new(UnexpectedTokenError::from_context(
            context,
            "Expected '}'".to_owned(),
            tokens.last().unwrap().clone()
        )))
    );

    Good(Expression::BlockExpr { exprs }, parsed_tokens)
}

pub(super) fn parse_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();
    let lhs = parse_try!(parse_primary_expr, tokens, settings, context, parsed_tokens);
    let expr = parse_try!(
        parse_binary_expr,
        tokens,
        settings,
        context,
        parsed_tokens,
        0,
        &lhs
    );
    Good(expr, parsed_tokens)
}

pub(super) fn parse_binary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
    expr_precedence: i32,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    // start with LHS value
    let mut result = lhs.clone();
    let mut parsed_tokens = Vec::new();

    loop {
        // continue until the current token is not an operator
        // or it is an operator with precedence lesser than expr_precedence
        let (operator, precedence) = match tokens.last() {
            Some(Operator(op)) => match settings.operator_precedence.get(op) {
                Some(pr) if *pr >= expr_precedence => (op.clone(), *pr),
                None => {
                    return error(Box::new(UnexpectedTokenError::from_context(
                        context,
                        "Unknown operator found".to_owned(),
                        tokens.last().unwrap().clone(),
                    )))
                }
                _ => break,
            },
            _ => break,
        };
        tokens.pop();
        parsed_tokens.push(Operator(operator.clone()));

        // parse primary RHS expression
        let mut rhs = parse_try!(parse_primary_expr, tokens, settings, context, parsed_tokens);

        // parse all the RHS operators until their precedence is
        // bigger than the current one
        loop {
            let binary_rhs = match tokens.last().cloned() {
                Some(Operator(ref op)) => match settings.operator_precedence.get(op).copied() {
                    Some(pr) if pr > precedence => {
                        parse_try!(
                            parse_binary_expr,
                            tokens,
                            settings,
                            context,
                            parsed_tokens,
                            pr,
                            &rhs
                        )
                    }
                    None => {
                        return error(Box::new(UnexpectedTokenError::from_context(
                            context,
                            "unknown operator found".to_owned(),
                            tokens.last().unwrap().clone(),
                        )))
                    }
                    _ => break,
                },
                _ => break,
            };

            rhs = binary_rhs;
        }

        // merge LHS and RHS
        result = BinaryExpr {
            op: operator,
            lhs: Box::new(result),
            rhs: Box::new(rhs),
        };
    }

    Good(result, parsed_tokens)
}