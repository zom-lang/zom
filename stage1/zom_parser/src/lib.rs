use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;

use crate::prelude::*;
use crate::source_file::SourceFile;

pub mod block;
pub mod declaration;
pub(crate) mod err;
pub mod expr;
pub(crate) mod prelude;
pub mod source_file;
pub mod stmt;
pub mod types;

lazy_static! {
    static ref PR_TABLE: HashMap<BinOperation, (Associativity, u16)> = {
        use zom_common::token::{
            PR_ADD_SUB, PR_AND, PR_COMP, PR_COMP_EQ_NE, PR_MUL_DIV_REM, PR_OR, PR_SHIFT, PR_XOR,
        };
        use Associativity::*;
        use BinOperation::*;
        HashMap::from([
            (Mul, (L2R, PR_MUL_DIV_REM)),
            (Div, (L2R, PR_MUL_DIV_REM)),
            (Rem, (L2R, PR_MUL_DIV_REM)),
            // ..
            (Add, (L2R, PR_ADD_SUB)),
            (Sub, (L2R, PR_ADD_SUB)),
            // ..
            (RShift, (L2R, PR_SHIFT)),
            (LShift, (L2R, PR_SHIFT)),
            // ..
            (CompLT, (L2R, PR_COMP)),
            (CompGT, (L2R, PR_COMP)),
            (CompLTE, (L2R, PR_COMP)),
            (CompGTE, (L2R, PR_COMP)),
            // ..
            (CompEq, (L2R, PR_COMP_EQ_NE)),
            (CompNe, (L2R, PR_COMP_EQ_NE)),
            // ..
            (And, (L2R, PR_AND)),
            (Xor, (L2R, PR_XOR)),
            (Or, (L2R, PR_OR)),
        ])
    };
}

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
    fn last(&self) -> &Token {
        self.tokens.last().unwrap()
    }

    /// Did the EOF token has been poped?
    pub fn reached_eof(&self) -> bool {
        // if the vector of token is empty, without the EOF token,
        // that means we reached EOF
        self.tokens.is_empty() || token_parteq!(self.last(), T::EOF)
    }

    /// Did the next token mark the end of the expression?
    pub fn expr_end(&self) -> bool {
        matches!(
            self.last().tt,
            T::SemiColon | T::Comma | T::CloseParen | T::CloseBracket | T::CloseBrace | T::Else
        )
    }

    pub fn pr_get(&self, op: BinOperation) -> (Associativity, u16) {
        PR_TABLE
            .get(&op)
            .cloned()
            .expect("Binary operator not in binary table of precedence, impossible in theory")
    }
}

#[macro_export]
macro_rules! expect_token {
    ($parser:expr => [ $($token:pat, $result:expr);+ ], $expected:expr, $parsed_tokens:expr ) => {
        expect_token!($parser => [ $( $token, $result );+ ] else {
            let found = $parser.pop();
            return Error(Box::new(ExpectedToken::from(&found, $expected)))
        }, $parsed_tokens)
    };
    ($parser:expr => [ $($token:pat, $result:expr);+ ] else $unmatched:block, $parsed_tokens:expr ) => {
        match &$parser.last().tt {
            $(
                // used, because if $result is a no return expression, it will throw those 2 warnings
                #[allow(unreachable_code, unused_variables)]
                $token => {
                    let res = $result;
                    $parsed_tokens.push($parser.pop());
                    res
                },
            )+
            _ => $unmatched
        }
    };
}

#[macro_export]
macro_rules! parse_try {
    ($parser:expr => $ast_type:ty, $parsed_tokens:expr) => {
        parse_try!(fn; $parser => <$ast_type as Parse>::parse, $parsed_tokens)
    };
    (fn; $parser:expr => $parsing_func:expr, $parsed_tokens:expr $(, $arg:expr)* ) => {
        match $parsing_func($parser, $($arg),*) {
            Good(ast, tokens) => {
                $parsed_tokens.extend(tokens);
                ast
            }
            Error(err) => return Error(err),
        }
    };
    (continue; $parser:expr => $ast_type:ty, $parsed_tokens:expr) => {
        match <$ast_type as Parse>::parse($parser) {
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

impl<T: fmt::Debug> fmt::Debug for ParsingResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Good(v, toks) => f.debug_tuple("Good").field(v).field(toks).finish(),
            Error(_) => f.debug_tuple("Error").field(&"...").finish(),
        }
    }
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
        match $left.tt {
            $right => true,
            _ => false
        }
    );
);
