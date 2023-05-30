use std::fmt;

pub use Token::{
    CloseParen, Comma, Delimiter, Extern, Float, Func, Ident, Int, OpenParen, Operator,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Operators
    Operator(char),

    // Separators
    OpenParen,  // means `open parentheis`
    CloseParen, // means `close parentheis`
    Delimiter,  // ` ; ` character
    Comma,      // ` , ` character

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
