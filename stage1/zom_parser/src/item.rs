//! Module responsible for parsing items, function definition, function declaration or global symbols.

use crate::prelude::*;

use crate::function::Function;
use crate::symbol::Symbol;
use crate::CodeLocation;

use crate::function::{parse_extern, parse_function};
use crate::symbol::parse_global_symbol;

#[derive(PartialEq, Clone, Debug)]
pub enum Item {
    Function(Function),
    GlobalSymbol(Symbol),
}

use Item::*;

impl CodeLocation for Item {
    fn span(&self) -> Range<usize> {
        use Item::*;
        match self {
            Function(f) => f.span.clone(),
            GlobalSymbol(g) => g.span.clone(),
        }
    }
}

pub fn parse_item(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> Option<PartParsingResult<Item>> {
    Some(match tokens.last() {
        Some(Token {
            tt: Fn | Extern, ..
        }) => parse_fn_item(tokens, settings, context),
        Some(Token {
            tt: Var | Const, ..
        }) => parse_global_symbol_item(tokens, settings, context),
        Some(Token { tt: EOF, .. }) => {
            tokens.pop();
            return None;
        }
        None => return None,
        _ => {
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Fn, Extern, Var, Const],
                tokens.last().unwrap().tt
            )
        }
    })
}

/// Private function used by 'parse_item' function, to wrap a function declaration or definition inside an Item
fn parse_fn_item(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Item> {
    let mut parsed_tokens = Vec::new();

    Good(
        Function(match tokens.last().map(|t| t.tt.clone()) {
            Some(Fn) => parse_try!(parse_function, tokens, settings, context, parsed_tokens),
            Some(Extern) => parse_try!(parse_extern, tokens, settings, context, parsed_tokens),
            _ => unreachable!(),
        }),
        parsed_tokens,
    )
}

fn parse_global_symbol_item(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Item> {
    let mut parsed_tokens = Vec::new();

    Good(
        GlobalSymbol(parse_try!(
            parse_global_symbol,
            tokens,
            settings,
            context,
            parsed_tokens
        )),
        parsed_tokens,
    )
}
