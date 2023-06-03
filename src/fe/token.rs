//! This is the token of Mona
//!
//! It is in its own file because later on, there will be lot of tokens type.

use std::fmt;

pub use Token::{
    CloseParen, Comma, Delimiter, Extern, Float, Func, Ident, Int, OpenParen, Operator,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Operators
    Operator(String),

    // Structural symbols
    OpenParen,    // ` ( ` 
    CloseParen,   // ` ) ` 

    OpenBracket,  // ` [ ` 
    CloseBracket, // ` ] ` 

    OpenBrace,    // ` { ` 
    CloseBrace,   // ` } ` 

    Colon,
    Delimiter,    // ` ; ` 
    Comma,        // ` , ` 

    // Literals
    Int(i32),
    Float(f32),

    // Keywords
    Func,
    Extern,

    // Identifier
    Ident(String), // Identifier is a alphanumeric string
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Token::Int(val) = &self {
            return write!(f, "Int:{}", val);
        } else if let Token::Float(val) = &self {
            return write!(f, "Float:{}", val);
        }
        write!(f, "{:?}", &self)
    }
}
