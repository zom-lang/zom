//! This module parse function

use std::ops::RangeInclusive;

use zom_common::token::Token;

use crate::{err_et, expect_token, impl_span, parse_try, parser::types::parse_type};

use super::{
    block::{parse_block, Block},
    types::Type,
    ASTNode, ParserSettings, ParsingContext, PartParsingResult,
};

use self::PartParsingResult::{Bad, Good, NotComplete};

use zom_common::token::{CloseParen, Colon, Comma, Ident, OpenParen, SemiColon};

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    prototype: Prototype,
    body: Option<Block>,
    return_ty: Type,
    span: RangeInclusive<usize>,
}

impl_span!(Function);

#[derive(PartialEq, Clone, Debug)]
pub struct Arg {
    pub name: String,
    pub type_arg: Type,
    pub span: RangeInclusive<usize>,
}

impl_span!(Arg);

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<Arg>,
    pub span: RangeInclusive<usize>,
}

impl_span!(Prototype);

pub(super) fn parse_extern(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<ASTNode> {
    // eat Extern token
    let mut parsed_tokens = vec![tokens.last().unwrap().clone()];
    tokens.pop();

    let start = *parsed_tokens.last().unwrap().span.start();

    let prototype = parse_try!(parse_prototype, tokens, settings, context, parsed_tokens);

    let return_ty = parse_try!(parse_type, tokens, settings, context, parsed_tokens);

    let t = tokens.last().unwrap().clone();

    expect_token!(
        context,
        [SemiColon, SemiColon, ()] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![SemiColon], t.tt)
    );

    let end = *parsed_tokens.last().unwrap().span.start();
    Good(
        ASTNode::FunctionNode(Function {
            prototype,
            body: None,
            return_ty,
            span: start..=end,
        }),
        parsed_tokens,
    )
}

pub(super) fn parse_function(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<ASTNode> {
    // eat Func token
    let mut parsed_tokens: Vec<Token> = vec![tokens.last().unwrap().clone()];
    tokens.pop();

    let start = *parsed_tokens.last().unwrap().span.start();

    let prototype = parse_try!(parse_prototype, tokens, settings, context, parsed_tokens);

    let return_ty = parse_try!(parse_type, tokens, settings, context, parsed_tokens);

    let body = parse_try!(parse_block, tokens, settings, context, parsed_tokens);

    let end = *parsed_tokens.last().unwrap().span.end();
    Good(
        ASTNode::FunctionNode(Function {
            prototype,
            body: Some(body),
            return_ty,
            span: start..=end,
        }),
        parsed_tokens,
    )
}

pub(super) fn parse_prototype(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Prototype> {
    let mut parsed_tokens = Vec::new();
    let t = tokens.last().unwrap().clone();

    let name = expect_token!(
        context,
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![Ident(String::new())], t.tt)
    );

    let start = *parsed_tokens.last().unwrap().span.start();
    let t = tokens.last().unwrap().clone();

    expect_token!(
        context,
        [OpenParen, OpenParen, ()] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![OpenParen], t.tt)
    );

    let mut args = Vec::new();
    loop {
        let name_arg;
        let t = tokens.last().unwrap().clone();
        expect_token!(
            context, [
            Ident(arg), Ident(arg.clone()), name_arg = arg;
            CloseParen, CloseParen, break
        ] <= tokens,
             parsed_tokens,
            err_et!(context, t, vec![Ident(String::new())], t.tt)
        );
        let start = *parsed_tokens.last().unwrap().span.start();

        let t = tokens.last().unwrap().clone();
        expect_token!(
            context,
            [Colon, Colon, {}] <= tokens,
            parsed_tokens,
            err_et!(context, t, vec![Colon], t.tt)
        );
        let type_arg = parse_try!(parse_type, tokens, settings, context, parsed_tokens);
        let end = *parsed_tokens.last().unwrap().span.end();

        args.push(Arg {
            name: name_arg,
            type_arg,
            span: start..=end,
        });
        let t = tokens.last().unwrap().clone();

        expect_token!(
            context, [
            Comma, Comma, {};
            CloseParen, CloseParen, break
        ] <= tokens,
             parsed_tokens,
            err_et!(context, t, vec![Comma], t.tt)
        );
    }

    let end = *parsed_tokens.last().unwrap().span.start();

    Good(
        Prototype {
            name,
            args,
            span: start..=end,
        },
        parsed_tokens,
    )
}
