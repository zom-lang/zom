//! This module contains the parsing for expressions.

use crate::prelude::*;

use self::Expr::*;

use crate::block::{parse_block, parse_block_expr, Block};

#[derive(PartialEq, Clone, Debug)]
pub struct Expression {
    pub expr: Expr,
    pub span: Range<usize>,
}

impl_span!(Expression);

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    IntLiteralExpr(u64),
    CharLiteralExpr(char),
    StrLiteralExpr(String),
    SymbolExpr(String),
    BinaryExpr {
        op: BinOperation,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    CallExpr {
        fn_operand: Box<Expression>,
        args: Vec<Expression>,
    },
    BlockExpr(Block),
    BooleanExpr(bool),
    UndefinedExpr,
    ConditionalExpr {
        cond: Box<Expression>,
        branch_true: Box<Expression>,
        branch_false: Box<Option<Expression>>,
        /// Semi colon needed ?
        sc_need: bool,
    },
    ReturnExpr(Option<Box<Expression>>),
    UnaryExpr {
        unary_op: UnaryOperation,
        expr: Box<Expression>,
    },
    WhileExpr {
        label: Option<String>,
        controlling_expr: Box<Expression>,
        loop_body: Box<Block>,
    },
    BreakExpr {
        label: Option<String>,
        value: Option<Box<Expression>>,
    },
    ContinueExpr {
        label: Option<String>,
        value: Option<Box<Expression>>,
    },
    MemberAccessExpr {
        expr: Box<Expression>,
        qualified_name: String,
    },
}

impl Expression {
    /// Is the semicolon mandatory when parsing the expression statement
    pub fn semicolon(&self) -> bool {
        match *self {
            Expression {
                expr: BlockExpr(_) | WhileExpr { .. },
                ..
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
        Some(Token { tt: Ident(_), .. }) if is_labeled_expr(tokens) => {
            parse_labeled_expr(tokens, settings, context)
        }
        Some(Token { tt: Ident(_), .. }) => parse_symbol_expr(tokens, settings, context),
        Some(Token { tt: Int(_), .. }) => parse_int_literal_expr(tokens, settings, context),
        Some(Token { tt: OpenParen, .. }) => parse_parenthesis_expr(tokens, settings, context),
        Some(Token { tt: OpenBrace, .. }) => parse_block_expr(tokens, settings, context),
        Some(Token {
            tt: True | False, ..
        }) => parse_boolean_expr(tokens, settings, context),
        Some(Token { tt: Undefined, .. }) => parse_undefined_expr(tokens, settings, context),
        Some(Token { tt: If, .. }) => parse_conditional_expr(tokens, settings, context),
        Some(Token { tt: Return, .. }) => parse_return_expr(tokens, settings, context),
        Some(Token { tt: Oper(_), .. }) => parse_left_unary_expr(tokens, settings, context),
        Some(Token { tt: While, .. }) => parse_while_expr(tokens, settings, context, None),
        Some(Token { tt: Break, .. }) => parse_break_expr(tokens, settings, context),
        Some(Token { tt: Continue, .. }) => parse_continue_expr(tokens, settings, context),
        Some(Token { tt: Char(..), .. }) => parse_char_literal_expr(tokens, settings, context),
        Some(Token { tt: Str(..), .. }) => parse_str_literal_expr(tokens, settings, context),
        None => NotComplete,
        _ => err_et!(
            context,
            tokens.last().unwrap(),
            vec![Ident(String::new()), Int(0), OpenParen, OpenBrace],
            tokens.last().unwrap().tt
        ),
    }
}

pub fn parse_symbol_expr(
    tokens: &mut Vec<Token>,
    _: &mut ParserSettings,
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

    let span = parsed_tokens.last().unwrap().clone().span.clone();

    Good(
        Expression {
            expr: SymbolExpr(name),
            span,
        },
        parsed_tokens,
    )
}

pub fn parse_int_literal_expr(
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

    let span = parsed_tokens.last().unwrap().span.clone();

    Good(
        Expression {
            expr: IntLiteralExpr(value),
            span,
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
    let mut parsed_tokens: Vec<Token> = vec![];
    let t = tokens.last().unwrap().clone();
    expect_token!(
        context,
        [OpenParen, OpenParen, ()] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![OpenParen], t.tt)
    );

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

    let mut result = match tokens.last() {
        Some(Token { tt: Oper(op), .. }) if BinOperation::try_from(op.clone()).is_ok() => {
            parse_try!(
                parse_binary_expr,
                tokens,
                settings,
                context,
                parsed_tokens,
                0,
                &lhs
            )
        }
        Some(Token { tt: Oper(op), .. }) if is_right_unary_op(op.clone()) => parse_try!(
            parse_right_unary_expr,
            tokens,
            settings,
            context,
            parsed_tokens,
            &lhs
        ),
        Some(Token { tt: OpenParen, .. }) => parse_try!(
            parse_call_expr,
            tokens,
            settings,
            context,
            parsed_tokens,
            &lhs
        ),
        Some(Token { tt: Oper(_), .. }) if is_member_access_expr(tokens) => parse_try!(
            parse_member_access_expr,
            tokens,
            settings,
            context,
            parsed_tokens,
            &lhs
        ),
        _ => lhs,
    };

    while !is_expr_end(tokens) && is_right_unary_start(tokens.last().unwrap().tt.clone()) {
        result = parse_try!(
            // try to call parse_expr instead of calling a separate fn
            parse_unary_expr,
            tokens,
            settings,
            context,
            parsed_tokens,
            &result
        );
    }
    Good(result, parsed_tokens)
}

pub fn is_right_unary_start(token: TokenType) -> bool {
    match token {
        Oper(op) if is_right_unary_op(op.clone()) => true,
        OpenParen => true,
        _ => false,
    }
}

/// When calling this function you should assert that the next token is a right unary expr because it panics if it's not.
fn parse_unary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    match tokens.last() {
        Some(Token { tt: Oper(op), .. }) if is_right_unary_op(op.clone()) => {
            parse_right_unary_expr(tokens, settings, context, lhs)
        }
        Some(Token { tt: OpenParen, .. }) => parse_call_expr(tokens, settings, context, lhs),
        Some(Token { tt: Oper(_), .. }) if is_member_access_expr(tokens) => {
            parse_member_access_expr(tokens, settings, context, lhs)
        }
        _ => panic!("doesn't start with a right unary token."),
    }
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
    Equal,
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
            Equal => BOp::Equal,
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
        tt: Oper(op),
        span: _,
    }) = tokens.last()
    {
        let bin_op = match BinOperation::try_from(op.clone()) {
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
            tt: Oper(op),
            span: _,
        }) = tokens.last().cloned()
        {
            if BinOperation::try_from(op.clone()).is_err() {
                continue;
            }
            let bin_op = match BinOperation::try_from(op) {
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

    let cond = Box::new(parse_try!(
        parse_parenthesis_expr,
        tokens,
        settings,
        context,
        parsed_tokens
    ));

    let branch_true = Box::new(parse_try!(
        parse_expr,
        tokens,
        settings,
        context,
        parsed_tokens
    ));

    let branch_false = Box::new(if token_parteq!(tokens.last().cloned(), Else) {
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
    // lpt = last parsed token
    let lpt = parsed_tokens.last().unwrap();

    let sc_need = !token_parteq!(no_opt lpt, CloseBrace);

    let end = lpt.span.end;

    Good(
        Expression {
            expr: Expr::ConditionalExpr {
                cond,
                branch_true,
                branch_false,
                sc_need,
            },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_return_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat Return keyword
    let mut parsed_tokens: Vec<Token> = vec![tokens.last().unwrap().clone()];
    tokens.pop();

    let start = parsed_tokens.last().unwrap().span.start;

    let expr = if !is_expr_end(tokens) {
        Some(Box::new(parse_try!(
            parse_expr,
            tokens,
            settings,
            context,
            parsed_tokens
        )))
    } else {
        None
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

/// Unary-Operations enum.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum UnaryOperation {
    AddrOf,
    Minus,
    Not,
    Deref,
}

impl TryFrom<Operator> for UnaryOperation {
    type Error = ();

    fn try_from(op: Operator) -> Result<Self, Self::Error> {
        use self::UnaryOperation as UOp;
        use zom_common::token::Operator::*;
        Ok(match op {
            Ampersand => UOp::AddrOf,
            Exclamationmark => UOp::Not,
            DotAsterisk => UOp::Deref,
            Minus => UOp::Minus,
            _ => return Err(()),
        })
    }
}

pub fn is_right_unary_op(op: Operator) -> bool {
    matches!(op, zom_common::token::Operator::DotAsterisk)
}

pub fn parse_left_unary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];

    let op = expect_token!(
        context,
        [Oper(op), Oper(op.clone()), op] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![
                Oper(Operator::Ampersand),
                Oper(Operator::Minus),
                Oper(Operator::Exclamationmark)
            ],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

    let unary_op = match UnaryOperation::try_from(op.clone()) {
        Ok(v) => v,
        Err(_) => {
            return Bad(ZomError::new(
                Position::try_from_range(
                    context.pos,
                    parsed_tokens.last().unwrap().span.clone(),
                    context.source_file.clone(),
                    context.filename.clone().into(),
                ),
                format!("not a left unary operator, {}", op),
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
            expr: UnaryExpr { unary_op, expr },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_right_unary_expr(
    tokens: &mut Vec<Token>,
    _: &mut ParserSettings,
    context: &mut ParsingContext,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];
    let expr = Box::new(lhs.clone());

    let unary_op = match UnaryOperation::try_from(expect_token!(
        context,
        [Oper(op), Oper(op.clone()), op] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Oper(Operator::DotAsterisk)],
            tokens.last().unwrap().tt
        )
    )) {
        Ok(v) => v,
        Err(_) => {
            return Bad(ZomError::new(
                Position::try_from_range(
                    context.pos,
                    parsed_tokens.last().unwrap().span.clone(),
                    context.source_file.clone(),
                    context.filename.clone().into(),
                ),
                "not a right unary operator".to_string(),
                false,
                None,
                vec![],
            ))
        }
    };

    let span = parsed_tokens.last().unwrap().span.clone();

    Good(
        Expression {
            expr: UnaryExpr { unary_op, expr },
            span,
        },
        parsed_tokens,
    )
}

pub fn is_labeled_expr(tokens: &[Token]) -> bool {
    matches!(tokens.last(), Some(Token { tt: Ident(_), .. }) if token_parteq!(tokens.get(tokens.len() - 2), Colon))
}

pub fn parse_labeled_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];

    let label = expect_token!(
        context,
        [Ident(label), Ident(label.clone()), label] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Ident(String::new())],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

    expect_token!(
        context,
        [Colon, Colon, ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Colon],
            tokens.last().unwrap().tt
        )
    );

    let expr = match tokens.last() {
        Some(Token { tt: While, .. }) => parse_try!(parse_while_expr, tokens, settings, context, parsed_tokens, Some(label)),
        e => todo!("this expr cannot be  labeled, parse the expr but throw an error with the range of that parsed expr {:?}", e)
    };

    Good(
        Expression {
            expr: expr.expr,
            span: start..expr.span.end,
        },
        parsed_tokens,
    )
}

pub fn parse_while_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
    label: Option<String>,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens: Vec<Token> = vec![];

    expect_token!(
        context,
        [While, While, ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![While],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

    let controlling_expr = Box::new(parse_try!(
        parse_parenthesis_expr,
        tokens,
        settings,
        context,
        parsed_tokens
    ));

    let loop_body = Box::new(parse_try!(
        parse_block,
        tokens,
        settings,
        context,
        parsed_tokens
    ));

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Expression {
            expr: WhileExpr {
                label,
                controlling_expr,
                loop_body,
            },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_break_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];

    expect_token!(
        context,
        [Break, Break, ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Break],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

    let label = if let Some(Token { tt: Colon, .. }) = tokens.last() {
        expect_token!(
            context,
            [Colon, Colon, ()] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Colon],
                tokens.last().unwrap().tt
            )
        );

        expect_token!(
            context,
            [Ident(label), Ident(label.clone()), Some(label)] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Ident(String::new())],
                tokens.last().unwrap().tt
            )
        )
    } else {
        None
    };

    let value = if !is_expr_end(tokens) {
        Some(Box::new(parse_try!(
            parse_expr,
            tokens,
            settings,
            context,
            parsed_tokens
        )))
    } else {
        None
    };

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Expression {
            expr: BreakExpr { label, value },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn is_expr_end(tokens: &[Token]) -> bool {
    matches!(
        tokens.last().unwrap().tt,
        SemiColon | Comma | CloseParen | CloseBracket | CloseBrace | Else
    )
}

pub fn parse_continue_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];

    expect_token!(
        context,
        [Continue, Continue, ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Continue],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

    let label = if let Some(Token { tt: Colon, .. }) = tokens.last() {
        expect_token!(
            context,
            [Colon, Colon, ()] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Colon],
                tokens.last().unwrap().tt
            )
        );

        expect_token!(
            context,
            [Ident(label), Ident(label.clone()), Some(label)] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                tokens.last().unwrap(),
                vec![Ident(String::new())],
                tokens.last().unwrap().tt
            )
        )
    } else {
        None
    };

    let value = if !is_expr_end(tokens) {
        Some(Box::new(parse_try!(
            parse_expr,
            tokens,
            settings,
            context,
            parsed_tokens
        )))
    } else {
        None
    };

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Expression {
            expr: ContinueExpr { label, value },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_call_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];
    let fn_operand = Box::new(lhs.clone());

    expect_token!(
        context,
        [OpenParen, OpenParen, ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![OpenParen],
            tokens.last().unwrap().tt
        )
    );
    let start = parsed_tokens.last().unwrap().span.start;
    let mut args = Vec::new();

    loop {
        expect_token!(
            context,
            [CloseParen, CloseParen, break] else {
                args.push(parse_try!(parse_expr, tokens, settings, context, parsed_tokens));
            } <= tokens, parsed_tokens
        );

        let t = tokens.last().unwrap().clone();
        expect_token!(
            context,
            [Comma, Comma, ();
             CloseParen, CloseParen, break] <= tokens,
            parsed_tokens,
            err_et!(
                context,
                t,
                vec![Comma, CloseParen], t.tt
            )
        )
    }
    let end = parsed_tokens.last().unwrap().span.end;
    Good(
        Expression {
            expr: CallExpr { fn_operand, args },
            span: start..end,
        },
        parsed_tokens,
    )
}

pub fn parse_char_literal_expr(
    tokens: &mut Vec<Token>,
    _: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];

    let t = tokens.last().unwrap().clone();
    let char = expect_token!(
        context,
        [Char(c), Char(c), c] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![Char(' ')], t.tt)
    );

    Good(
        Expression {
            expr: CharLiteralExpr(char),
            span: t.span,
        },
        parsed_tokens,
    )
}

pub fn parse_str_literal_expr(
    tokens: &mut Vec<Token>,
    _: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];

    let t = tokens.last().unwrap().clone();
    let str = expect_token!(
        context,
        [Str(s), Str(s.clone()), s] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![Str(String::new())], t.tt)
    );

    Good(
        Expression {
            expr: StrLiteralExpr(str),
            span: t.span,
        },
        parsed_tokens,
    )
}

pub fn is_member_access_expr(tokens: &[Token]) -> bool {
    matches!(
        tokens.get(tokens.len() - 2),
        Some(Token { tt: Ident(_), .. })
        if token_parteq!(tokens.last(), Oper(Operator::Dot))
    )
}

pub fn parse_member_access_expr(
    tokens: &mut Vec<Token>,
    _: &mut ParserSettings,
    context: &mut ParsingContext,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = vec![];
    let expr = Box::new(lhs.clone());
    expect_token!(
        context,
        [Oper(Operator::Dot), Oper(Operator::Dot), ()] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Oper(Operator::Dot)],
            tokens.last().unwrap().tt
        )
    );

    let start = parsed_tokens.last().unwrap().span.start;

    let qualified_name = expect_token!(
        context,
        [Ident(qn), Ident(qn.clone()), qn] <= tokens,
        parsed_tokens,
        err_et!(
            context,
            tokens.last().unwrap(),
            vec![Ident(String::new())],
            tokens.last().unwrap().tt
        )
    );

    let end = parsed_tokens.last().unwrap().span.end;

    Good(
        Expression {
            expr: MemberAccessExpr {
                expr,
                qualified_name,
            },
            span: start..end,
        },
        parsed_tokens,
    )
}
