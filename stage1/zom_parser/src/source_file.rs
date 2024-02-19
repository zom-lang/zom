//! Module responsible for parsing the top level source file.
use crate::prelude::*;

#[derive(Debug)]
pub struct SourceFile {
    pub pkg_path: QualifiedIdentifier,
    pub import_decls: Vec<ImportDecl>,
    pub span: Range<usize>,
}

impl Parse for SourceFile {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();

        expect_token!(parser => [T::Package, ()], Package, parsed_tokens);

        let start = span_toks!(start parsed_tokens);

        let pkg_path = parse_try!(parser, QualifiedIdentifier, parsed_tokens);

        let mut import_decls = Vec::new();
        while token_parteq!(parser.last(), T::Import) {
            import_decls.push(parse_try!(continue; parser, ImportDecl, parsed_tokens));
        }

        // TODO(Larsouille25): add top level declaration parsing

        let end = span_toks!(end parsed_tokens);

        Good(
            SourceFile {
                pkg_path,
                import_decls,
                span: start..end,
            },
            parsed_tokens,
        )
    }
}

#[derive(Debug)]
pub struct QualifiedIdentifier {
    pub path: Vec<String>,
    pub span: Range<usize>,
}

impl Parse for QualifiedIdentifier {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();
        let mut path = Vec::new();

        path.push(expect_token!(parser => [T::Ident(name), name.clone()], Ident, parsed_tokens));
        let start = span_toks!(start parsed_tokens);

        while token_parteq!(parser.last(), T::Oper(Operator::Dot)) {
            expect_token!(parser => [T::Oper(Operator::Dot), ()], Dot, parsed_tokens);
            path.push(
                expect_token!(parser => [T::Ident(name), name.clone()], Ident, parsed_tokens),
            );
        }

        let end = span_toks!(end parsed_tokens);

        Good(
            QualifiedIdentifier {
                path,
                span: start..end,
            },
            parsed_tokens,
        )
    }
}

#[derive(Debug)]
pub struct ImportDecl {
    pub path: QualifiedIdentifier,
    pub alias: Option<String>,
    pub span: Range<usize>,
}

impl Parse for ImportDecl {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();

        expect_token!(parser => [T::Import, ()], Import, parsed_tokens);

        let start = span_toks!(start parsed_tokens);

        let path = parse_try!(parser, QualifiedIdentifier, parsed_tokens);

        let alias = if token_parteq!(parser.last(), T::As) {
            expect_token!(parser => [T::As, ()], As, parsed_tokens);
            expect_token!(parser => [T::Ident(alias), Some(alias.clone())], Ident, parsed_tokens)
        } else {
            None
        };

        let end = span_toks!(end parsed_tokens);

        Good(
            ImportDecl {
                path,
                alias,
                span: start..end,
            },
            parsed_tokens,
        )
    }
}
