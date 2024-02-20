use crate::prelude::*;
use crate::source_file::SourceFile;

pub(crate) mod err;
pub(crate) mod prelude;
pub mod source_file;

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

    pub fn parse(mut self) -> FinalRes<'a, SourceFile> {
        match SourceFile::parse(&mut self) {
            Good(ast, ..) => FinalRes::Ok(ast, self.lctx.clone()),
            Error(err) => {
                self.lctx.push_boxed(err);
                FinalRes::Err(self.lctx.stream())
            }
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

    /// Did the EOF token has been poped?
    pub fn reached_eof(&self) -> bool {
        // if the vector of token is empty, without the EOF token,
        // that means we reached EOF
        self.tokens.is_empty()
    }
}

#[macro_export]
macro_rules! expect_token {
    ($parser:expr => [ $($token:pat, $result:expr);+ ], $expected:expr, $parsed_tokens:expr ) => {
        match $parser.last() {
            $(
                Token { tt: $token, .. } => {
                    let res = $result;
                    $parsed_tokens.push($parser.pop());
                    res
                },
            )+
            _ => {
                let found = $parser.pop();
                return Error(Box::new(ExpectedToken::from(&found, $expected)))
            }
        }
    };
}

#[macro_export]
macro_rules! parse_try {
    ($parser:expr => $ast_type:ty, $parsed_tokens:expr) => {
        match <$ast_type as Parse>::parse($parser) {
            Good(ast, tokens) => {
                $parsed_tokens.extend(tokens);
                ast
            }
            Error(err) => return Error(err),
        }
    };
    (continue; $parser:expr => $ast_type:ty, $parsed_tokens:expr) => {
        match <$ast_type>::parse($parser) {
            Good(ast, tokens) => {
                $parsed_tokens.extend(tokens);
                ast
            }
            Error(err) => {
                if $parser.reached_eof() {
                    return Error(err);
                }
                $parser.lctx.push_boxed(err);
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! span_toks {
    ($toks:expr) => {
        $toks.last().unwrap().span.clone()
    };
    ($id:ident $toks:expr) => {
        $toks.last().unwrap().span.$id
    };
}

pub enum ParsingResult<T> {
    Good(T, Vec<Token>),
    Error(Box<dyn Log>),
}

pub trait Parse {
    type Output;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output>;
}

/// This macro is to test the equality of a token but without checking the span.
/// return true if it's equal or false if it's not.
#[macro_export]
macro_rules! token_parteq(
    ($left:expr, $right:pat) => (
        match $left {
            Token { tt: $right, ..} => true,
            _ => false
        }
    );
);
