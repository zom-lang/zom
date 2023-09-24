//! This module parse function

use std::{ops::Range, str::FromStr};

use zom_common::token::{Str, Token};

use crate::{err_et, expect_token, impl_span, parse_try, parser::types::parse_type, token_parteq};

use super::{
    block::{parse_block, Block},
    types::Type,
    ParserSettings, ParsingContext, PartParsingResult,
};

use self::PartParsingResult::*;

use zom_common::token::{CloseParen, Colon, Comma, Ident, OpenBrace, OpenParen, SemiColon};

use zom_common::error::{Position, ZomError};

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    abi: ABI,
    prototype: Prototype,
    return_ty: Type,
    body: Option<Block>,
    pub span: Range<usize>,
}

impl_span!(Function);

#[derive(PartialEq, Clone, Debug)]
pub enum ABI {
    C,
    CXX,
    Zom,
}

impl ToString for ABI {
    fn to_string(&self) -> String {
        match self {
            ABI::C => "C",
            ABI::CXX => "CXX",
            ABI::Zom => "Zom",
        }
        .to_owned()
    }
}

impl FromStr for ABI {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(ABI::C),
            "CXX" => Ok(ABI::CXX),
            "Zom" => Ok(ABI::Zom),
            abi => Err(format!("Unknown ABI `{}`", abi)),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Arg {
    pub name: String,
    pub type_arg: Type,
    pub span: Range<usize>,
}

impl_span!(Arg);

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<Arg>,
    pub span: Range<usize>,
}

impl_span!(Prototype);

pub fn parse_extern(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Function> {
    // eat Extern keyword
    let mut parsed_tokens = vec![tokens.last().unwrap().clone()];
    tokens.pop();

    let start = parsed_tokens.last().unwrap().span.start;

    let t = tokens.last().unwrap().clone();

    let abi = ABI::from_str(&expect_token!(
        context,
        [Str(abi), Str(abi.clone()), abi] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![Str("C".to_string()), Str("CXX".to_string()), Str("Zom".to_string())], t.tt)
    ));

    if let Err(err) = abi {
        return Bad(ZomError::new(
            Position::try_from_range(
                context.pos,
                parsed_tokens.last().unwrap().span.clone(),
                context.source_file.clone(),
                context.filename.clone().into(),
            ),
            err,
            false,
            None,
            vec![],
        ));
    }

    let abi = abi.unwrap();

    let prototype = parse_try!(parse_prototype, tokens, settings, context, parsed_tokens);

    let return_ty = parse_try!(parse_type, tokens, settings, context, parsed_tokens);

    let t = tokens.last().unwrap().clone();

    let body = if token_parteq!(no_opt t, OpenBrace) {
        Some(parse_try!(
            parse_block,
            tokens,
            settings,
            context,
            parsed_tokens
        ))
    } else {
        expect_token!(
            context,
            [SemiColon, SemiColon, ()] <= tokens,
            parsed_tokens,
            err_et!(context, t, vec![SemiColon], t.tt)
        );
        None
    };

    let end = parsed_tokens.last().unwrap().span.start;
    Good(
        Function {
            abi,
            prototype,
            body,
            return_ty,
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_function(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Function> {
    // eat Func keyword
    let mut parsed_tokens: Vec<Token> = vec![tokens.last().unwrap().clone()];
    tokens.pop();

    let start = parsed_tokens.last().unwrap().span.start;

    let prototype = parse_try!(parse_prototype, tokens, settings, context, parsed_tokens);

    let return_ty = parse_try!(parse_type, tokens, settings, context, parsed_tokens);

    let body = parse_try!(parse_block, tokens, settings, context, parsed_tokens);

    let end = parsed_tokens.last().unwrap().span.end;
    Good(
        Function {
            abi: ABI::Zom,
            prototype,
            body: Some(body),
            return_ty,
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_prototype(
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

    let start = parsed_tokens.last().unwrap().span.start;
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
        let start = parsed_tokens.last().unwrap().span.start;

        let t = tokens.last().unwrap().clone();
        expect_token!(
            context,
            [Colon, Colon, {}] <= tokens,
            parsed_tokens,
            err_et!(context, t, vec![Colon], t.tt)
        );
        let type_arg = parse_try!(parse_type, tokens, settings, context, parsed_tokens);
        let end = parsed_tokens.last().unwrap().span.end;

        args.push(Arg {
            name: name_arg,
            type_arg,
            span: start..end,
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

    let end = parsed_tokens.last().unwrap().span.start;

    Good(
        Prototype {
            name,
            args,
            span: start..end,
        },
        parsed_tokens,
    )
}
