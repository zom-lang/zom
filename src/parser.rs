use crate::Token;
use std::fmt;

pub trait Node: fmt::Display {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct NumberNode {
    pub token: Token,
}
impl Node for NumberNode {}

impl NumberNode {
    pub fn new(token: Token) -> NumberNode {
        NumberNode { token }
    }
}

impl fmt::Display for NumberNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

#[derive(Debug, PartialEq)]
pub struct BinOpNode {
    pub op_token: Token,
    left_node: NumberNode,
    right_node: NumberNode,
}
impl Node for BinOpNode {}

impl BinOpNode {
    pub fn new(left_node: NumberNode, op_token: Token, right_node: NumberNode) -> BinOpNode {
        BinOpNode {
            op_token,
            left_node,
            right_node,
        }
    }
}

impl fmt::Display for BinOpNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.right_node, self.op_token, self.left_node
        )
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    tok_index: usize,
    current_tok: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut parser = Parser {
            tokens,
            tok_index: 0,
            current_tok: None,
        };
        parser.advance();
        parser
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.tok_index += 1;
        if self.tok_index < self.tokens.len() {
            self.current_tok = Some(self.tokens[self.tok_index]);
            return self.current_tok;
        }
        None
    }

    pub fn factor(&mut self) -> Option<NumberNode> {
        self.current_tok?;
        let token = self.current_tok.unwrap();

        if let Token::Int(_) = token {
            self.advance();
            return Some(NumberNode::new(token));
        }
        if let Token::Float(_) = token {
            self.advance();
            return Some(NumberNode::new(token));
        }
        None
    }

    pub fn term(&mut self) -> Option<BinOpNode> {
        let left = self.factor().unwrap();
        let mut bin_op = None;

        if self.current_tok.is_none() {
            // return None;
        }
        let token = self.current_tok.unwrap();

        while let Token::Mul | Token::Div = token {
            let op_tok = self.current_tok.unwrap();
            self.advance();
            let right = self.factor().unwrap();
            bin_op = Some(BinOpNode::new(left, op_tok, right));
        }

        bin_op
    }

    pub fn expr(&self) {
        todo!()
    }
}
