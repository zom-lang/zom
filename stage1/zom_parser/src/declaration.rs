//! Module responsible for parsing top level declarations.
use crate::{block::Block, prelude::*, types::Type};

#[derive(Debug)]
pub struct TopLevelDeclaration {
    pub public: bool,
    pub decl: Declaration,
    pub span: Range<usize>,
}

impl Parse for TopLevelDeclaration {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();

        let (public, start) = if token_parteq!(parser.last(), T::Pub) {
            expect_token!(parser => [T::Pub, ()], Pub, parsed_tokens);
            (true, span_toks!(start parsed_tokens))
        } else {
            (false, span_toks!(start parser.tokens))
        };
        let decl = parse_try!(parser => Declaration, parsed_tokens);

        let end = span_toks!(end parsed_tokens);

        Good(
            TopLevelDeclaration {
                public,
                decl,
                span: start..end,
            },
            parsed_tokens,
        )
    }
}

#[derive(Debug)]
pub enum Declaration {
    Function {
        name: String,
        args: Vec<Arg>,
        ret_ty: Type,
        block: Block,
    },
}

impl Parse for Declaration {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        match parser.last() {
            Token { tt: T::Fn, .. } => parse_fn_decl(parser),
            found => Error(Box::new(ExpectedToken::from(found, PartAST::Declaration))),
        }
    }
}

pub fn parse_fn_decl(parser: &mut Parser) -> ParsingResult<Declaration> {
    let mut parsed_tokens = Vec::new();
    expect_token!(parser => [T::Fn, ()], Fn, parsed_tokens);

    let name = expect_token!(parser => [T::Ident(name), name.clone()], Ident, parsed_tokens);

    expect_token!(parser => [T::OpenParen, ()], OpenParen, parsed_tokens);

    let mut args = Vec::new();
    loop {
        args.push(parse_try!(parser => Arg, parsed_tokens));
        expect_token!(parser => [T::Comma, (); T::CloseParen, break], Comma, parsed_tokens);
    }

    expect_token!(parser => [T::CloseParen, ()], CloseParen, parsed_tokens);

    let ret_ty = parse_try!(parser => Type, parsed_tokens);

    let block = parse_try!(parser => Block, parsed_tokens);

    Good(
        Declaration::Function {
            name,
            args,
            ret_ty,
            block,
        },
        parsed_tokens,
    )
}

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub ty: Type,
    pub span: Range<usize>,
}

impl Parse for Arg {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();

        let name = expect_token!(parser => [T::Ident(name), name.clone()], Ident, parsed_tokens);
        let start = span_toks!(start parsed_tokens);

        expect_token!(parser => [T::Colon, ()], Colon, parsed_tokens);
        let ty = parse_try!(parser => Type, parsed_tokens);

        let end = span_toks!(end parsed_tokens);

        Good(
            Arg {
                name,
                ty,
                span: start..end,
            },
            parsed_tokens,
        )
    }
}
