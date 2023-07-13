use zom_common::{
    error::parser::UnexpectedTokenError,
    token::Token::{self, Extern, Func},
};

use crate::{expect_token, parse_try, parser::error, FromContext};

use super::{
    expr::{parse_block_expr, Expression},
    ASTNode, ParserSettings, ParsingContext, PartParsingResult, Type,
};

pub use self::Expression::{BinaryExpr, BlockExpr, CallExpr, LiteralExpr, VariableExpr};

use self::PartParsingResult::{Bad, Good, NotComplete};

use zom_common::token::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Option<Expression>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Arg {
    pub name: String,
    pub type_: Type,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
}

pub(super) fn parse_extern(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<ASTNode> {
    // eat Extern token
    tokens.pop();
    let mut parsed_tokens = vec![Extern];
    let prototype = parse_try!(parse_prototype, tokens, settings, context, parsed_tokens);
    Good(
        ASTNode::FunctionNode(Function {
            prototype,
            body: None,
        }),
        parsed_tokens,
    )
}

pub(super) fn parse_function(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<ASTNode> {
    // eat Def token
    tokens.pop();
    let mut parsed_tokens = vec![Func];
    let prototype = parse_try!(parse_prototype, tokens, settings, context, parsed_tokens);
    let body = parse_try!(parse_block_expr, tokens, settings, context, parsed_tokens);

    Good(
        ASTNode::FunctionNode(Function {
            prototype,
            body: Some(body),
        }),
        parsed_tokens,
    )
}

pub(super) fn parse_prototype(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Prototype> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        context,
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        error(Box::new(UnexpectedTokenError::from_context(
            context,
            "Expected function name in prototype".to_owned(),
            tokens.last().unwrap().clone()
        )))
    );

    expect_token!(
        context,
        [OpenParen, OpenParen, ()] <= tokens,
        parsed_tokens,
        error(Box::new(UnexpectedTokenError::from_context(
            context,
            "Expected '(' in prototype".to_owned(),
            tokens.last().unwrap().clone()
        )))
    );

    let mut args = Vec::new();
    loop {
        expect_token!(
            context, [
            Ident(arg), Ident(arg.clone()), args.push(arg.clone());
            CloseParen, CloseParen, break
        ] <= tokens,
             parsed_tokens,
            error(
                Box::new(UnexpectedTokenError::from_context(
                    context,
                    "Expected an identifier in prototype"
                        .to_owned(),
                    tokens.last().unwrap().clone()
                ))
            )
        );

        expect_token!(
            context, [
            Comma, Comma, {};
            CloseParen, CloseParen, break
        ] <= tokens,
             parsed_tokens,
            error(
                Box::new(UnexpectedTokenError::from_context(
                    context,
                    "Expected ',' in prototype"
                        .to_owned(),
                    tokens.last().unwrap().clone()
                ))
            )
        );
    }

    Good(Prototype { name, args }, parsed_tokens)
}
