//! Module responsible for parsing items, function definition, function declaration or global symbols.

use crate::prelude::*;

use crate::block::{parse_block, Block};

use crate::symbol::{parse_global_symbol, Symbol};
use crate::types::{parse_type, Type};

#[derive(PartialEq, Clone, Debug)]
pub struct Item {
    public: bool,
    item: PrimaryItem,
    span: Range<usize>,
}

impl_span!(Item);

#[derive(PartialEq, Clone, Debug)]
pub enum PrimaryItem {
    Function {
        abi: Option<String>,
        prototype: Prototype,
        body: Option<Block>,
    },
    GlobalSymbol {
        abi: Option<String>,
        s: Symbol,
    },
}

use PrimaryItem::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Arg {
    name: String,
    type_arg: Type,
    span: Range<usize>,
}

impl_span!(Arg);

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    name: String,
    args: Vec<Arg>,
    return_type: Type,
    span: Range<usize>,
}

impl_span!(Prototype);

pub fn parse_item(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> Option<PartParsingResult<Item>> {
    let mut parsed_tokens = Vec::new();
    let mut public = false;
    match tokens.last() {
        Some(Token { tt: Pub, .. }) => {
            parsed_tokens.push(tokens.pop().unwrap());
            public = true;
        }
        Some(Token { tt: EOF, .. }) => {
            tokens.pop();
            return None;
        }
        Some(_) => {}
        None => return None,
    }
    // we cannot use the parse_try! macro because this function was a special return type,
    // so we expand it directly.
    let item = match parse_primary_item(tokens, settings, context) {
        Good(ast, toks) => {
            parsed_tokens.extend(toks);
            ast
        }
        NotComplete => {
            parsed_tokens.reverse();
            tokens.extend(parsed_tokens);
            return Some(NotComplete);
        }
        Bad(error) => return Some(Bad(error)),
    };

    let span = parsed_tokens[0].span.start..parsed_tokens.last().unwrap().span.end;
    Some(Good(Item { public, item, span }, parsed_tokens))
}

pub fn parse_primary_item(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<PrimaryItem> {
    match tokens.last() {
        Some(Token { tt: Fn, .. }) => parse_fn_pitem(tokens, settings, context),
        Some(Token {
            tt: Var | Const, ..
        }) => parse_global_symbol_pitem(tokens, settings, context),
        Some(Token { tt: Extern, .. }) => parse_extern_pitem(tokens, settings, context),
        None => unreachable!(),
        _ => {
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Fn, Extern, Var, Const],
                tokens.last().unwrap().tt
            )
        }
    }
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

    let return_type = parse_try!(parse_type, tokens, settings, context, parsed_tokens);

    let end = parsed_tokens.last().unwrap().span.start;

    Good(
        Prototype {
            name,
            args,
            return_type,
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_extern_pitem(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<PrimaryItem> {
    // we are looking two tokens ahead to know which parsing function we need to call
    match tokens.get(tokens.len() - 3) {
        Some(Token { tt: Fn, .. }) => parse_fn_pitem(tokens, settings, context),
        Some(Token {
            tt: Var | Const, ..
        }) => parse_global_symbol_pitem(tokens, settings, context),
        Some(t @ Token { tt, .. }) => return err_et!(context, t, vec![Fn, Var, Const], tt),
        None => NotComplete,
    }
}

pub fn parse_fn_pitem(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<PrimaryItem> {
    let mut parsed_tokens = Vec::new();
    let mut abi = None;
    if let Some(Token { tt: Extern, .. }) = tokens.last() {
        expect_token!(
            context,
            [Extern, Extern, ()] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Extern],
                tokens.last().unwrap().tt
            )
        );
        abi = expect_token!(
            context,
            [Str(a), Str(a.clone()), Some(a)] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Extern],
                tokens.last().unwrap().tt
            )
        );
    }
    expect_token!(
        context,
        [Fn, Fn, ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Extern],
            tokens.last().unwrap().tt
        )
    );
    let prototype = parse_try!(parse_prototype, tokens, settings, context, parsed_tokens);
    let body = match tokens.last() {
        Some(Token { tt: OpenBrace, .. }) => Some(parse_try!(
            parse_block,
            tokens,
            settings,
            context,
            parsed_tokens
        )),
        Some(Token { tt: SemiColon, .. }) => {
            expect_token!(
                context,
                [SemiColon, SemiColon, None] <= tokens,
                parsed_tokens,
                err_et!(
                    context,
                    tokens.last().unwrap(),
                    vec![SemiColon],
                    tokens.last().unwrap().tt
                )
            )
        }
        Some(t @ Token { tt, .. }) => return err_et!(context, t, vec![OpenBrace, SemiColon], tt),
        None => return NotComplete,
    };

    Good(
        Function {
            abi,
            prototype,
            body,
        },
        parsed_tokens,
    )
}

pub fn parse_global_symbol_pitem(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<PrimaryItem> {
    let mut parsed_tokens = Vec::new();
    let mut abi = None;
    if let Some(Token { tt: Extern, .. }) = tokens.last() {
        expect_token!(
            context,
            [Extern, Extern, ()] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Extern],
                tokens.last().unwrap().tt
            )
        );
        abi = expect_token!(
            context,
            [Str(a), Str(a.clone()), Some(a)] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Extern],
                tokens.last().unwrap().tt
            )
        );
    }
    let s = parse_try!(
        parse_global_symbol,
        tokens,
        settings,
        context,
        parsed_tokens
    );
    Good(GlobalSymbol { abi, s }, parsed_tokens)
}
