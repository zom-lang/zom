use std::fmt;
// TT = Token Type
pub const TT_INT: &str = "TT_INT";
pub const TT_FLOAT: &str = "FLOAT";
pub const TT_PLUS: &str = "PLUS";
pub const TT_MINUS: &str = "MINUS";
pub const TT_MUL: &str = "MUL";
pub const TT_DIV: &str = "DIV";
pub const TT_LPAREN: &str = "LPAREN";
pub const TT_RPAREN: &str = "RPAREN";

#[derive(Debug)]
pub struct Token<V> {
    type_: String,
    value: Option<V>,
}

impl<V> Token<V> {
    pub fn new(type_: &str, value: Option<V>) -> Token<V> {
        Token {
            type_: type_.to_string(),
            value,
        }
    }
}

impl<V: fmt::Debug> fmt::Display for Token<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(val) => write!(f, "{}:{:?}", self.type_, val),
            None => write!(f, "{}", self.type_),
        }
    }
}
