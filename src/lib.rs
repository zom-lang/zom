use std::fmt;

#[derive(Debug)]
pub struct Token<T, V> {
    type_: T,
    value: Option<V>,
}

impl<T, V> Token<T, V> {
    pub fn new(type_: T, value: Option<V>) -> Token<T, V> {
        Token { type_, value }
    }
}

impl<T: fmt::Display, V: fmt::Debug> fmt::Display for Token<T, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(val) => write!(f, "{}:{:?}", self.type_, val),
            None => write!(f, "{}", self.type_),
        }
    }
}
