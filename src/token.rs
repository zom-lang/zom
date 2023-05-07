use std::fmt;

#[derive(Debug)]
pub enum Token {
    Int(i32),
    Float(f32),
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Token::Int(val) = &self {
            println!("This is an int");
            return write!(f, "Int:{}", val);
        } else if let Token::Float(val) = &self {
            println!("This is a float");
            return write!(f, "Float:{}", val);
        }
        write!(f, "{:?}", &self)
    }
}
