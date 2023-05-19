use crate::token::*;

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}

impl ParseNode {
    pub fn new(entry: GrammarItem) -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry,
        }
    }
}

pub fn print(tree: &ParseNode) -> String {
    match tree.entry {
        GrammarItem::Paren => {
            format!("({})",
                    print(tree.children.get(0).expect("parens need one child")))
        }
        GrammarItem::Sum => {
            let lhs = print(tree.children.get(0).expect("sums need two children"));
            let rhs = print(tree.children.get(1).expect("sums need two children"));
            format!("{} + {}", lhs, rhs)
        }
        GrammarItem::Sub => {
            let lhs = print(tree.children.get(0).expect("sums need two children"));
            let rhs = print(tree.children.get(1).expect("sums need two children"));
            format!("{} - {}", lhs, rhs)
        }
        GrammarItem::Product => {
            let lhs = print(tree.children.get(0).expect("products need two children"));
            let rhs = print(tree.children.get(1).expect("products need two children"));
            format!("{} * {}", lhs, rhs)
        }
        GrammarItem::Quotient => {
            let lhs = print(tree.children.get(0).expect("products need two children"));
            let rhs = print(tree.children.get(1).expect("products need two children"));
            format!("{} / {}", lhs, rhs)
        }
        GrammarItem::Float(n) => format!("{}", n),
        GrammarItem::Int(n) => format!("{}", n),
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&self) -> Result<ParseNode, String> {
        // TODO: Change the String in the result by a proper Error struct
        Self::parse_expr(&self.tokens, 0).and_then(|(n, i)| {
            if i == self.tokens.len() {
                Ok(n)
            } else {
                Err(format!(
                    "Expected end of input, found {:?} at {}",
                    self.tokens[i], i
                ))
            }
        })
    }

    pub fn parse_expr(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
        let (node_summand, next_pos) = Self::parse_summand(tokens, pos)?;
        let c = tokens.get(next_pos);
        match c {
            // Some(&Token::Plus | &Token::Minus) => {
            //     // recurse on the expr
            //     let mut sum = ParseNode::new(GrammarItem::try_from(*c.unwrap())?);
            //     sum.children.push(node_summand);
            //     let (rhs, i) = Self::parse_expr(tokens, next_pos + 1)?;
            //     sum.children.push(rhs);
            //     Ok((sum, i))
            // }
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
            // Some(&Token::Mul | &Token::Div) => {
            //     // recurse on the summand
            //     let mut product = ParseNode::new(GrammarItem::try_from(*c.unwrap())?);
            //     product.children.push(node_term);
            //     let (rhs, i) = Self::parse_summand(tokens, next_pos + 1)?;
            //     product.children.push(rhs);
            //     Ok((product, i))
            // }
            _ => {
                // we have just the term production, nothing more.
                Ok((node_term, next_pos))
            }
        }
    }

    pub fn parse_term(tokens: &Vec<Token>, pos: usize) -> Result<(ParseNode, usize), String> {
        let c: &Token = tokens.get(pos).ok_or(String::from(
            "Unexpected end of input, expected paren or number",
        ))?;
        match *c {
            Token::Int(n) => Ok((ParseNode::new(GrammarItem::Int(n)), pos + 1)),
            Token::Float(n) => Ok((ParseNode::new(GrammarItem::Float(n)), pos + 1)),
            Token::OpenParen => {
                Self::parse_expr(tokens, pos + 1).and_then(|(node, next_pos)| {
                    if let Some(&Token::CloseParen) = tokens.get(next_pos) {
                        // okay!
                        let mut paren = ParseNode::new(GrammarItem::Paren);
                        paren.children.push(node);
                        Ok((paren, next_pos + 1))
                    } else {
                        Err(format!(
                            "Expected closing paren at {} but found {:?}",
                            next_pos,
                            tokens.get(next_pos)
                        ))
                    }
                })
            }
            _ => Err(format!(
                "Unexpected token {:?}, expected paren or number",
                { c }
            )),
        }
    }
}
