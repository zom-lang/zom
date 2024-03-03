//! This crate is responsible for the parsing of the Token Stream into
//! an Abstract Syntax Tree.
//!
//! The parser is implemented with a [Recursive descent parser][1] and
//! an [Operator-precedence parser][2] for parsing of arithmetic
//! expressions for ally the convenience of the recursive parser with
//! the speed of the operator-precedence for parsing arithmetic
//! expressions.
//!
//! [1]: https://en.wikipedia.org/wiki/Recursive_descent_parser
//! [2]: https://en.wikipedia.org/wiki/Operator-precedence_parser

use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;

use crate::expr::Operation;
use crate::prelude::*;
use crate::source_file::SourceFile;

pub mod block;
pub(crate) mod err;
pub mod expr;
pub(crate) mod prelude;
pub mod source_file;
pub mod stmt;
pub mod toplvldecl;
pub mod types;
pub mod var_decl;

lazy_static! {
    static ref PR_TABLE: HashMap<Operation, (Associativity, u16)> = {
        use zom_common::operator::{
            PR_DEREFERENCE, PR_UNARY, PR_ADD_SUB, PR_AND, PR_COMP, PR_COMP_EQ_NE, PR_MUL_DIV_REM, PR_OR, PR_SHIFT, PR_XOR,
        };
        use Associativity::*;
        use BinOperation::*;
        use UnaryOperation::*;
        use crate::expr::Operation::*;
        HashMap::from([
            (Unary(Dereference), (L2R, PR_DEREFERENCE)),
            // ..
            (Unary(AddressOf), (R2L, PR_UNARY)),
            (Unary(Negation), (R2L, PR_UNARY)),
            (Unary(Not), (R2L, PR_UNARY)),
            // ..
            (Binary(Mul), (L2R, PR_MUL_DIV_REM)),
            (Binary(Div), (L2R, PR_MUL_DIV_REM)),
            (Binary(Rem), (L2R, PR_MUL_DIV_REM)),
            // ..
            (Binary(Add), (L2R, PR_ADD_SUB)),
            (Binary(Sub), (L2R, PR_ADD_SUB)),
            // ..
            (Binary(RShift), (L2R, PR_SHIFT)),
            (Binary(LShift), (L2R, PR_SHIFT)),
            // ..
            (Binary(CompLT), (L2R, PR_COMP)),
            (Binary(CompGT), (L2R, PR_COMP)),
            (Binary(CompLTE), (L2R, PR_COMP)),
            (Binary(CompGTE), (L2R, PR_COMP)),
            // ..
            (Binary(CompEq), (L2R, PR_COMP_EQ_NE)),
            (Binary(CompNe), (L2R, PR_COMP_EQ_NE)),
            // ..
            (Binary(And), (L2R, PR_AND)),
            (Binary(Xor), (L2R, PR_XOR)),
            (Binary(Or), (L2R, PR_OR)),
        ])
    };
}

pub struct Parser<'a> {
    /// Reversed list of token
    tokens: Vec<Token>,
    lctx: LogContext<'a>,
    pub default_precedence: u16,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], lctx: LogContext<'a>) -> Parser<'a> {
        let mut rest = tokens.to_vec();
        rest.reverse();
        Parser {
            tokens: rest,
            lctx,
            default_precedence: 0,
        }
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

    /// Did the EOF token has been reached?
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

    pub fn pr_get<O: Into<Operation>>(&self, op: O) -> (Associativity, u16) {
        PR_TABLE
            .get(&op.into())
            .cloned()
            .expect("Binary operator not in binary table of precedence, impossible in theory")
    }

    /// Get the nth starting at the end.
    ///
    /// It may panic, if:
    /// - the offset is greater than the amount of tokens
    /// - the index is not in range of the tokens list
    ///
    /// **e.g:**
    /// ```text
    ///    fn test
    ///    ^1 ^2 offset of end_nth
    /// ```
    pub fn end_nth(&self, offset: usize) -> &Token {
        self.tokens.get(self.tokens.len() - offset).unwrap()
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
    ($parser:expr => $ast_type:ty, $parsed_tokens:expr $(,in $in_between:expr)?) => {
        parse_try!(fn; $parser => <$ast_type as Parse>::parse, $parsed_tokens $($in_between)?)
    };
    (fn; $parser:expr => $parsing_func:expr, $parsed_tokens:expr $(,in $in_between:expr)? $(, $arg:expr)* ) => {
        match $parsing_func($parser, $($arg),*) {
            Good(ast, tokens) => {
                $parsed_tokens.extend(tokens);
                $($in_between;)?
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
        span_toks!($id last $toks)
    };
    ($id:ident $tip:ident $toks:expr) => {
        $toks.$tip().unwrap().span.$id
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
