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
