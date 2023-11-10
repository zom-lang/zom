//! Module responsible for parsing symbol, `const` and `var`.

use crate::prelude::*;

use crate::expr::{parse_expr, Expr, Expression};
use crate::types::{parse_type, Type};

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
    let mut parsed_tokens = vec![];

    let is_var = expect_token!(
        context,
        [Var, Var, true;
         Const, Const, false] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Var, Const],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

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
        [Oper(Operator::Equal), Oper(Operator::Equal), ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Oper(Operator::Equal)],
            tokens.last().unwrap().tt
        )
    );

    let value = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);

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

pub fn parse_global_symbol(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Symbol> {
    let mut parsed_tokens = Vec::new();
    let symbol = parse_try!(parse_symbol, tokens, settings, context, parsed_tokens);

    if let Expression {
        expr: Expr::UndefinedExpr,
        ref span,
    } = symbol.value
    {
        context.push_err(ZomError::new(
            Position::try_from_range(
                context.pos,
                span.clone(),
                context.source_file.clone(),
                context.filename.clone().into(),
            ),
            "a global cannot have an undefined value".to_string(),
            false,
            None,
            vec![],
        ))
    }

    let t = parsed_tokens.last().unwrap();

    expect_token!(
        context,
        [SemiColon, SemiColon, ()] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![SemiColon], t.tt)
    );

    Good(symbol, parsed_tokens)
}
