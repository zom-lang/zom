//! fe means `front end`.
//!
//! The front end of Zom is the Lexer, Token, Parser and AST.


use parser::ParsingContext;
use zom_common::{error::parser::UnexpectedTokenError, token::Token};

pub mod lexer;
pub mod parser;

pub trait FromContext {
    fn from_context(context: &mut ParsingContext, details: String, token: Token) -> Self;
}

impl FromContext for UnexpectedTokenError {
    fn from_context(context: &mut ParsingContext, details: String, token: Token) -> Self {
        UnexpectedTokenError::from_pos(context.pos, context.full_tokens.clone(), &mut context.source_file, &mut context.filename, details, token)
    }
}
