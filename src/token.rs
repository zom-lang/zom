use std::fmt;

pub use Token::{
    CloseParen, Comma, Delimiter, Extern, Float, Func, Ident, Int, OpenParen, Operator,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Operators
    Operator(String),

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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GrammarItem {
    Product,
    Quotient,
    Sum,
    Sub,

    Int(i32),
    Float(f32),

    Paren,
}

impl TryFrom<Token> for GrammarItem {
    type Error = String;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Operator(c) => match c.as_str() {
                "+" => Ok(Self::Sum),
                "-" => Ok(Self::Sub),
                "*" => Ok(Self::Product),
                "/" => Ok(Self::Quotient),
                _ => Err(format!("The `{c}` is not an operator!")),
            },

            Token::OpenParen | Token::CloseParen => Ok(Self::Paren),

            Token::Int(v) => Ok(Self::Int(v)),
            Token::Float(v) => Ok(Self::Float(v)),

            _ => Err("Not implemented yet.".to_string()),
        }
    }
}

impl fmt::Display for GrammarItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let GrammarItem::Int(val) = &self {
            return write!(f, "Int:{}", val);
        } else if let GrammarItem::Float(val) = &self {
            return write!(f, "Float:{}", val);
        }
        write!(f, "{:?}", &self)
    }
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
