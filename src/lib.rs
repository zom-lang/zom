use std::fmt::{self, Result};

pub struct Token<T, V> {
    _type: T,
    value: Box<V>,
}

// impl<T, V> fmt::Display for Token<T, V> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // write!(f, "{}, {}", self._type, self.value)
//
//     }
// }
