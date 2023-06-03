//! This is the token of Mona
//!
//! It is in its own file because later on, there will be lot of tokens type.

use std::fmt;

pub use Token::*;

/// Plus, `+`
pub const OP_PLUS: &str  = "+";
/// Minus, `-`
pub const OP_MINUS: &str = "-";
/// Mul, `+`
pub const OP_MUL: &str   = "*";
/// Div, `/`
pub const OP_DIV: &str   = "/";
/// Mod (remainder), `%`
pub const OP_MOD: &str   = "%";
/// Power, `^`
pub const OP_POWER: &str = "^";

/// Equal, `=`, used to assignement
pub const OP_EQ: &str = "=";

/// Compare Equality, `==`
pub const OP_COMP_EQ: &str  = "==";
/// Compare Non-Equality, `!=`
pub const OP_COMP_NE: &str  = "!=";
/// Compare Greater than, `>`
pub const OP_COMP_GT: &str  = ">";
/// Compare Less than, `<`
pub const OP_COMP_LT: &str  = "<";
/// Compare Greater Than or Equal to, `=>`
pub const OP_COMP_GTE: &str = "=>";
/// Compare Less than or Equal to, `=<`
pub const OP_COMP_LTE: &str = "=<";

/// Logical OR, `||`
pub const OP_OR: &str  = "||";
/// Logical AND, `||`
pub const OP_AND: &str = "&&";

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

    Colon,        // ` : `
    Delimiter,    // ` ; ` 
    Comma,        // ` , ` 

    // Literals
    Int(i32),
    Float(f32),

    // Keywords
    Func,
    Extern,
    Let,

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
