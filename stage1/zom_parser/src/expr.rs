//! This module contains the parsing for expressions.

use crate::prelude::*;

use self::Expr::*;

use crate::block::{parse_block_expr, Block};

// it's imported to disambiguate global imports of the prelude
use zom_common::token::Operator;

#[derive(PartialEq, Clone, Debug)]
pub struct Expression {
    pub expr: Expr,
    pub span: Range<usize>,
}

impl_span!(Expression);

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    LiteralExpr(i32),
    VariableExpr(String),
    BinaryExpr {
        op: BinOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    CallExpr(String, Vec<Expression>),
    BlockExpr(Block),
    BooleanExpr(bool),
    UndefinedExpr,
    ConditionalExpr {
        cond: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Option<Expression>>,
        /// Semi colon needed ?
        sc_need: bool,
    },
    ReturnExpr(Option<Box<Expression>>),
    LeftUnaryExpr {
        unary_op: UnaryOperator,
        expr: Box<Expression>,
    },
}

impl Expression {
    pub fn is_semicolon_needed(&self) -> bool {
        match *self {
            Expression {
                expr: BlockExpr(_), ..
            } => false,
            Expression {
                expr: ConditionalExpr { sc_need, .. },
                ..
            } => sc_need,
            _ => true,
        }
    }
}

pub fn parse_primary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    match tokens.last() {
        Some(Token { tt: Ident(_), .. }) => parse_ident_expr(tokens, settings, context),
        Some(Token { tt: Int(_), .. }) => parse_literal_expr(tokens, settings, context),
        Some(Token { tt: OpenParen, .. }) => parse_parenthesis_expr(tokens, settings, context),
        Some(Token { tt: OpenBrace, .. }) => parse_block_expr(tokens, settings, context),
        Some(Token {
            tt: True | False, ..
        }) => parse_boolean_expr(tokens, settings, context),
        Some(Token { tt: Undefined, .. }) => parse_undefined_expr(tokens, settings, context),
        Some(Token { tt: If, .. }) => parse_conditional_expr(tokens, settings, context),
        Some(Token { tt: Return, .. }) => parse_return(tokens, settings, context),
        Some(Token {
            tt: Operator(_), ..
        }) => parse_unary_expr(tokens, settings, context),
        None => NotComplete,
        _ => err_et!(
            context,
            tokens.last().unwrap(),
            vec![Ident(String::new()), Int(0), OpenParen, OpenBrace],
            tokens.last().unwrap().tt
        ),
    }
}

pub fn parse_ident_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        context,
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Ident(String::new())],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().clone().span.start;

    let end = parsed_tokens.last().unwrap().clone().span.end;

    expect_token!(
        context,
        [OpenParen, OpenParen, ()]
        else {return Good(
            Expression { expr: VariableExpr(name), span: start..end },
            parsed_tokens)}
        <= tokens, parsed_tokens);

    let mut args = Vec::new();
    loop {
        expect_token!(
            context,
            [CloseParen, CloseParen, break]
            else {
                args.push(parse_try!(parse_expr, tokens, settings, context, parsed_tokens));
            }
            <= tokens, parsed_tokens
        );

        let t = tokens.last().unwrap().clone();
        expect_token!(
            context, [
            Comma, Comma, {};
            CloseParen, CloseParen, break
        ] <= tokens,
             parsed_tokens,
            err_et!(context, t, vec![Comma, CloseParen], t.tt)
        );
    }

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Expression {
            expr: CallExpr(name, args),
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_literal_expr(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let t: Token = tokens.last().unwrap().clone();
    let value = expect_token!(
        context,
        [Int(val), Int(val), val] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![Int(0), Float(0.0)], t.tt)
    );
    let start = parsed_tokens.last().unwrap().span.start;

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Expression {
            expr: LiteralExpr(value),
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_parenthesis_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat the opening parenthesis
    let mut parsed_tokens: Vec<Token> = vec![tokens.last().unwrap().clone()];
    let t = tokens.last().unwrap().clone();
    tokens.pop();

    let expr = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);

    expect_token!(
        context,
        [CloseParen, CloseParen, ()] <= tokens,
        parsed_tokens,
        {
            use zom_common::error::{Position, ZomError};
            Bad(ZomError::new(
                Position::try_from_range(
                    context.pos,
                    t.span.clone(),
                    context.source_file.clone(),
                    context.filename.clone().into(),
                ),
                "unclosed delimiter `)`".to_owned(),
                false,
                None,
                vec![],
            ))
        }
    );
    // idk if the span is correct.
    Good(expr, parsed_tokens)
}

pub fn parse_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();
    let lhs = parse_try!(parse_primary_expr, tokens, settings, context, parsed_tokens);
    let expr = parse_try!(
        parse_binary_expr,
        tokens,
        settings,
        context,
        parsed_tokens,
        0,
        &lhs
    );
    // idk if the span is correct.
    Good(expr, parsed_tokens)
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum BinOperator {
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
    Equal,
}

impl TryFrom<Operator> for BinOperator {
    type Error = ();

    fn try_from(op: Operator) -> Result<Self, Self::Error> {
        Ok(match op {
            Operator::Mul => BinOperator::Mul,
            Operator::Div => BinOperator::Div,
            Operator::Rem => BinOperator::Rem,
            Operator::Add => BinOperator::Add,
            Operator::Sub => BinOperator::Sub,
            Operator::RShift => BinOperator::RShift,
            Operator::LShift => BinOperator::LShift,
            Operator::CompLT => BinOperator::CompLT,
            Operator::CompGT => BinOperator::CompGT,
            Operator::CompLTE => BinOperator::CompLTE,
            Operator::CompGTE => BinOperator::CompGTE,
            Operator::CompEq => BinOperator::CompEq,
            Operator::CompNe => BinOperator::CompNe,
            Operator::And => BinOperator::And,
            Operator::Or => BinOperator::Or,
            Operator::Xor => BinOperator::Xor,
            Operator::Equal => BinOperator::Equal,
            _ => panic!("This is not a binary operator"),
        })
    }
}

pub fn parse_binary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
    expr_precedence: i32,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    // start with LHS value
    let mut result = lhs.clone();
    let mut parsed_tokens: Vec<Token> = Vec::new();

    // continue until the current token is not an operator
    // or it is an operator with precedence lesser than expr_precedence
    while let Some(Token {
        tt: Operator(op),
        span: _,
    }) = tokens.last()
    {
        let bin_op = match BinOperator::try_from(op.clone()) {
            Ok(v) => v,
            Err(_) => {
                return err_et!(
                    context,
                    tokens.last().unwrap(),
                    Vec::<TokenType>::new(),
                    tokens.last().unwrap().tt
                )
            }
        };
        let (operator, precedence) = match settings.bin_op_pr.get(&bin_op) {
            Some(pr) if *pr >= expr_precedence => (bin_op.clone(), *pr),
            None => {
                return err_et!(
                    context,
                    tokens.last().unwrap(),
                    Vec::<TokenType>::new(),
                    tokens.last().unwrap().tt
                )
            }
            _ => break,
        };
        parsed_tokens.push(tokens.last().unwrap().clone());
        tokens.pop();

        // parse primary RHS expression
        let mut rhs = parse_try!(parse_primary_expr, tokens, settings, context, parsed_tokens);

        // parse all the RHS operators until their precedence is
        // bigger than the current one
        while let Some(Token {
            tt: Operator(op),
            span: _,
        }) = tokens.last().cloned()
        {
            let bin_op = match BinOperator::try_from(op) {
                Ok(v) => v,
                Err(_) => {
                    return err_et!(
                        context,
                        tokens.last().unwrap(),
                        Vec::<TokenType>::new(),
                        tokens.last().unwrap().tt
                    )
                }
            };
            let binary_rhs = match settings.bin_op_pr.get(&bin_op).copied() {
                Some(pr) if pr > precedence => {
                    parse_try!(
                        parse_binary_expr,
                        tokens,
                        settings,
                        context,
                        parsed_tokens,
                        pr,
                        &rhs
                    )
                }
                None => {
                    return err_et!(
                        context,
                        tokens.last().unwrap(),
                        Vec::<TokenType>::new(),
                        tokens.last().unwrap().tt
                    )
                }
                _ => break,
            };

            rhs = binary_rhs;
        }

        // merge LHS and RHS
        result = Expression {
            expr: BinaryExpr {
                op: operator,
                lhs: Box::new(result),
                rhs: Box::new(rhs.clone()),
            },
            span: lhs.span.start..rhs.span.end,
        };
    }

    Good(result, parsed_tokens)
}

pub fn parse_boolean_expr(
    tokens: &mut Vec<Token>,
    _: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat either True or False keyword
    let mut parsed_tokens = vec![];
    let bool = expect_token!(
        context,
        [True, True, true;
         False, False, false] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![True, False],
            tokens.last().unwrap().tt
        )
    );

    let span = parsed_tokens.last().unwrap().span.clone();

    Good(
        Expression {
            expr: Expr::BooleanExpr(bool),
            span,
        },
        parsed_tokens,
    )
}

pub fn parse_undefined_expr(
    tokens: &mut Vec<Token>,
    _: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat Undefined keyword
    let mut parsed_tokens = Vec::new();
    expect_token!(
        context,
        [Undefined, Undefined, ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Undefined],
            tokens.last().unwrap().tt
        )
    );
    Good(
        Expression {
            expr: Expr::UndefinedExpr,
            span: parsed_tokens.last().unwrap().span.clone(),
        },
        parsed_tokens,
    )
}

pub fn parse_conditional_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    expect_token!(
        context,
        [If, If, ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![If],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

    let starts_paren = token_parteq!(tokens.last().cloned(), OpenParen);
    dbg!(&starts_paren);

    let cond = Box::new(parse_try!(
        parse_expr,
        tokens,
        settings,
        context,
        parsed_tokens
    ));
    dbg!(&cond);

    let then_expr = Box::new(parse_try!(
        parse_expr,
        tokens,
        settings,
        context,
        parsed_tokens
    ));
    dbg!(&then_expr);

    if let Expression {
        expr: BlockExpr(..),
        ..
    } = *then_expr
    {
    } else if !starts_paren {
        context.push_err(ZomError::new(
            Position::try_from_range(
                context.pos,
                cond.span.clone(),
                context.source_file.clone(),
                context.filename.clone().into(),
            ),
            "unparenthesized condition when their is no block".to_string(),
            false,
            Some("wrap the condition in parentheses".to_string()),
            vec![],
        ))
    }

    let else_expr = Box::new(if token_parteq!(tokens.last().cloned(), Else) {
        expect_token!(
            context,
            [Else, Else, ()] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Else],
                tokens.last().unwrap().tt
            )
        );
        Some(parse_try!(
            parse_expr,
            tokens,
            settings,
            context,
            parsed_tokens
        ))
    } else {
        None
    });
    dbg!(&else_expr);
    let lpt = parsed_tokens.last().unwrap();
    dbg!(&lpt);

    let sc_need = !token_parteq!(no_opt lpt, CloseBrace);
    dbg!(&sc_need);

    let end = lpt.span.end;

    Good(
        Expression {
            expr: Expr::ConditionalExpr {
                cond,
                then_expr,
                else_expr,
                sc_need,
            },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_return(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat Return keyword
    let mut parsed_tokens: Vec<Token> = vec![tokens.last().unwrap().clone()];
    tokens.pop();

    let start = parsed_tokens.last().unwrap().span.start;

    let expr = if token_parteq!(no_opt tokens.last().unwrap(), SemiColon) {
        None
    } else {
        Some(Box::new(parse_try!(
            parse_expr,
            tokens,
            settings,
            context,
            parsed_tokens
        )))
    };

    let end = parsed_tokens.last().unwrap().span.end;
    Good(
        Expression {
            expr: ReturnExpr(expr),
            span: start..end,
        },
        parsed_tokens,
    )
}

/// Pre-Unary-Operator enum.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum UnaryOperator {
    AddrOf,
    Not,
}

pub fn parse_unary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];

    let op = expect_token!(
        context,
        [Operator(op), Operator(op.clone()), op] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Operator(Operator::Add)],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

    let unary_op = match op {
        Operator::Not => UnaryOperator::Not,
        Operator::AddrOf => UnaryOperator::AddrOf,
        op => {
            return Bad(ZomError::new(
                Position::try_from_range(
                    context.pos,
                    parsed_tokens.last().unwrap().span.clone(),
                    context.source_file.clone(),
                    context.filename.clone().into(),
                ),
                format!("not a post unary operator, {}", op),
                false,
                None,
                vec![],
            ))
        }
    };

    let expr = Box::new(parse_try!(
        parse_expr,
        tokens,
        settings,
        context,
        parsed_tokens
    ));

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Expression {
            expr: LeftUnaryExpr { unary_op, expr },
            span: start..end,
        },
        parsed_tokens,
    )
}
