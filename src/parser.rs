use crate::Token;

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: Token,
}

impl ParseNode {
    pub fn new(entry: Token) -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry,
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

    pub fn parse(&self) -> Result<ParseNode, String> { // TODO: Change the String in the result by a proper Error struct
        Self::parse_expr(&self.tokens, 0).and_then(|(n, i)| if i == self.tokens.len() {
            Ok(n)
        } else {
            Err(format!("Expected end of input, found {:?} at {}", self.tokens[i], i))
        })
    }

    pub fn parse_expr(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
        let (node_summand, next_pos) = Self::parse_summand(tokens, pos)?;
        let c = tokens.get(next_pos);
        match c {
            Some(&Token::Plus | &Token::Minus) => {
                // recurse on the expr
                let mut sum = ParseNode::new(*c.unwrap());
                sum.children.push(node_summand);
                let (rhs, i) = Self::parse_expr(tokens, next_pos + 1)?;
                sum.children.push(rhs);
                Ok((sum, i))
            }
            _ => {
                // we have just the summand production, nothing more.
                Ok((node_summand, next_pos))
            }
        }
    }

    pub fn parse_summand(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
        let (node_term, next_pos) = Self::parse_term(tokens, pos)?;
        let c = tokens.get(next_pos);
        match c {
            Some(&Token::Mul | &Token::Div) => {
                // recurse on the summand
                let mut product = ParseNode::new(*c.unwrap());
                product.children.push(node_term);
                let (rhs, i) = Self::parse_summand(tokens, next_pos + 1)?;
                product.children.push(rhs);
                Ok((product, i))
            }
            _ => {
                // we have just the term production, nothing more.
                Ok((node_term, next_pos))
            }
        }
    }

    pub fn parse_term(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
        let c: &Token = tokens.get(pos)
            .ok_or(String::from("Unexpected end of input, expected paren or number"))?;
        match c {
            &Token::Int(n) => {
                Ok((ParseNode::new(Token::Int(n)), pos + 1))
            }
            &Token::LParen => {
                Self::parse_expr(tokens, pos + 1).and_then(|(node, next_pos)| {
                    if let Some(&Token::RParen) = tokens.get(next_pos) {
                        // okay!
                        Ok((node, next_pos + 1))
                    } else {
                        Err(format!("Expected closing paren at {} but found {:?}",
                                    next_pos,
                                    tokens.get(next_pos)))
                    }
                })
            }
            _ => {
                Err(format!("Unexpected token {:?}, expected paren or number", {
                    c
                }))
            }
        }
    }
}
