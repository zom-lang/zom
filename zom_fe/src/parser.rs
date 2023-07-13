//! This is the parser of Zom
//!
//! It is entirely made for Zom, without using dependencies.

use std::collections::HashMap;

use zom_common::error::parser::UnexpectedTokenError;
use zom_common::error::ZomError;
use zom_common::token::Token;
use zom_common::token::*;

use crate::FromContext;

pub use self::ASTNode::FunctionNode;

pub use crate::parser::expr::Expression::{
    BinaryExpr, BlockExpr, CallExpr, LiteralExpr, VariableExpr,
};

use self::function::{parse_extern, parse_function, Function};
use self::PartParsingResult::{Bad, Good, NotComplete};

pub mod expr;
pub mod function;
pub mod statement;

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode {
    FunctionNode(Function),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Simple(String),
}

pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), Box<dyn ZomError>>;
pub type ParsingResult2 = Result<Vec<ASTNode>, Vec<Box<dyn ZomError>>>;

#[derive(Debug)]
enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(Box<dyn ZomError>),
}

fn error<T>(err: Box<dyn ZomError>) -> PartParsingResult<T> {
    Bad(err)
}

#[derive(Debug)]
pub struct ParsingContext {
    pub pos: usize,
    pub filename: String,
    pub source_file: String,
    pub full_tokens: Vec<Token>,
}

impl ParsingContext {
    pub fn new(filename: String, source_file: String, full_tokens: Vec<Token>) -> ParsingContext {
        ParsingContext {
            pos: 0,
            filename,
            source_file,
            full_tokens,
        }
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

        operator_precedence.insert(OP_MUL.to_owned(), PRECEDE_MUL_DIV_REM);
        operator_precedence.insert(OP_DIV.to_owned(), PRECEDE_MUL_DIV_REM);
        operator_precedence.insert(OP_REM.to_owned(), PRECEDE_MUL_DIV_REM);

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
            _ => Bad(Box::new(UnexpectedTokenError::from_context(
                context,
                "Expected a function definition or a declaration of an external function."
                    .to_owned(),
                cur_token.clone(),
            ))),
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

#[macro_export]
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
            Bad(error) => return Bad(error)
        }
    )
);

#[macro_export]
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
            _ => { $context.advance(); return $error }
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

fn parse_type(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Type> {
    match tokens.last() {
        Some(&Ident(_)) => parse_type_simple(tokens, settings, context),
        None => NotComplete,
        tok => error(Box::new(UnexpectedTokenError::from_context(
            context,
            format!("unknow token when expecting a type, found {:?}", tok),
            tokens.last().unwrap().clone(),
        ))),
    }
}

fn parse_type_simple(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Type> {
    let mut parsed_tokens = Vec::new();

    let name: String = expect_token!(
        context,
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        error(Box::new(UnexpectedTokenError::from_context(
            context,
            "Type name expected".to_owned(),
            tokens.last().unwrap().clone()
        )))
    );

    Good(Type::Simple(name), parsed_tokens)
}
