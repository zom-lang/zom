use zom_common::{token::Token::{self, *}, error::parser::UnexpectedTokenError};

use crate::{expect_token, parse_try, parser::{error, statement::parse_statement}, FromContext};

use super::{expr::Expression, ParserSettings, ParsingContext, PartParsingResult, statement::Statement};

use crate::parser::PartParsingResult::*;

#[derive(PartialEq, Clone, Debug)]
pub struct BlockCodeExpr {
    code: Vec<Statement>,
    returned_expr: Box<Expression>
}

pub(super) fn parse_block_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat the opening brace
    tokens.pop();
    let mut parsed_tokens = vec![OpenBrace];

    let mut stmts = vec![];

    while Some(&CloseBrace) != tokens.last() {
        let stmt = parse_try!(parse_statement, tokens, settings, context, parsed_tokens);
        let semi = stmt.is_semi_need();

        // FIXME: Allow Binary operation in expression, in statements to allow `a = <expr>`..

        stmts.push(stmt);

        if semi {
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

    println!("res = {stmts:?}");

    todo!()
    // Good(Expression::BlockExpr { exprs }, parsed_tokens)
}