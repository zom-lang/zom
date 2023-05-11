use crate::Token;

pub struct NumberNode {
    pub token: Token,
}

impl NumberNode {
    pub fn new(token: Token) -> NumberNode {
        NumberNode { token }
    }
}