//! Module responsible for parsing types.
use crate::prelude::*;
use PrimitiveTy::*;

#[derive(Debug)]
pub struct Type {
    pub ty: Ty,
    pub span: Range<usize>,
}

impl Parse for Type {
    type Output = Self;

    /// Parsing for Types
    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();

        let ty = parse_try!(parser => Ty, parsed_tokens);
        let start = span_toks!(start first parsed_tokens);
        let end = span_toks!(end parsed_tokens);

        Good(
            Type {
                ty,
                span: start..end,
            },
            parsed_tokens,
        )
    }
}

#[derive(Debug)]
pub enum Ty {
    PrimTy(PrimitiveTy),
    PointerTy {
        is_const: bool,
        pointed_ty: Box<Type>,
    },
}

impl Parse for Ty {
    type Output = Self;

    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        match &parser.last().tt {
            T::Ident(name) if PRIM_TYPES.contains(&name.as_str()) => PrimitiveTy::parse(parser),
            T::Oper(Operator::Asterisk) => parse_pointer_ty(parser),
            _ => Error(Box::new(ExpectedToken::from(parser.last(), PartAST::Type))),
        }
    }
}

pub const VOID_TYPE: &str = "void";
pub const BOOL_TYPE: &str = "bool";

pub const U8_TYPE: &str = "u8";
pub const U16_TYPE: &str = "u16";
pub const U32_TYPE: &str = "u32";
pub const U64_TYPE: &str = "u64";
pub const U128_TYPE: &str = "u128";
pub const USIZE_TYPE: &str = "usize";

pub const I8_TYPE: &str = "i8";
pub const I16_TYPE: &str = "i16";
pub const I32_TYPE: &str = "i32";
pub const I64_TYPE: &str = "i64";
pub const I128_TYPE: &str = "i128";
pub const ISIZE_TYPE: &str = "isize";

pub const F16_TYPE: &str = "f16";
pub const F32_TYPE: &str = "f32";
pub const F64_TYPE: &str = "f64";
pub const F128_TYPE: &str = "f128";

pub const PRIM_TYPES: &[&str] = &[
    VOID_TYPE, BOOL_TYPE, U8_TYPE, U16_TYPE, U32_TYPE, U64_TYPE, U128_TYPE, USIZE_TYPE, I8_TYPE,
    I16_TYPE, I32_TYPE, I64_TYPE, I128_TYPE, ISIZE_TYPE, F16_TYPE, F32_TYPE, F64_TYPE, F128_TYPE,
];

#[derive(Debug)]
pub enum PrimitiveTy {
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
}

impl Parse for PrimitiveTy {
    type Output = Ty;

    /// Parsing for primitive types
    fn parse(parser: &mut Parser) -> ParsingResult<Self::Output> {
        let mut parsed_tokens = Vec::new();
        let name = expect_token!(parser => [T::Ident(name), name.clone()], Ident, parsed_tokens);

        let prim_ty = match name.as_str() {
            VOID_TYPE => Void,
            BOOL_TYPE => Bool,

            U8_TYPE => U8,
            U16_TYPE => U16,
            U32_TYPE => U32,
            U64_TYPE => U64,
            U128_TYPE => U128,
            USIZE_TYPE => USize,

            I8_TYPE => I8,
            I16_TYPE => I16,
            I32_TYPE => I32,
            I64_TYPE => I64,
            I128_TYPE => I128,
            ISIZE_TYPE => ISize,

            F16_TYPE => F16,
            F32_TYPE => F32,
            F64_TYPE => F64,
            F128_TYPE => F128,

            _ => unreachable!(),
        };

        Good(Ty::PrimTy(prim_ty), parsed_tokens)
    }
}

/// Parsing for `* [ "const" ] TYPE` type
pub fn parse_pointer_ty(parser: &mut Parser) -> ParsingResult<Ty> {
    let mut parsed_tokens = Vec::new();

    expect_token!(parser => [T::Oper(Operator::Asterisk), ()], T::Oper(Operator::Asterisk), parsed_tokens);

    let is_const = expect_token!(parser => [T::Const, true] else { false }, parsed_tokens);

    let pointed_ty = Box::new(parse_try!(parser => Type, parsed_tokens));

    Good(
        Ty::PointerTy {
            is_const,
            pointed_ty,
        },
        parsed_tokens,
    )
}
