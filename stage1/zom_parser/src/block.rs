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

        // TODO(Larsouille25): Statments parsing here

        expect_token!(parser => [T::CloseBrace, ()], CloseBrace, parsed_tokens);
        let end = span_toks!(end parsed_tokens);

        Good(
            Block {
                stmts: Vec::new(),
                span: start..end,
            },
            parsed_tokens,
        )
    }
}
