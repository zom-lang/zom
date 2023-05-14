use crate::Token;
use crate::error::GeneralError;
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: Token,
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: Token::LParen,
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
        }
    }

    pub fn parse(&self) {

    }
}
