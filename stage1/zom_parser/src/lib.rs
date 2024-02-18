use zom_common::token::Token;
use zom_common::token::TokenType as T;
use zom_errors::prelude::*;
use zom_errors::FmtToken::*;

pub(crate) mod err;

pub struct Parser<'a> {
    /// Reversed list of token
    tokens: Vec<Token>,
    lctx: LogContext<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], lctx: LogContext<'a>) -> Parser<'a> {
        let mut rest = tokens.to_vec();
        rest.reverse();
        Parser { tokens: rest, lctx }
    }

    pub fn parse(&mut self) -> FinalRes<SourceFile> {
        expect_token!(self => [T::Ident(_), ()], [Ident, Ident]);

        if self.lctx.failed() {
            FinalRes::Err(self.lctx.stream())
        } else {
            FinalRes::Ok((), self.lctx.clone())
        }
    }

    /// Returns and removes the last token of the reverse token stream
    fn pop(&mut self) -> Token {
        // here we unwrap because when the EOF token comes, we stop pop token
        self.tokens
            .pop()
            .expect("another token has been poped after the EOF token")
    }

    /// Returns the last tokens without removing it from the reversed token stream
    fn last(&mut self) -> &Token {
        self.tokens.last().unwrap()
    }
}

type SourceFile = ();

pub type ParserRes<T> = Option<T>;

#[macro_export]
macro_rules! expect_token {
    ($parser:expr => [ $($token:pat, $result:stmt);+ ], $expected:expr ) => {
        match $parser.last() {
            $(
                Token { tt: $token, .. } => {
                    _ = $parser.pop();
                    $result
                },
            )+
            found => {
                let found = found.clone();
                $parser.lctx.push($crate::err::ExpectedToken::from(&found, $expected))
            }
        }
    };
}
