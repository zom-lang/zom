//! This module is responsible for the parsing of types.

use std::ops::RangeInclusive;

use zom_common::token::Token;

use crate::{err_et, expect_token, impl_span};

use super::{ParserSettings, ParsingContext, PartParsingResult};

use self::PartParsingResult::{Bad, Good, NotComplete};

use zom_common::token::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Type {
    pub type_variant: TypeVariant,
    pub span: RangeInclusive<usize>,
}

impl_span!(Type);

#[derive(PartialEq, Clone, Debug)]
pub enum TypeVariant {
    PrimitiveType(PrimitiveType),
}

/// A macro to facilitate the match in the `parse_primitive_type` function.
macro_rules! match_primitype {
    ( $name:expr, $ptoken:expr, $([$typename:pat => $primitive_type:expr]),* ) => (
        match $name {
            $(
                $typename => Good(
                    Type {
                        type_variant: TypeVariant::PrimitiveType($primitive_type),
                        span: $ptoken.last().unwrap().span.clone()
                    },
                    $ptoken
                ),
            )*
            _ => panic!("NEED TO REMAKE THE ERROR SYSTEM #4")
        }
    );
}

// For now, PrimitiveType doesn't store the span because no error requires it to.
#[derive(PartialEq, Clone, Debug)]
pub enum PrimitiveType {
    Void,
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

pub const VOID_TYPE_NAME: &str = "void";
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

pub fn parse_type(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Type> {
    match tokens.last() {
        Some(Token { tt: Ident(_), .. }) => parse_primitive_type(tokens, settings, context),
        None => NotComplete,
        _ => err_et!(
            context,
            tokens.last().unwrap(),
            vec![Ident(String::new())],
            tokens.last().unwrap().tt
        ),
    }
}

fn parse_primitive_type(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
    context: &mut ParsingContext,
) -> PartParsingResult<Type> {
    let mut parsed_tokens = Vec::new();
    let t = tokens.last().unwrap().clone();

    let name: String = expect_token!(
        context,
        [Ident(name), Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        err_et!(context, t, vec![OpenParen], t.tt)
    );

    use PrimitiveType::*;

    match_primitype!(
        name.as_str(),
        parsed_tokens,
        [BOOL_TYPE_NAME => Bool],
        [VOID_TYPE_NAME => Void],

        [U8_TYPE_NAME => U8],
        [U16_TYPE_NAME => U16],
        [U32_TYPE_NAME => U32],
        [U64_TYPE_NAME => U64],
        [U128_TYPE_NAME => U128],
        [USIZE_TYPE_NAME => USize],

        [I8_TYPE_NAME => I8],
        [I16_TYPE_NAME => I16],
        [I32_TYPE_NAME => I32],
        [I64_TYPE_NAME => I64],
        [I128_TYPE_NAME => I128],
        [ISIZE_TYPE_NAME => ISize],

        [F16_TYPE_NAME => F16],
        [F32_TYPE_NAME => F32],
        [F64_TYPE_NAME => F64],
        [F128_TYPE_NAME => F128],

        [CHAR_TYPE_NAME => Char],
        [STR_TYPE_NAME => Str]
    )
}
