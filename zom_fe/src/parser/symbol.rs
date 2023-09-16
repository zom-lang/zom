//! Module responsible for parsing symbol, `const` and `var`.

use core::ops::Range;

use crate::parser::expr::parse_expr;
use crate::parser::types::{parse_type, Type};
use crate::{err_et, expect_token, parse_try};
use crate::{impl_span, parser::expr::Expression};

use super::{ParserSettings, ParsingContext, PartParsingResult, PartParsingResult::*};

use zom_common::token::Operator;
use zom_common::token::{Token, TokenType::*};

#[derive(PartialEq, Clone, Debug)]
pub struct Symbol {
    /// true if it's `var ...` and false if not
    is_var: bool,
    name: String,
    ty: Option<Type>,
    value: Expression,
    pub span: Range<usize>,
}

impl_span!(Symbol);

pub fn parse_symbol(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Symbol> {
    // eat either Var or Const keyword
    let mut parsed_tokens = vec![tokens.last().unwrap().clone()];
    let tok = tokens.pop();

    let start = parsed_tokens.last().unwrap().span.start;

    let is_var = match tok.map(|t| t.tt) {
        Some(Var) => true,
        Some(Const) => false,
        None => return NotComplete,
        _ => panic!("WTF a symbol doesn't starting with either 'var' or 'const'"),
    };

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

    dbg!(tokens.last());

    let ty = if let Some(Token { tt: Colon, .. }) = tokens.last() {
        expect_token!(
            context,
            [Colon, Colon, ()] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Colon],
                tokens.last().unwrap().tt
            )
        );

        Some(parse_try!(
            parse_type,
            tokens,
            settings,
            context,
            parsed_tokens
        ))
    } else {
        None
    };

    expect_token!(
        context,
        [Operator(Operator::Equal), Operator(Operator::Equal), ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Operator(Operator::Equal)],
            tokens.last().unwrap().tt
        )
    );

    let value = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);
    dbg!(&value);

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Symbol {
            is_var,
            name,
            ty,
            value,
            span: start..end,
        },
        parsed_tokens,
    )
}
