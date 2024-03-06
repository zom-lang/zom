//! Module responsible for parsing blocks.
use crate::prelude::*;
use crate::stmt::Statement;

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub span: Range<usize>,
}

impl Parse for Block {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();

        expect_token!(parser => [T::OpenBrace, ()], OpenBrace, parsed_tokens);
        let start = span_toks!(start parsed_tokens);

        let mut stmts = Vec::new();
        while !token_parteq!(parser.last(), T::CloseBrace) {
            // parse and push the stmt
            stmts.push(parse_try!(parser => Statement, parsed_tokens));

            // maybe expect a semicolon
            expect_token!(parser => [T::SemiColon, ()] else {}, parsed_tokens);
        }

        expect_token!(parser => [T::CloseBrace, ()], CloseBrace, parsed_tokens);
        let end = span_toks!(end parsed_tokens);

        Good(
            Block {
                stmts,
                span: start..end,
            },
            parsed_tokens,
        )
    }
}
