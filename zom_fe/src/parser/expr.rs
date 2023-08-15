//! This module contains the parsing for expressions.

use std::ops::RangeInclusive;

use zom_common::token::Token;
use zom_common::token::*;

use crate::{err_et, expect_token, impl_span, parse_try};

use self::Expr::{BinaryExpr, BlockExpr, CallExpr, LiteralExpr, VariableExpr};

use crate::parser::PartParsingResult::{Bad, Good, NotComplete};

use crate::parser::PartParsingResult;

use super::block::{parse_block_expr, Block};
use super::{ParserSettings, ParsingContext};

#[derive(PartialEq, Clone, Debug)]
pub struct Expression {
    pub expr: Expr,
    pub span: RangeInclusive<usize>,
}

impl_span!(Expression);

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    LiteralExpr(i32),
    VariableExpr(String),
    BinaryExpr {
        op: Operator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    CallExpr(String, Vec<Expression>),
    BlockExpr(Block),
}

impl Expression {
    pub fn is_semicolon_needed(&self) -> bool {
        !matches!(
            *self,
            Expression {
                expr: BlockExpr(_),
                ..
            }
        )
    }
}

pub fn parse_primary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    match tokens.last() {
        Some(Token { tt: Ident(_), .. }) => parse_ident_expr(tokens, settings, context),
        Some(Token { tt: Int(_), .. }) => parse_literal_expr(tokens, settings, context),
        Some(Token { tt: OpenParen, .. }) => parse_parenthesis_expr(tokens, settings, context),
        Some(Token { tt: OpenBrace, .. }) => parse_block_expr(tokens, settings, context),
        None => NotComplete,
        _ => err_et!(
            context,
            tokens.last().unwrap(),
            vec![Ident(String::new()), Int(0), OpenParen, OpenBrace],
            tokens.last().unwrap().tt
        ),
    }
}

pub fn parse_ident_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        context,
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Ident(String::new())],
            tokens.last().unwrap().tt
        )
    );

    let start = *parsed_tokens.last().unwrap().clone().span.start();

    let end = *parsed_tokens.last().unwrap().clone().span.end();

    expect_token!(
        context,
        [OpenParen, OpenParen, ()]
        else {return Good(
            Expression { expr: VariableExpr(name), span: start..=end },
            parsed_tokens)}
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

        let t = tokens.last().unwrap().clone();
        expect_token!(
            context, [
            Comma, Comma, {};
            CloseParen, CloseParen, break
        ] <= tokens,
             parsed_tokens,
            err_et!(context, t, vec![Comma, CloseParen], t.tt)
        );
    }

    let end = *parsed_tokens.last().unwrap().span.end();

    Good(
        Expression {
            expr: CallExpr(name, args),
            span: start..=end,
        },
        parsed_tokens,
    )
}

pub fn parse_literal_expr(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let t: Token = tokens.last().unwrap().clone();
    let value = expect_token!(
        context,
        [Int(val), Int(val), val] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![Int(0), Float(0.0)], t.tt)
    );
    let start = *parsed_tokens.last().unwrap().span.start();

    let end = *parsed_tokens.last().unwrap().span.end();

    Good(
        Expression {
            expr: LiteralExpr(value),
            span: start..=end,
        },
        parsed_tokens,
    )
}

pub fn parse_parenthesis_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat the opening parenthesis
    let mut parsed_tokens: Vec<Token> = vec![tokens.last().unwrap().clone()];
    let t = tokens.last().unwrap().clone();
    tokens.pop();

    let expr = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);

    expect_token!(
        context,
        [CloseParen, CloseParen, ()] <= tokens,
        parsed_tokens,
        {
            use zom_common::error::{Position, ZomError};
            Bad(ZomError::new(
                Position::try_from_range(
                    context.pos,
                    t.span.clone(),
                    context.source_file.clone(),
                    context.filename.clone(),
                ),
                "unclosed delimiter `)`".to_owned(),
                false,
                None,
                vec![],
            ))
        }
    );
    // idk if the span is correct.
    Good(expr, parsed_tokens)
}

pub fn parse_expr(
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
    // idk if the span is correct.
    Good(expr, parsed_tokens)
}

pub fn parse_binary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
    expr_precedence: i32,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    // start with LHS value
    let mut result = lhs.clone();
    let mut parsed_tokens: Vec<Token> = Vec::new();

    // continue until the current token is not an operator
    // or it is an operator with precedence lesser than expr_precedence
    while let Some(Token {
        tt: Operator(op),
        span: _,
    }) = tokens.last()
    {
        let (operator, precedence) = match settings.operator_precedence.get(op) {
            Some(pr) if *pr >= expr_precedence => (op.clone(), *pr),
            None => {
                return err_et!(
                    context,
                    tokens.last().unwrap(),
                    Vec::<TokenType>::new(),
                    tokens.last().unwrap().tt
                )
            }
            _ => break,
        };
        parsed_tokens.push(tokens.last().unwrap().clone());
        tokens.pop();

        // parse primary RHS expression
        let mut rhs = parse_try!(parse_primary_expr, tokens, settings, context, parsed_tokens);

        // parse all the RHS operators until their precedence is
        // bigger than the current one
        while let Some(Token {
            tt: Operator(ref op),
            span: _,
        }) = tokens.last().cloned()
        {
            let binary_rhs = match settings.operator_precedence.get(op).copied() {
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
                    return err_et!(
                        context,
                        tokens.last().unwrap(),
                        Vec::<TokenType>::new(),
                        tokens.last().unwrap().tt
                    )
                }
                _ => break,
            };

            rhs = binary_rhs;
        }

        // merge LHS and RHS
        result = Expression {
            expr: BinaryExpr {
                op: operator,
                lhs: Box::new(result),
                rhs: Box::new(rhs.clone()),
            },
            span: *lhs.span.start()..=*rhs.span.end(),
        };
    }

    Good(result, parsed_tokens)
}
