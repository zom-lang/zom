//! Module responsible for parsing top level declarations.
use crate::{block::Block, prelude::*, types::Type, var_decl::VarDecl};

#[derive(Debug)]
pub struct TopLevelDeclaration {
    pub public: bool,
    pub decl: TopLvlDecl,
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
        let decl = parse_try!(parser => TopLvlDecl, parsed_tokens);

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
pub enum TopLvlDecl {
    Function {
        lib: Option<String>,
        proto: Prototype,
        body: Option<Block>,
    },
    GlobalVarDecl(VarDecl),
}

impl Parse for TopLvlDecl {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        match &parser.last().tt {
            T::Fn => parse_fn_decl(parser),
            T::Extern => parse_extern_decl(parser),
            T::Const | T::Var => parse_global_var_decl(parser),
            _ => Error(Box::new(ExpectedToken::from(
                parser.last(),
                PartAST::Declaration,
            ))),
        }
    }
}

#[derive(Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<Arg>,
    pub ret_ty: Type,
}

impl Parse for Prototype {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();

        let name = expect_token!(parser => [T::Ident(name), name.clone()], Ident, parsed_tokens);

        expect_token!(parser => [T::OpenParen, ()], OpenParen, parsed_tokens);

        let mut args = Vec::new();
        loop {
            args.push(parse_try!(parser => Arg, parsed_tokens));
            expect_token!(parser => [T::Comma, (); T::CloseParen, break], Comma, parsed_tokens);
        }

        expect_token!(parser => [T::CloseParen, ()], CloseParen, parsed_tokens);

        let ret_ty = parse_try!(parser => Type, parsed_tokens);

        Good(Prototype { name, args, ret_ty }, parsed_tokens)
    }
}

pub fn parse_fn_decl(parser: &mut Parser) -> ParsingResult<TopLvlDecl> {
    let mut parsed_tokens = Vec::new();
    expect_token!(parser => [T::Fn, ()], Fn, parsed_tokens);

    let proto = parse_try!(parser => Prototype, parsed_tokens);

    let body = Some(parse_try!(parser => Block, parsed_tokens));

    Good(
        TopLvlDecl::Function {
            lib: None,
            proto,
            body,
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

pub fn parse_global_var_decl(parser: &mut Parser) -> ParsingResult<TopLvlDecl> {
    let mut parsed_tokens = Vec::new();

    let var_decl = parse_try!(parser => VarDecl, parsed_tokens);

    Good(TopLvlDecl::GlobalVarDecl(var_decl), parsed_tokens)
}

pub fn parse_extern_decl(parser: &mut Parser) -> ParsingResult<TopLvlDecl> {
    let mut parsed_tokens = Vec::new();

    expect_token!(parser => [T::Extern, ()], Extern, parsed_tokens);
    let lib = Some(expect_token!(parser => [T::Str(l), l.clone()], StrLit, parsed_tokens));

    expect_token!(parser => [T::Fn, ()], Fn, parsed_tokens);
    let proto = parse_try!(parser => Prototype, parsed_tokens);

    let body = if token_parteq!(parser.last(), T::OpenBrace) {
        Some(parse_try!(parser => Block, parsed_tokens))
    } else {
        None
    };

    Good(TopLvlDecl::Function { lib, proto, body }, parsed_tokens)
}
