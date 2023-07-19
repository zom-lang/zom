//! This module parse function

use zom_common::{error::parser::UnexpectedTokenError, token::Token};

use crate::{
    expect_token, parse_try,
    parser::{error, types::parse_type},
    FromContext,
};

use super::{
    block::parse_block_expr, expr::Expression, types::Type, ASTNode, ParserSettings,
    ParsingContext, PartParsingResult,
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
    pub type_arg: Type,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<Arg>,
}

pub(super) fn parse_extern(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<ASTNode> {
    // eat Extern token
    let mut parsed_tokens = vec![tokens.last().unwrap().clone()];
    tokens.pop();
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
    let mut parsed_tokens: Vec<Token> = vec![tokens.last().unwrap().clone()];
    tokens.pop();
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
    settings: &mut ParserSettings,
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
        let name_arg;
        expect_token!(
            context, [
            Ident(arg), Ident(arg.clone()), name_arg = arg;
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
            Colon, Colon, {};
            CloseParen, CloseParen, break
        ] <= tokens,
             parsed_tokens,
            error(
                Box::new(UnexpectedTokenError::from_context(
                    context,
                    "Expected ':' in argument of a prototype"
                        .to_owned(),
                    tokens.last().unwrap().clone()
                ))
            )
        );

        let type_arg = parse_try!(parse_type, tokens, settings, context, parsed_tokens);

        args.push(Arg {
            name: name_arg,
            type_arg,
        });

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
