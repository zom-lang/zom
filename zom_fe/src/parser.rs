//! This is the parser of Zom
//!
//! It is entirely made for Zom, without using dependencies.

use std::collections::HashMap;
use std::ops::Range;

use zom_common::error::ZomError;
use zom_common::token::Token;
use zom_common::token::*;

use self::item::{parse_item, Item};
use self::PartParsingResult::*;

pub mod block;
pub mod expr;
pub mod function;
pub mod item;
pub mod statement;
pub mod symbol;
pub mod types;

pub type ParsingResult = Result<(Vec<Item>, Vec<Token>), Vec<ZomError>>;

#[derive(Debug)]
pub enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(ZomError),
}

#[derive(Debug)]
pub struct ParsingContext {
    pub pos: usize,
    pub filename: String,
    pub source_file: String,
    errors: Vec<ZomError>,
}

impl ParsingContext {
    pub fn new(filename: String, source_file: String) -> ParsingContext {
        ParsingContext {
            pos: 0,
            filename,
            source_file,
            errors: vec![],
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn push_err(&mut self, err: ZomError) {
        self.errors.push(err);
    }

    pub fn push_errors(&mut self, errors: Vec<ZomError>) {
        self.errors.extend(errors);
    }
}

/// err_et mean `error expected token`
#[macro_export]
macro_rules! err_et(
    ($context:expr, $last_token:expr, $expected:expr, $found:expr, $end:stmt) => (
        {
            use zom_common::error::{Position, ZomError};
            use zom_common::token::TokenType;
            if $expected.is_empty() {
                panic!("One or more expected values are needed.");
            }
            $context.push_err(ZomError::new(
                Position::try_from_range(
                    $context.pos,
                    $last_token.span.clone(),
                    $context.source_file.clone(),
                    $context.filename.clone().into()
                ),
                if $expected.len() == 1 {
                    format!("expected {}, found {}", $expected[0], $found)
                }else {
                    format!("expected one of {}, found {}", TokenType::format_toks($expected), $found)
                },
                false,
                None,
                vec![]
            ));
            $end
        }
    );

    ($context:expr, $last_token:expr, $expected:expr, $found:expr) => (
        {
            use zom_common::error::{Position, ZomError};
            use zom_common::token::TokenType;
            if $expected.is_empty() {
                panic!("One or more expected values are needed.");
            }
            Bad(ZomError::new(
                Position::try_from_range(
                    $context.pos,
                    $last_token.span.clone(),
                    $context.source_file.clone(),
                    $context.filename.clone().into()
                ),
                if $expected.len() == 1 {
                    format!("expected {}, found {}", $expected[0], $found)
                }else {
                    format!("expected one of {}, found {}", TokenType::format_toks($expected), $found)
                },
                false,
                None,
                vec![]
            ))
        }
    );
);

#[derive(Debug)]
pub struct ParserSettings {
    /// Binary operator precedence
    operator_precedence: HashMap<Operator, i32>,
}

impl Default for ParserSettings {
    fn default() -> Self {
        use zom_common::token::{Operator::*, *};
        let mut operator_precedence = HashMap::with_capacity(19);

        // Setup Operator Precedence according to the documentation

        operator_precedence.insert(Mul, PR_MUL_DIV_REM);
        operator_precedence.insert(Div, PR_MUL_DIV_REM);
        operator_precedence.insert(Rem, PR_MUL_DIV_REM);

        operator_precedence.insert(Add, PR_ADD_SUB);
        operator_precedence.insert(Sub, PR_ADD_SUB);

        operator_precedence.insert(RShift, PR_SHIFT);
        operator_precedence.insert(LShift, PR_SHIFT);

        operator_precedence.insert(CompLT, PR_COMP);
        operator_precedence.insert(CompGT, PR_COMP);
        operator_precedence.insert(CompLTE, PR_COMP);
        operator_precedence.insert(CompGTE, PR_COMP);

        operator_precedence.insert(CompEq, PR_COMP_EQ_NE);
        operator_precedence.insert(CompNe, PR_COMP_EQ_NE);

        operator_precedence.insert(BitAnd, PR_BIT_AND);
        operator_precedence.insert(BitXor, PR_BIT_XOR);
        operator_precedence.insert(BitOr, PR_BIT_OR);

        operator_precedence.insert(LogicAnd, PR_LOGIC_AND);
        operator_precedence.insert(LogicOr, PR_LOGIC_OR);

        operator_precedence.insert(Equal, PR_EQ);

        ParserSettings {
            operator_precedence,
        }
    }
}

pub fn parse(
    tokens: &[Token],
    parsed_tree: &[Item],
    settings: &mut ParserSettings,
    mut context: ParsingContext,
) -> ParsingResult {
    let mut rest = tokens.to_vec();
    // we read tokens from the end of the vector
    // using it as a stack
    rest.reverse();

    // we will add new AST nodes to already parsed ones
    let mut ast = parsed_tree.to_vec();

    while let Some(item) = parse_item(&mut rest, settings, &mut context) {
        match item {
            Good(ast_node, _) => ast.push(ast_node),
            NotComplete => break,
            Bad(err) => {
                context.push_err(err);
                return Err(context.errors); // TODO: try to not return here and keep parsing items.
            }
        }
    }

    if !context.errors.is_empty() {
        return Err(context.errors);
    }

    // unparsed tokens
    rest.reverse();
    Ok((ast, rest))
}

#[macro_export]
macro_rules! parse_try(
    ($function:ident, $tokens:ident, $settings:ident, $context:ident, $parsed_tokens:ident) => (
        parse_try!($function, $tokens, $settings, $context, $parsed_tokens,)
    );

    ($function:ident, $tokens:ident, $settings:ident, $context:ident, $parsed_tokens:ident, $($arg:expr),*) => (
        match $function($tokens, $settings, $context, $($arg),*) {
            Good(ast, toks) => {
                $parsed_tokens.extend(toks.into_iter());
                ast
            }
            NotComplete => {
                $parsed_tokens.reverse();
                $tokens.extend($parsed_tokens.into_iter());
                return NotComplete;
            }
            Bad(error) => return Bad(error)
        }
    )
);

#[macro_export]
macro_rules! expect_token (
    ($context:ident, [ $($token:pat, $value:expr, $result:stmt);+ ] <= $tokens:ident, $parsed_tokens:ident, $error:expr) => (
        match $tokens.pop() { // Where instead if .pop() use .last()
            $(
                Some(Token { tt: $token, span }) => { // And .pop()
                    $context.advance();
                    $parsed_tokens.push(Token { tt: $value, span });
                    $result
                },
             )+
             None => { // or here, like that in the err_et!() we can use .last() to have the token that hasn't been matched.
                $context.advance();
                $parsed_tokens.reverse();
                $tokens.extend($parsed_tokens.into_iter());
                return NotComplete;
             },
            _ => { $context.advance(); return $error } // TODO: try to move err_et!(..) here.
        }
    );

    ($context:ident, [ $($token:pat, $value:expr, $result:stmt);+ ] else $not_matched:block <= $tokens:ident, $parsed_tokens:ident) => (
        $context.advance();
        match $tokens.last().map(|i| {i.clone()}) {
            $(
                Some(Token { tt: $token, span}) => {
                    $tokens.pop();
                    $parsed_tokens.push(Token { tt: $value, span });
                    $result
                },
             )+
            _ => {$not_matched}
        }
    )
);

/// This macro is to test the equality of a token but without checking the span.
/// return true if it's equal or false if it's not.
#[macro_export]
macro_rules! token_parteq(
    ($left:expr, $right:expr) => (
        match $left {
            Some(Token { tt, span: _}) if tt == $right => true,
            _ => false
        }
    );

    (no_opt $left:expr, $right:expr) => (
        match $left {
            Token { ref tt, span: _} if *tt == $right => true,
            _ => false
        }
    );
);

pub trait CodeLocation {
    fn span(&self) -> Range<usize>;
}

#[macro_export]
macro_rules! impl_span(
    ($ast:ident) => (
        impl_span!($ast, span);
    );
    ($ast:ident, $span_field:ident) => (
        impl $crate::parser::CodeLocation for $ast {
            fn span(&self) -> Range<usize> {
                self.$span_field.clone()
            }
        }
    )
);
