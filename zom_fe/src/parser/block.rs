use std::ops::RangeInclusive;

use zom_common::{
    error::parser::UnexpectedTokenError,
    token::{Token, TokenType::*},
};

use crate::{
    expect_token, parse_try,
    parser::{error, statement::parse_statement},
    FromContext, token_parteq, impl_span,
};

use super::{
    expr::Expression, statement::Statement, ParserSettings, ParsingContext, PartParsingResult,
};

use crate::parser::PartParsingResult::*;

#[derive(PartialEq, Clone, Debug)]
pub struct BlockCodeExpr {
    pub code: Vec<Statement>,
    pub returned_expr: Option<Box<Expression>>,
    pub span: RangeInclusive<usize>,
}

impl_span!(BlockCodeExpr);

pub(super) fn parse_block_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<BlockCodeExpr> {
    // eat the opening brace
    let mut parsed_tokens = vec![tokens.last().unwrap().clone()];
    tokens.pop();

    let mut code = vec![];
    let mut returned_expr: Option<Box<Expression>> = None;

    loop {
        if token_parteq!(tokens.last(), &CloseBrace) {
            break;
        }

        let stmt = parse_try!(parse_statement, tokens, settings, context, parsed_tokens);
        let semi = stmt.is_semi_need();

        // FIXME: Allow Binary operation in expression, in statements to allow `a = <expr>`..

        if !token_parteq!(tokens.last(), &SemiColon) &&
            token_parteq!(tokens.last(), &CloseBrace) &&
            match stmt {
            Statement::Expr(_) => true,
            _ => false
        } {
            match stmt {
                Statement::Expr(ref e) => returned_expr = Some(Box::new(e.clone())),
                _ => {}
            }
            break;
        }

        code.push(stmt);

        if semi {
            expect_token!(
                context,
                [SemiColon, SemiColon, ()] <= tokens,
                parsed_tokens,
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

    Good(BlockCodeExpr {
        code,
        returned_expr,
        span: *parsed_tokens[0].span.start()..=*parsed_tokens.last().unwrap().span.end()
    }, parsed_tokens)
}
