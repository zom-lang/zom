//! This is the parser of Zom
//!
//! It is entirely made for Zom, without using dependencies.

use std::collections::HashMap;

use zom_common::error::parser::UnexpectedTokenError;
use zom_common::token::Token;
use zom_common::token::*;

pub use self::ASTNode::FunctionNode;

pub use self::Expression::{BinaryExpr, CallExpr, LiteralExpr, VariableExpr};

use self::PartParsingResult::{Bad, Good, NotComplete};

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode {
    FunctionNode(Function),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Option<Expression>,
    pub is_anonymous: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    LiteralExpr(i32),
    VariableExpr(String),
    BinaryExpr(String, Box<Expression>, Box<Expression>),
    CallExpr(String, Vec<Expression>),
}

pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), String>;

enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(String),
}

fn error<T>(message: &str) -> PartParsingResult<T> {
    Bad(message.to_string())
}

#[derive(Debug)]
pub struct ParsingContext {
    pos: usize,
    filename: String,
    source_file: String,
}

impl ParsingContext {
    pub fn new(filename: String, source_file: String) -> ParsingContext {
        ParsingContext { pos: 0, filename, source_file }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn source_file(&self) -> String {
        self.source_file.clone()
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }
}

#[derive(Debug)]
pub struct ParserSettings {
    operator_precedence: HashMap<String, i32>,
}

impl Default for ParserSettings {
    fn default() -> Self {
        let mut operator_precedence = HashMap::with_capacity(14);

        // Setup Operator Precedence according to the documentation

        operator_precedence.insert(OP_MUL.to_owned(), PRECEDE_MUL_DIV_MOD);
        operator_precedence.insert(OP_DIV.to_owned(), PRECEDE_MUL_DIV_MOD);
        operator_precedence.insert(OP_MOD.to_owned(), PRECEDE_MUL_DIV_MOD);

        operator_precedence.insert(OP_PLUS.to_owned(), PRECEDE_ADD_SUB);
        operator_precedence.insert(OP_MINUS.to_owned(), PRECEDE_ADD_SUB);

        operator_precedence.insert(OP_COMP_LT.to_owned(), PRECEDE_COMP);
        operator_precedence.insert(OP_COMP_GT.to_owned(), PRECEDE_COMP);
        operator_precedence.insert(OP_COMP_LTE.to_owned(), PRECEDE_COMP);
        operator_precedence.insert(OP_COMP_GTE.to_owned(), PRECEDE_COMP);

        operator_precedence.insert(OP_COMP_EQ.to_owned(), PRECEDE_EQ_NE);
        operator_precedence.insert(OP_COMP_NE.to_owned(), PRECEDE_EQ_NE);

        operator_precedence.insert(OP_AND.to_owned(), PRECEDE_AND);

        operator_precedence.insert(OP_OR.to_owned(), PRECEDE_OR);

        operator_precedence.insert(OP_EQ.to_owned(), PRECEDE_EQ);

        ParserSettings {
            operator_precedence,
        }
    }
}

pub fn parse(
    tokens: &[Token],
    parsed_tree: &[ASTNode],
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> ParsingResult {
    let mut rest = tokens.to_vec();
    // we read tokens from the end of the vector
    // using it as a stack
    rest.reverse();

    // we will add new AST nodes to already parsed ones
    let mut ast = parsed_tree.to_vec();

    while let Some(cur_token) = rest.last() {
        let result = match cur_token {
            Func => parse_function(&mut rest, settings, context),
            Extern => parse_extern(&mut rest, settings, context),
            SemiColon => {
                rest.pop();
                continue;
            }
            _ => Bad(
                // "Expected a function definition or a declaration of an external function."
                //     .to_owned(),
                format!("{}", UnexpectedTokenError::from_pos(context.pos, tokens.to_vec(), &mut context.source_file, &mut context.filename, "Expected a function definition or a declaration of an external function.".to_owned())),
            ),
        };
        match result {
            Good(ast_node, _) => ast.push(ast_node),
            NotComplete => break,
            Bad(message) => return Err(message),
        }
    }

    // unparsed tokens
    rest.reverse();
    Ok((ast, rest))
}

macro_rules! parse_try(
    ($function:ident, $tokens:ident, $settings:ident, $context:ident, $parsed_tokens:ident) => (
        parse_try!($function, $tokens, $settings, $context, $parsed_tokens,)
    );

    ($function:ident, $tokens:ident, $settings:ident, $context:ident, $parsed_tokens:ident, $($arg:expr),*) => (
        match $function($tokens, $settings, $context, $($arg),*) {
            Good(ast, toks) => {
                $parsed_tokens.extend(toks.into_iter());
                ast
            },
            NotComplete => {
                $parsed_tokens.reverse();
                $tokens.extend($parsed_tokens.into_iter());
                return NotComplete;
            },
            Bad(message) => return Bad(message)
        }
    )
);

macro_rules! expect_token (
    ($context:ident, [ $($token:pat, $value:expr, $result:stmt);+ ] <= $tokens:ident, $parsed_tokens:ident, $error:expr) => (
        match $tokens.pop() {
            $(
                Some($token) => {
                    $context.advance();
                    $parsed_tokens.push($value);
                    $result
                },
             )+
             None => {
                $context.advance();
                $parsed_tokens.reverse();
                $tokens.extend($parsed_tokens.into_iter());
                return NotComplete;
             },
            _ => { $context.advance(); return error($error) }
        }
    );

    ($context:ident, [ $($token:pat, $value:expr, $result:stmt);+ ] else $not_matched:block <= $tokens:ident, $parsed_tokens:ident) => (
        $context.advance();
        match $tokens.last().map(|i| {i.clone()}) {
            $(
                Some($token) => {
                    $tokens.pop();
                    $parsed_tokens.push($value);
                    $result
                },
             )+
            _ => {$not_matched}
        }
    )
);

fn parse_extern(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<ASTNode> {
    // eat Extern token
    tokens.pop();
    let mut parsed_tokens = vec![Extern];
    let prototype = parse_try!(parse_prototype, tokens, settings, context, parsed_tokens);
    Good(
        FunctionNode(Function {
            prototype,
            body: None,
            is_anonymous: false,
        }),
        parsed_tokens,
    )
}

fn parse_function(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<ASTNode> {
    // eat Def token
    tokens.pop();
    let mut parsed_tokens = vec![Func];
    let prototype = parse_try!(parse_prototype, tokens, settings, context, parsed_tokens);
    let body = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);

    Good(
        FunctionNode(Function {
            prototype,
            body: Some(body),
            is_anonymous: false,
        }),
        parsed_tokens,
    )
}

fn parse_prototype(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Prototype> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        context,
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        format!(
            "expected function name in prototype, tok pos = {}",
            context.pos
        )
        .as_str()
    );

    expect_token!(
        context,
        [OpenParen, OpenParen, ()] <= tokens,
        parsed_tokens,
        "expected '(' in prototype"
    );

    let mut args = Vec::new();
    loop {
        expect_token!(
            context, [
            Ident(arg), Ident(arg.clone()), args.push(arg.clone());
            Comma, Comma, continue;
            CloseParen, CloseParen, break
        ] <= tokens, parsed_tokens, "expected ')' in prototype");
    }

    Good(Prototype { name, args }, parsed_tokens)
}

fn parse_primary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    match tokens.last() {
        Some(&Ident(_)) => parse_ident_expr(tokens, settings, context),
        Some(&Int(_)) => parse_literal_expr(tokens, settings, context),
        Some(&OpenParen) => parse_parenthesis_expr(tokens, settings, context),
        None => NotComplete,
        _ => error("unknow token when expecting an expression"),
    }
}

fn parse_ident_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        context,
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        "identificator expected"
    );

    expect_token!(
        context,
        [OpenParen, OpenParen, ()]
        else {return Good(VariableExpr(name), parsed_tokens)}
        <= tokens, parsed_tokens);

    let mut args = Vec::new();
    loop {
        expect_token!(
            context,
            [CloseParen, CloseParen, break;
             Comma, Comma, continue]
            else {
                args.push(parse_try!(parse_expr, tokens, settings, context, parsed_tokens));
            }
            <= tokens, parsed_tokens);
    }

    Good(CallExpr(name, args), parsed_tokens)
}

fn parse_literal_expr(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let value = expect_token!(
        context,
        [Int(val), Int(val), val] <= tokens,
        parsed_tokens,
        "literal expected"
    );

    Good(LiteralExpr(value), parsed_tokens)
}

fn parse_parenthesis_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    // eat the opening parenthesis
    tokens.pop();
    let mut parsed_tokens = vec![OpenParen];

    let expr = parse_try!(parse_expr, tokens, settings, context, parsed_tokens);

    expect_token!(
        context,
        [CloseParen, CloseParen, ()] <= tokens,
        parsed_tokens,
        "')' expected"
    );

    Good(expr, parsed_tokens)
}

fn parse_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();
    let lhs = parse_try!(parse_primary_expr, tokens, settings, context, parsed_tokens);
    let expr = parse_try!(parse_binary_expr, tokens, settings, context, parsed_tokens, 0, &lhs);
    Good(expr, parsed_tokens)
}

fn parse_binary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
    expr_precedence: i32,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    // start with LHS value
    let mut result = lhs.clone();
    let mut parsed_tokens = Vec::new();

    loop {
        // continue until the current token is not an operator
        // or it is an operator with precedence lesser than expr_precedence
        let (operator, precedence) = match tokens.last() {
            Some(Operator(op)) => match settings.operator_precedence.get(op) {
                Some(pr) if *pr >= expr_precedence => (op.clone(), *pr),
                None => return error("unknown operator found"),
                _ => break,
            },
            _ => break,
        };
        tokens.pop();
        parsed_tokens.push(Operator(operator.clone()));

        // parse primary RHS expression
        let mut rhs = parse_try!(parse_primary_expr, tokens, settings, context, parsed_tokens);

        // parse all the RHS operators until their precedence is
        // bigger than the current one
        loop {
            let binary_rhs = match tokens.last().cloned() {
                Some(Operator(ref op)) => match settings.operator_precedence.get(op).copied() {
                    Some(pr) if pr > precedence => {
                        parse_try!(parse_binary_expr, tokens, settings, context, parsed_tokens, pr, &rhs)
                    }
                    None => return error("unknown operator found"),
                    _ => break,
                },
                _ => break,
            };

            rhs = binary_rhs;
        }

        // merge LHS and RHS
        result = BinaryExpr(operator, Box::new(result), Box::new(rhs));
    }

    Good(result, parsed_tokens)
}
