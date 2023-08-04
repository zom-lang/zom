use std::ops::RangeInclusive;

use zom_common::token::{Token, TokenType::*};

use crate::{
    err_et, expect_token, impl_span, parse_try,
    parser::statement::{parse_statement, Stmt},
    token_parteq,
};

use super::{
    expr::{Expr, Expression},
    statement::Statement,
    ParserSettings, ParsingContext, PartParsingResult,
};

use crate::parser::PartParsingResult::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub code: Vec<Statement>,
    pub returned_expr: Option<Box<Expression>>,
    pub span: RangeInclusive<usize>,
}

impl_span!(Block);

pub(super) fn parse_block(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Block> {
    // eat the opening brace
    let mut parsed_tokens = vec![tokens.last().unwrap().clone()];
    let t = tokens.last().unwrap().clone();
    tokens.pop();

    let start = *parsed_tokens.last().unwrap().span.start();

    let mut code = vec![];
    let mut returned_expr: Option<Box<Expression>> = None;

    loop {
        if token_parteq!(tokens.last(), &CloseBrace) {
            break;
        }

        let stmt = parse_try!(parse_statement, tokens, settings, context, parsed_tokens);
        let is_eof = token_parteq!(tokens.last(), &EOF);
        let semi = stmt.is_semi_need() && !is_eof;

        if (!token_parteq!(tokens.last(), &SemiColon))
            && token_parteq!(tokens.last(), &CloseBrace)
            && match stmt.stmt {
                Stmt::Expr(_) => true,
                _ => false,
            }
        {
            if let Stmt::Expr(ref e) = stmt.stmt {
                returned_expr = Some(Box::new(e.clone()))
            }
            break;
        }

        code.push(stmt);

        let t = parsed_tokens.last().unwrap().clone();
        if semi {
            expect_token!(
                context,
                [SemiColon, SemiColon, ()] <= tokens,
                parsed_tokens,
                // error(Box::new(UnexpectedTokenError::from_context(
                //     context,
                //     "Expected ';'".to_owned(),
                //     tokens.last().unwrap().clone()
                // )))
                err_et!(context, t, vec![SemiColon], t.tt)
            );
        }else if is_eof {
            break;
        }
    }

    expect_token!(
        context,
        [CloseBrace, CloseBrace, ()] <= tokens,
        parsed_tokens,
        // error(Box::new(UnexpectedTokenError::from_context(
        //     context,
        //     "Expected '}'".to_owned(),
        //     tokens.last().unwrap().clone()
        // )))
        {
            use zom_common::error::{Position, ZomError};
            Bad(ZomError::new(
                Position::try_from_range(
                    context.pos,
                    t.span.clone(),
                    context.source_file.clone(),
                    context.filename.clone(),
                ),
                "unclosed delimiter `}`".to_owned(),
                false,
                None,
                vec![],
            ))
        }
    );

    let end = *parsed_tokens.last().unwrap().span.end();

    Good(
        Block {
            code,
            returned_expr,
            span: start..=end,
        },
        parsed_tokens,
    )
}

pub fn parse_block_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    match parse_block(tokens, settings, context) {
        Good(block, parsed_tokens) => Good(
            Expression {
                expr: Expr::BlockExpr(block.clone()),
                span: block.span,
            },
            parsed_tokens,
        ),
        NotComplete => NotComplete,
        Bad(err) => Bad(err),
    }
}
