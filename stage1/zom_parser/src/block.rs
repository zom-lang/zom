//! Block statement parsing module.

use crate::prelude::*;

use crate::statement::parse_statement;

use super::{
    expr::{Expr, Expression},
    statement::Statement,
};

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub span: Range<usize>,
}

impl_span!(Block);

pub fn parse_block(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Block> {
    let mut parsed_tokens = vec![];
    let t = tokens.last().unwrap().clone();
    expect_token!(
        context,
        [OpenBrace, OpenBrace, ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![OpenBrace],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

    let mut stmts = vec![];

    loop {
        if token_parteq!(tokens.last(), CloseBrace | EOF) {
            break;
        }

        let stmt = parse_try!(parse_statement, tokens, settings, context, parsed_tokens);
        stmts.push(stmt);
    }

    expect_token!(
        context,
        [CloseBrace, CloseBrace, ()] <= tokens,
        parsed_tokens,
        {
            use zom_common::error::{Position, ZomError};
            Bad(ZomError::new(
                Position::try_from_range(
                    context.pos,
                    t.span.clone(),
                    context.source_file.clone(),
                    context.filename.clone().into(),
                ),
                "unclosed delimiter `}`".to_owned(),
                false,
                None,
                vec![],
            ))
        }
    );

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Block {
            stmts,
            span: start..end,
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
