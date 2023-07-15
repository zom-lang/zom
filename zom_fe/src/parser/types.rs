//! This module is responsible for the parsing of types.

use zom_common::{
    error::parser::UnexpectedTokenError,
    token::Token,
};

use crate::{expect_token, parser::error, FromContext};

use super::{
    expr::Expression,
    ParserSettings, ParsingContext, PartParsingResult,
};

pub use self::Expression::{BinaryExpr, BlockExpr, CallExpr, LiteralExpr, VariableExpr};

use self::PartParsingResult::{Good, NotComplete};

use zom_common::token::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    PrimitiveType(PrimitiveType),
}


#[derive(PartialEq, Clone, Debug)]
pub enum PrimitiveType {
    Bool,

    // Int unsigned
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,

    // Int signed
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,

    // Float
    F16,
    F32,
    F64,
    F128,

    // Char
    Char,

    // String slice
    Str, 
}

pub const BOOL_TYPE_NAME: &str = "bool";

pub const U8_TYPE_NAME: &str = "u8";
pub const U16_TYPE_NAME: &str = "u16";
pub const U32_TYPE_NAME: &str = "u32";
pub const U64_TYPE_NAME: &str = "u64";
pub const U128_TYPE_NAME: &str = "u128";
pub const USIZE_TYPE_NAME: &str = "usize";

pub const I8_TYPE_NAME: &str = "i8";
pub const I16_TYPE_NAME: &str = "i16";
pub const I32_TYPE_NAME: &str = "i32";
pub const I64_TYPE_NAME: &str = "i64";
pub const I128_TYPE_NAME: &str = "i128";
pub const ISIZE_TYPE_NAME: &str = "isize";

pub const F16_TYPE_NAME: &str = "f16";
pub const F32_TYPE_NAME: &str = "f32";
pub const F64_TYPE_NAME: &str = "f64";
pub const F128_TYPE_NAME: &str = "f128";

pub const CHAR_TYPE_NAME: &str = "char";
pub const STR_TYPE_NAME: &str = "str";

pub(super) fn parse_type(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Type> {
    match tokens.last() {
        Some(Token { tt: Ident(_), .. }) => parse_primitive_type(tokens, settings, context),
        None => NotComplete,
        tok => error(Box::new(UnexpectedTokenError::from_context(
            context,
            format!("unknow token when expecting a type, found {:?}", tok),
            tokens.last().unwrap().clone(),
        ))),
    }
}

fn parse_primitive_type(
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

    match name.as_str() {
        BOOL_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::Bool), parsed_tokens),

        U8_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::U8), parsed_tokens),
        U16_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::U16), parsed_tokens),
        U32_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::U32), parsed_tokens),
        U64_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::U64), parsed_tokens),
        U128_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::U128), parsed_tokens),
        USIZE_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::USize), parsed_tokens),

        I8_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::I8), parsed_tokens),
        I16_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::I16), parsed_tokens),
        I32_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::I32), parsed_tokens),
        I64_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::I64), parsed_tokens),
        I128_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::I128), parsed_tokens),
        ISIZE_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::ISize), parsed_tokens),

        F16_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::F16), parsed_tokens),
        F32_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::F32), parsed_tokens),
        F64_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::F64), parsed_tokens),
        F128_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::F128), parsed_tokens),

        CHAR_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::Char), parsed_tokens),
        STR_TYPE_NAME => Good(Type::PrimitiveType(PrimitiveType::Str), parsed_tokens),
        _ => panic!("NEED TO REMAKE THE ERROR SYSTEM")
    }
}