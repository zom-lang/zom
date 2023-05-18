use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    // Operators
    Plus,
    Minus,
    Mul,
    Div,

    // Separators
    LParen,
    RParen,

    // Literals
    Int(i32),
    Float(f32),
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
    type Error = &'static str;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Self::Sum),
            Token::Minus => Ok(Self::Sub),
            Token::Mul => Ok(Self::Product),
            Token::Div => Ok(Self::Quotient),

            Token::LParen | Token::RParen => Ok(Self::Paren),

            Token::Int(v) => Ok(Self::Int(v)),
            Token::Float(v) => Ok(Self::Float(v)),

            // _ => Err("Not implemented yet.")
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
