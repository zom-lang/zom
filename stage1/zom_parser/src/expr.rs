//! Module responsible for parsing expression.
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Expression {
    pub expr: Expr,
    pub span: CodeSpan,
}

impl Parse for Expression {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();

        let lhs = parse_try!(parser => Expr, parsed_tokens);

        let result = match parser.last() {
            _ => lhs,
        };

        Good(result, parsed_tokens)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    BinaryExpr {
        lhs: Box<Expression>,
        op: BinOperation,
        rhs: Box<Expression>,
    },

    // Primary Expression
    IntLitExpr(u64),
    CharLitExpr(char),
    StrLitExpr(String),
    BoolLitExpr(bool),
    IdentifierExpr(String),
}

impl Parse for Expr {
    type Output = Expression;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        // Only parses Primary Expression, so not BinaryExpr and UnaryExpr
        match parser.last() {
            Token { tt: T::Int(_), .. } => parse_intlit_expr(parser),
            Token { tt: T::Char(_), .. } => parse_charlit_expr(parser),
            Token { tt: T::Str(_), .. } => parse_strlit_expr(parser),
            Token {
                tt: T::True | T::False,
                ..
            } => parse_boollit_expr(parser),
            Token {
                tt: T::Ident(_), ..
            } => parse_identifier_expr(parser),
            found => Error(Box::new(ExpectedToken::from(found, PartAST::Expression))),
        }
    }
}

pub fn parse_intlit_expr(parser: &mut Parser) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let int = expect_token!(parser => [T::Int(i), i.clone()], IntLit, parsed_tokens);

    Good(
        Expression {
            expr: Expr::IntLitExpr(int),
            span: span_toks!(parsed_tokens),
        },
        parsed_tokens,
    )
}

pub fn parse_charlit_expr(parser: &mut Parser) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let char = expect_token!(parser => [T::Char(c), c.clone()], IntLit, parsed_tokens);

    Good(
        Expression {
            expr: Expr::CharLitExpr(char),
            span: span_toks!(parsed_tokens),
        },
        parsed_tokens,
    )
}

pub fn parse_strlit_expr(parser: &mut Parser) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let str = expect_token!(parser => [T::Str(s), s.clone()], IntLit, parsed_tokens);

    Good(
        Expression {
            expr: Expr::StrLitExpr(str),
            span: span_toks!(parsed_tokens),
        },
        parsed_tokens,
    )
}

pub fn parse_boollit_expr(parser: &mut Parser) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let bool = expect_token!(parser => [T::True, true; T::False, false], IntLit, parsed_tokens);

    Good(
        Expression {
            expr: Expr::BoolLitExpr(bool),
            span: span_toks!(parsed_tokens),
        },
        parsed_tokens,
    )
}

pub fn parse_identifier_expr(parser: &mut Parser) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let int = expect_token!(parser => [T::Ident(name), name.clone()], IntLit, parsed_tokens);

    Good(
        Expression {
            expr: Expr::IdentifierExpr(int),
            span: span_toks!(parsed_tokens),
        },
        parsed_tokens,
    )
}
