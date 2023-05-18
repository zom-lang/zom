use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    // Operators
    Plus,
    Minus,
    Mul,
    Div,

    LParen,
    RParen,

    // Literals
    Int(i32),
    Float(f32),
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let TokenType::Int(val) = &self {
            return write!(f, "Int:{}", val);
        } else if let TokenType::Float(val) = &self {
            return write!(f, "Float:{}", val);
        }
        write!(f, "{:?}", &self)
    }
}

/// This is a much lighter version of the Position struct, it's used to track where a token was in the text code.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LightPosition {
    line: usize,
    column: usize,
}

impl LightPosition {
    pub fn new(line: usize, column: usize) -> LightPosition {
        LightPosition { line, column }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    token: TokenType,
    pos_start: LightPosition,
    pos_end: LightPosition,
}

impl Token {
    pub fn new(token: TokenType, pos_start: LightPosition, pos_end: LightPosition) -> Token {
        Token {
            token,
            pos_start,
            pos_end,
        }
    }

    pub fn get_toktype(&self) -> TokenType {
        self.token
    }

    pub fn get_pos_start(&self) -> LightPosition {
        self.pos_start
    }

    pub fn get_pos_end(&self) -> LightPosition {
        self.pos_end
    }
}
