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

        let mut result = lhs;
        loop {
            result = match &parser.last().tt {
                T::Oper(op) if BinOperation::try_from(op.clone()).is_ok() => {
                    parse_try!(fn; parser => parse_binary_expr, parsed_tokens, 0, &result)
                }
                T::OpenParen => parse_try!(fn; parser => parse_call_expr, parsed_tokens, &result),
                T::Oper(Operator::Dot) => {
                    parse_try!(fn; parser => parse_member_access_expr, parsed_tokens, &result)
                }
                _ => break,
            };
        }

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
    CallExpr {
        fn_op: Box<Expression>,
        args: Vec<Expression>,
    },
    MemberAccessExpr {
        expr: Box<Expression>,
        member_name: String,
    },

    // Primary Expression
    IntLitExpr(u64),
    CharLitExpr(char),
    StrLitExpr(String),
    BoolLitExpr(bool),
    IdentifierExpr(String),
    ParenthesizedExpr(Box<Expression>),
}

impl Parse for Expr {
    type Output = Expression;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        // Only parses Primary Expression, so not BinaryExpr and UnaryExpr
        match &parser.last().tt {
            T::Int(_) => parse_intlit_expr(parser),
            T::Char(_) => parse_charlit_expr(parser),
            T::Str(_) => parse_strlit_expr(parser),
            T::True | T::False => parse_boollit_expr(parser),
            T::Ident(_) => parse_identifier_expr(parser),
            T::OpenParen => parse_parenthesized_expr(parser),
            _ => Error(Box::new(ExpectedToken::from(
                parser.last(),
                PartAST::Expression,
            ))),
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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum BinOperation {
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    RShift,
    LShift,
    CompLT,
    CompGT,
    CompLTE,
    CompGTE,
    CompEq,
    CompNe,
    And,
    Or,
    Xor,
}

impl TryFrom<Operator> for BinOperation {
    type Error = ();

    fn try_from(op: Operator) -> Result<Self, Self::Error> {
        use self::BinOperation as BOp;
        use zom_common::token::Operator::*;
        Ok(match op {
            Ampersand => BOp::And,
            Asterisk => BOp::Mul,
            Caret => BOp::Xor,
            Equal2 => BOp::CompEq,
            ExclamationmarkEqual => BOp::CompNe,
            LArrow => BOp::CompLT,
            LArrow2 => BOp::LShift,
            LArrowEqual => BOp::CompLTE,
            Minus => BOp::Sub,
            Percent => BOp::Rem,
            Pipe2 => BOp::Or,
            Plus => BOp::Add,
            RArrow => BOp::CompGT,
            RArrow2 => BOp::RShift,
            RArrowEqual => BOp::CompGTE,
            Slash => BOp::Div,
            _ => return Err(()),
        })
    }
}

#[repr(u16)]
#[derive(Clone, Debug, PartialEq)]
pub enum Associativity {
    /// Left to Right
    L2R,
    /// Right to Left
    R2L,
}

pub fn parse_binary_expr(
    parser: &mut Parser,
    min_precedence: u16,
    lhs: &Expression,
) -> ParsingResult<Expression> {
    // TODO: handle post & pre unary expr in the same parsing function
    let mut parsed_tokens = Vec::new();
    let mut lhs = lhs.clone();

    while let Token {
        tt: T::Oper(op), ..
    } = parser.last()
    {
        // check if it's a binary operator
        let op = match BinOperation::try_from(op.clone()) {
            Ok(v) => v,
            Err(()) => break,
        };

        // get precedence of the bin op
        let (_, op_precede) = parser.pr_get(op.clone());
        if op_precede < min_precedence {
            break;
        }

        // we only pop the bin op now because if it wasn't a bin op
        // it would of get poped, and we don't want that.
        parser.pop();

        // parse the right-hand side of the binary expr
        let mut rhs = parse_try!(parser => Expr, parsed_tokens);

        while let Token {
            tt: T::Oper(lh_op), ..
        } = parser.last()
        {
            // check if it's a binary operator
            let lh_op = match BinOperation::try_from(lh_op.clone()) {
                Ok(v) => v,
                Err(()) => break,
            };

            // get the precedence of lookahead operator
            let (lh_assoc, lh_op_precede) = parser.pr_get(lh_op.clone());

            // break of the inner loop if the precedence of the lookahead operator is
            // less or equal if Associativity is left to right
            match lh_assoc {
                Associativity::L2R if lh_op_precede <= op_precede => break,
                Associativity::R2L if lh_op_precede < op_precede => break,
                _ => {}
            }
            rhs = parse_try!(fn; parser => parse_binary_expr, parsed_tokens, lh_op_precede, &rhs);
        }
        // compute the span of the bin expr
        let span = lhs.span.start..rhs.span.end;

        // merge the rhs and op with lhs
        lhs = Expression {
            expr: Expr::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            },
            span,
        };
    }

    Good(lhs, parsed_tokens)
}

pub fn parse_call_expr(parser: &mut Parser, lhs: &Expression) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();
    let fn_op = Box::new(lhs.clone());

    let start = fn_op.span.start;

    expect_token!(parser => [T::OpenParen, ()], OpenParen, parsed_tokens);

    let mut args = Vec::new();
    loop {
        expect_token!(parser => [T::Comma, if args.is_empty() { continue };T::CloseParen, break] else {}, parsed_tokens);
        args.push(parse_try!(parser => Expression, parsed_tokens));
    }

    expect_token!(parser => [T::CloseParen, ()], CloseParen, parsed_tokens);

    let end = span_toks!(end parsed_tokens);

    Good(
        Expression {
            expr: Expr::CallExpr { fn_op, args },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_parenthesized_expr(parser: &mut Parser) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    expect_token!(parser => [T::OpenParen, ()], OpenParen, parsed_tokens);
    let start = span_toks!(start parsed_tokens);

    let expr = parse_try!(parser => Expression, parsed_tokens);

    expect_token!(parser => [T::CloseParen, ()], CloseParen, parsed_tokens);
    let end = span_toks!(end parsed_tokens);

    Good(
        Expression {
            expr: Expr::ParenthesizedExpr(Box::new(expr)),
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_member_access_expr(
    parser: &mut Parser,
    lhs: &Expression,
) -> ParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let expr = Box::new(lhs.clone());
    let start = expr.span.start;

    expect_token!(parser => [T::Oper(Operator::Dot), ()], Dot, parsed_tokens);

    let member_name = expect_token!(parser => [T::Ident(name), name.clone()], Ident, parsed_tokens);
    let end = span_toks!(end parsed_tokens);

    Good(
        Expression {
            expr: Expr::MemberAccessExpr { expr, member_name },
            span: start..end,
        },
        parsed_tokens,
    )
}

#[derive(Debug)]
pub struct ExpressionList(pub Vec<Expression>);

impl ExpressionList {
    pub fn parse(parser: &mut Parser) -> ParsingResult<ExpressionList> {
        let mut parsed_tokens = Vec::new();

        let mut exprs = Vec::new();
        loop {
            exprs.push(parse_try!(parser => Expression, parsed_tokens));
            expect_token!(parser => [T::Comma, ()] else { break }, parsed_tokens);
        }

        Good(ExpressionList(exprs), parsed_tokens)
    }

    pub fn parse_with(parser: &mut Parser, first: Expression) -> ParsingResult<ExpressionList> {
        let mut parsed_tokens = Vec::new();

        let mut exprs = vec![first];
        loop {
            expect_token!(parser => [T::Comma, ()] else { break }, parsed_tokens);
            exprs.push(parse_try!(parser => Expression, parsed_tokens));
        }

        Good(ExpressionList(exprs), parsed_tokens)
    }
}

impl Parse for ExpressionList {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<ExpressionList> {
        let mut parsed_tokens = Vec::new();

        let mut exprs = Vec::new();
        loop {
            exprs.push(parse_try!(parser => Expression, parsed_tokens));
            expect_token!(parser => [T::Comma, ()] else { break }, parsed_tokens);
        }

        Good(ExpressionList(exprs), parsed_tokens)
    }
}
